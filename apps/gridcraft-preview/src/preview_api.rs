//! JavaScript API for GridCraft Preview.
//!
//! Exports functions via wasm-bindgen that can be called from JavaScript.
//! This provides a high-level API for controlling the preview from JS.

use serde_json::json;
use wasm_bindgen::prelude::*;
use web_sys::{Blob, File};

use crate::web;

/// GridCraft Preview JavaScript API.
///
/// Example usage from JavaScript:
/// ```js
/// import init, { GridCraftPreview } from './gridcraft-preview.js';
///
/// await init();
/// const preview = GridCraftPreview.new('gridcraft_canvas');
/// await preview.load_file(fileBlob, 'workbook.xlsx');
/// preview.zoom_in();
/// ```
#[wasm_bindgen]
pub struct GridCraftPreview {
    // Internal state would go here
    // For now, this is a placeholder that will interact with the running app
}

#[wasm_bindgen]
impl GridCraftPreview {
    /// Create a new preview instance.
    ///
    /// Note: In the current implementation, the preview is initialized
    /// automatically when the WASM module loads. This constructor is
    /// provided for API compatibility.
    #[wasm_bindgen(constructor)]
    pub fn new() -> Result<GridCraftPreview, JsValue> {
        Ok(GridCraftPreview {})
    }

    /// Load a file from a Blob or File object.
    #[wasm_bindgen(js_name = loadFile)]
    pub async fn load_file(&mut self, file: JsValue, filename: Option<String>) -> Result<(), JsValue> {
        let (name, bytes) = if let Some(file_obj) = file.dyn_ref::<File>() {
            let name = filename.unwrap_or_else(|| file_obj.name());
            let bytes = read_file_bytes(file_obj).await?;
            (name, bytes)
        } else if let Some(blob) = file.dyn_ref::<Blob>() {
            let name = filename.unwrap_or_else(|| "workbook.xlsx".to_string());
            let bytes = read_blob_bytes(blob).await?;
            (name, bytes)
        } else {
            return Err(JsValue::from_str("Expected File or Blob object"));
        };

        web::queue_file(name, bytes).map_err(|error| JsValue::from_str(&error))?;

        Ok(())
    }

    /// Zoom in on the current sheet.
    #[wasm_bindgen(js_name = zoomIn)]
    pub fn zoom_in(&mut self) -> Result<(), JsValue> {
        send_command("preview.zoomIn", json!({}))
    }

    /// Zoom out on the current sheet.
    #[wasm_bindgen(js_name = zoomOut)]
    pub fn zoom_out(&mut self) -> Result<(), JsValue> {
        send_command("preview.zoomOut", json!({}))
    }

    /// Zoom to fit the window.
    #[wasm_bindgen(js_name = zoomToFit)]
    pub fn zoom_to_fit(&mut self) -> Result<(), JsValue> {
        send_command("preview.zoomToFit", json!({"viewWidth": 1200, "viewHeight": 700}))
    }

    /// Set zoom to a specific percentage.
    #[wasm_bindgen(js_name = setZoom)]
    pub fn set_zoom(&mut self, percentage: f64) -> Result<(), JsValue> {
        if !percentage.is_finite() {
            return Err(JsValue::from_str("zoom percentage must be finite"));
        }
        send_command("view.zoom", json!({"percent": percentage.clamp(10.0, 400.0)}))
    }

    /// Switch between the default read-only viewer and editable workbook mode.
    #[wasm_bindgen(js_name = setEditable)]
    pub fn set_editable(&mut self, editable: bool) -> Result<(), JsValue> {
        send_command("preview.setEditable", json!({"editable": editable}))
    }

    /// Configure whether the workbook is read-only.
    #[wasm_bindgen(js_name = setReadOnly)]
    pub fn set_read_only(&mut self, read_only: bool) -> Result<(), JsValue> {
        self.set_editable(!read_only)
    }

    /// Get list of sheet names.
    #[wasm_bindgen(js_name = getSheetNames)]
    pub fn get_sheet_names(&self) -> Result<Vec<JsValue>, JsValue> {
        // TODO: Query the session for sheet names
        Ok(vec![])
    }

    /// Get the current sheet name.
    #[wasm_bindgen(js_name = getCurrentSheet)]
    pub fn get_current_sheet(&self) -> Result<String, JsValue> {
        // TODO: Query the session for current sheet
        Ok("Sheet1".to_string())
    }

    /// Switch to a different sheet by name.
    #[wasm_bindgen(js_name = switchSheet)]
    pub fn switch_sheet(&mut self, name: String) -> Result<(), JsValue> {
        send_command("sheet.activate", json!({"name": name}))
    }

    /// Get the current selection range.
    #[wasm_bindgen(js_name = getSelection)]
    pub fn get_selection(&self) -> Result<String, JsValue> {
        // TODO: Query the session for selection
        Ok("A1".to_string())
    }

    /// Select a range.
    #[wasm_bindgen(js_name = selectRange)]
    pub fn select_range(&mut self, range: String) -> Result<(), JsValue> {
        send_command("selection.set", json!({"range": range}))
    }

    /// Find text in the current sheet.
    #[wasm_bindgen(js_name = find)]
    pub fn find(&mut self, query: String, match_case: Option<bool>) -> Result<u32, JsValue> {
        let match_case = match_case.unwrap_or(false);
        send_command("search.find", json!({"query": query, "matchCase": match_case}))?;
        // TODO: Return actual match count
        Ok(0)
    }

    /// Find next occurrence.
    #[wasm_bindgen(js_name = findNext)]
    pub fn find_next(&mut self) -> Result<bool, JsValue> {
        send_command("search.findNext", json!({}))?;
        Ok(true)
    }

    /// Find previous occurrence.
    #[wasm_bindgen(js_name = findPrevious)]
    pub fn find_previous(&mut self) -> Result<bool, JsValue> {
        send_command("search.findPrevious", json!({}))?;
        Ok(true)
    }

    /// Set the interface language.
    ///
    /// Supported languages: en, zh, es, ja, ko, ru, pt, de, cs, fr, sr, tr, pl, it, uk, vi, nl, ro, el
    #[wasm_bindgen(js_name = setLanguage)]
    pub fn set_language(&mut self, lang: String) -> Result<(), JsValue> {
        send_command("app.language.set", json!({"language": lang}))
    }

    /// Set the theme.
    ///
    /// Options: "light", "dark", "auto"
    #[wasm_bindgen(js_name = setTheme)]
    pub fn set_theme(&mut self, theme: String) -> Result<(), JsValue> {
        let mode = match theme.as_str() {
            "auto" => "system",
            "light" | "dark" => theme.as_str(),
            _ => return Err(JsValue::from_str("theme must be light, dark, or auto")),
        };
        send_command("view.theme", json!({"mode": mode}))
    }

    /// Configure whether the legacy `.xls` compatibility notice is shown after opening a file.
    #[wasm_bindgen(js_name = setShowXlsImportNotice)]
    pub fn set_show_xls_import_notice(&mut self, show: bool) -> Result<(), JsValue> {
        send_command("preview.showXlsImportNotice", json!({"show": show}))
    }
}

/// Send a command to the running app.
///
/// This is a placeholder - in a full implementation, this would communicate
/// with the running PreviewShell instance.
fn send_command(command: &str, params: serde_json::Value) -> Result<(), JsValue> {
    web::queue_command(command, params).map_err(|error| JsValue::from_str(&error))
}

/// Read bytes from a File object.
async fn read_file_bytes(file: &File) -> Result<Vec<u8>, JsValue> {
    read_blob_bytes(file.as_ref()).await
}

/// Read bytes from a Blob object using FileReader.
async fn read_blob_bytes(blob: &Blob) -> Result<Vec<u8>, JsValue> {
    use wasm_bindgen::JsCast;
    use wasm_bindgen_futures::JsFuture;

    let file_reader = web_sys::FileReader::new()?;
    let promise = js_sys::Promise::new(&mut |resolve, reject| {
        let reader = file_reader.clone();
        let onload = Closure::wrap(Box::new(move |_event: web_sys::ProgressEvent| {
            if let Ok(result) = reader.result() {
                resolve.call1(&JsValue::NULL, &result).ok();
            }
        }) as Box<dyn FnMut(_)>);

        let onerror = Closure::wrap(Box::new(move |_event: web_sys::ProgressEvent| {
            reject.call1(&JsValue::NULL, &JsValue::from_str("Failed to read file")).ok();
        }) as Box<dyn FnMut(_)>);

        file_reader.set_onload(Some(onload.as_ref().unchecked_ref()));
        file_reader.set_onerror(Some(onerror.as_ref().unchecked_ref()));

        onload.forget();
        onerror.forget();
    });

    file_reader.read_as_array_buffer(blob)?;
    let result = JsFuture::from(promise).await?;
    let array = js_sys::Uint8Array::new(&result);

    Ok(array.to_vec())
}
