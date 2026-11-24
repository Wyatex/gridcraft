//! JavaScript API for GridCraft Preview.
//!
//! Exports functions via `wasm-bindgen` for controlling the read-only spreadsheet preview from JS.

use serde_json::json;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Blob, File};

use crate::web;

const READ_ONLY_ERROR: &str = "GridCraft Preview is read-only; editing is not supported in this build";

/// GridCraft Preview JavaScript API.
#[wasm_bindgen]
pub struct GridCraftPreview {
    last_find: Option<(String, bool)>,
}

#[wasm_bindgen]
impl GridCraftPreview {
    /// Create a new preview handle connected to the running canvas instance.
    #[wasm_bindgen(constructor)]
    pub fn new() -> Result<GridCraftPreview, JsValue> {
        Ok(GridCraftPreview { last_find: None })
    }

    /// Load a spreadsheet from a `Blob` or `File` object.
    #[wasm_bindgen(js_name = loadFile)]
    pub async fn load_file(&mut self, file: JsValue, filename: Option<String>) -> Result<(), JsValue> {
        let (name, bytes) = if let Some(file_obj) = file.dyn_ref::<File>() {
            let name = filename.unwrap_or_else(|| file_obj.name());
            let bytes = read_blob_bytes(file_obj.as_ref()).await?;
            (name, bytes)
        } else if let Some(blob) = file.dyn_ref::<Blob>() {
            let name = filename.unwrap_or_else(|| "workbook.xlsx".to_string());
            let bytes = read_blob_bytes(blob).await?;
            (name, bytes)
        } else {
            return Err(JsValue::from_str("Expected File or Blob object"));
        };

        web::queue_file(name, bytes).map_err(|error| JsValue::from_str(&error))
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

    /// Zoom to fit the current viewport.
    #[wasm_bindgen(js_name = zoomToFit)]
    pub fn zoom_to_fit(&mut self) -> Result<(), JsValue> {
        send_command("preview.zoomToFit", json!({}))
    }

    /// Set zoom to a specific percentage.
    #[wasm_bindgen(js_name = setZoom)]
    pub fn set_zoom(&mut self, percentage: f64) -> Result<(), JsValue> {
        if !percentage.is_finite() {
            return Err(JsValue::from_str("zoom percentage must be finite"));
        }
        send_command("view.zoom", json!({"percent": percentage.clamp(10.0, 400.0)}))
    }

    /// Preview builds are strictly read-only; enabling editing returns an error.
    #[wasm_bindgen(js_name = setEditable)]
    pub fn set_editable(&mut self, editable: bool) -> Result<(), JsValue> {
        if editable {
            return Err(JsValue::from_str(READ_ONLY_ERROR));
        }
        send_command("preview.setEditable", json!({"editable": false}))
    }

    /// Preview builds are strictly read-only; disabling read-only returns an error.
    #[wasm_bindgen(js_name = setReadOnly)]
    pub fn set_read_only(&mut self, read_only: bool) -> Result<(), JsValue> {
        self.set_editable(!read_only)
    }

    /// Get list of sheet names in the active workbook.
    #[wasm_bindgen(js_name = getSheetNames)]
    pub fn get_sheet_names(&self) -> Result<Vec<JsValue>, JsValue> {
        Ok(web::snapshot().sheet_names.into_iter().map(JsValue::from).collect())
    }

    /// Get the currently active sheet name.
    #[wasm_bindgen(js_name = getCurrentSheet)]
    pub fn get_current_sheet(&self) -> Result<String, JsValue> {
        Ok(web::snapshot().current_sheet)
    }

    /// Switch to a different sheet by name.
    #[wasm_bindgen(js_name = switchSheet)]
    pub fn switch_sheet(&mut self, name: String) -> Result<(), JsValue> {
        send_command("sheet.activate", json!({"sheet": name}))
    }

    /// Get the current selection range in A1 notation.
    #[wasm_bindgen(js_name = getSelection)]
    pub fn get_selection(&self) -> Result<String, JsValue> {
        Ok(web::snapshot().selection)
    }

    /// Select a cell or range.
    #[wasm_bindgen(js_name = selectRange)]
    pub fn select_range(&mut self, range: String) -> Result<(), JsValue> {
        send_command("selection.set", json!({"range": range}))
    }

    /// Queue `edit.find` for text in the current sheet.
    #[wasm_bindgen(js_name = find)]
    pub fn find(&mut self, query: String, match_case: Option<bool>) -> Result<(), JsValue> {
        if query.is_empty() {
            return Err(JsValue::from_str("search query cannot be empty"));
        }
        let match_case = match_case.unwrap_or(false);
        self.last_find = Some((query.clone(), match_case));
        send_command("edit.find", json!({"what": query, "matchCase": match_case}))
    }

    /// Queue `edit.find` again with the previous query.
    #[wasm_bindgen(js_name = findNext)]
    pub fn find_next(&mut self) -> Result<(), JsValue> {
        let Some((query, match_case)) = self.last_find.clone() else {
            return Err(JsValue::from_str("call find() before findNext()"));
        };
        send_command("edit.find", json!({"what": query, "matchCase": match_case}))
    }

    /// Backward search is not supported by the engine.
    #[wasm_bindgen(js_name = findPrevious)]
    pub fn find_previous(&mut self) -> Result<(), JsValue> {
        Err(JsValue::from_str("findPrevious is not supported by the engine"))
    }

    /// Set the interface language (`en` or `zh` in the slim preview build).
    #[wasm_bindgen(js_name = setLanguage)]
    pub fn set_language(&mut self, lang: String) -> Result<(), JsValue> {
        send_command("app.language.set", json!({"language": lang}))
    }

    /// Set the theme (`"light"`, `"dark"`, or `"auto"`).
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

fn send_command(command: &str, params: serde_json::Value) -> Result<(), JsValue> {
    web::queue_command(command, params).map_err(|error| JsValue::from_str(&error))
}

async fn read_blob_bytes(blob: &Blob) -> Result<Vec<u8>, JsValue> {
    let buffer = JsFuture::from(blob.array_buffer()).await?;
    Ok(js_sys::Uint8Array::new(&buffer).to_vec())
}
