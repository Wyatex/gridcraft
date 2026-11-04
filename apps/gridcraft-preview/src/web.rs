//! The browser shell for GridCraft Preview.
//!
//! Starts the eframe web runner with preview-specific configuration.

use gridcraft_engine::Session;
use gridcraft_ui_egui::{Inbox, SheetApp};
use std::collections::VecDeque;
use std::sync::{Arc, Mutex};
use wasm_bindgen::JsCast as _;

use crate::preview_ui;

const CANVAS_ID: &str = "gridcraft_canvas";
const LOADING_ID: &str = "gridcraft_loading";

/// Commands and files sent by the JavaScript API are applied by the eframe app on its next frame.
/// The browser and eframe share a thread, but the queue keeps the JS-facing API independent from
/// the `SheetApp` value owned by the runner closure.
struct PreviewBridge {
    inbox: Inbox,
    commands: VecDeque<(String, serde_json::Value)>,
}

static PREVIEW_BRIDGE: once_cell::sync::Lazy<Arc<Mutex<PreviewBridge>>> =
    once_cell::sync::Lazy::new(|| Arc::new(Mutex::new(PreviewBridge { inbox: Inbox::default(), commands: VecDeque::new() })));

static PREVIEW_CONTEXT: once_cell::sync::Lazy<Arc<Mutex<Option<egui::Context>>>> = once_cell::sync::Lazy::new(|| Arc::new(Mutex::new(None)));

/// Queue a command for the running preview application.
pub(crate) fn queue_command(id: &str, params: serde_json::Value) -> Result<(), String> {
    let mut bridge = PREVIEW_BRIDGE.lock().map_err(|_| "preview bridge is unavailable".to_string())?;
    bridge.commands.push_back((id.to_string(), params));
    drop(bridge);
    request_repaint();
    Ok(())
}

/// Queue bytes from a browser `File`/`Blob` for the app's normal asynchronous open path.
pub(crate) fn queue_file(name: String, bytes: Vec<u8>) -> Result<(), String> {
    let bridge = PREVIEW_BRIDGE.lock().map_err(|_| "preview bridge is unavailable".to_string())?;
    bridge.inbox.lock().map_err(|_| "preview file inbox is unavailable".to_string())?.push((name, bytes));
    drop(bridge);
    request_repaint();
    Ok(())
}

fn request_repaint() {
    if let Ok(guard) = PREVIEW_CONTEXT.lock()
        && let Some(ctx) = guard.as_ref()
    {
        ctx.request_repaint();
    }
}

/// Starts the eframe web runner for GridCraft Preview.
pub fn start() {
    eframe::WebLogger::init(log::LevelFilter::Info).ok();

    wasm_bindgen_futures::spawn_local(async {
        let Some(document) = web_sys::window().and_then(|w| w.document()) else {
            log::error!("no document");
            return;
        };

        let Some(canvas) = document.get_element_by_id(CANVAS_ID).and_then(|e| e.dyn_into::<web_sys::HtmlCanvasElement>().ok()) else {
            log::error!("missing <canvas id=\"{CANVAS_ID}\">");
            return;
        };

        let mut options = eframe::WebOptions::default();

        // Adjust WebGPU device limits
        if let eframe::egui_wgpu::WgpuSetup::CreateNew(setup) = &mut options.wgpu_options.wgpu_setup {
            let default_descriptor = std::sync::Arc::clone(&setup.device_descriptor);

            setup.device_descriptor = std::sync::Arc::new(move |adapter| {
                let mut descriptor = default_descriptor(adapter);
                descriptor.required_limits = descriptor.required_limits.or_worse_values_from(&adapter.limits());
                descriptor
            });
        }

        // Force WebGL if ?webgl in URL
        if query().contains("webgl") {
            if let eframe::egui_wgpu::WgpuSetup::CreateNew(create) = &mut options.wgpu_options.wgpu_setup {
                create.instance_descriptor.backends = eframe::wgpu::Backends::GL;
            }
        }

        let result = eframe::WebRunner::new()
            .start(
                canvas,
                options,
                Box::new(move |cc| {
                    let inbox =
                        PREVIEW_BRIDGE.lock().map(|bridge| bridge.inbox.clone()).unwrap_or_else(|poisoned| poisoned.into_inner().inbox.clone());
                    let mut services = preview_ui::preview_services();
                    services.inbox = Some(inbox.clone());

                    // Set up copy_html if browser supports it
                    services.copy_html = Some(Box::new(copy_html));

                    // Get a session clone for the app
                    let session = Session::new();
                    let mut app = SheetApp::new(session, services);

                    // Preview is a viewer by default. Keep the policy at the engine boundary so
                    // commands from the toolbar, keyboard, clipboard and embedding API all share
                    // the same read-only behavior.
                    app.set_read_only(true);

                    // Configure for preview mode
                    preview_ui::configure_preview_app(&mut app);

                    // Follow browser's language (except CJK due to font requirements)
                    let browser = gridcraft_ui_egui::i18n::Language::system();
                    if !matches!(
                        browser,
                        gridcraft_ui_egui::i18n::Language::Zh | gridcraft_ui_egui::i18n::Language::Ja | gridcraft_ui_egui::i18n::Language::Ko
                    ) {
                        app.ui.language = browser;
                    }

                    // Follow system theme
                    app.ui.system_theme = true;

                    SheetApp::setup_context_for_language(&cc.egui_ctx, false, app.ui.language);

                    if let Ok(mut context) = PREVIEW_CONTEXT.lock() {
                        *context = Some(cc.egui_ctx.clone());
                    }

                    Ok(Box::new(PreviewShell { app, inbox }))
                }),
            )
            .await;

        // Handle loading screen
        let is_ok = result.is_ok();
        if let Some(el) = document.get_element_by_id(LOADING_ID) {
            match result {
                Ok(()) => el.remove(),
                Err(ref e) => el.set_inner_html(&format!(
                    "<p>GridCraft Preview failed to start: {e:?}</p>\
                     <p>A browser with WebGPU or WebGL2 is required.</p>"
                )),
            }
        }

        // Notify that preview API is ready
        if is_ok {
            if let Some(window) = web_sys::window() {
                let event = web_sys::CustomEvent::new("gridcraft-preview-ready").ok();
                if let Some(event) = event {
                    let _ = window.dispatch_event(&event);
                }
            }
        }
    });
}

fn query() -> String {
    web_sys::window().and_then(|w| w.location().search().ok()).unwrap_or_default()
}

struct PreviewShell {
    app: SheetApp,
    inbox: Inbox,
}

impl PreviewShell {
    fn run_preview_command(&mut self, ctx: &egui::Context, id: &str, params: serde_json::Value) -> Result<(), String> {
        match id {
            // These convenience commands are exposed by the JS API, while the engine stores a
            // concrete percentage on the active sheet.
            "preview.zoomIn" | "preview.zoomOut" => {
                let current = self.app.session.active().and_then(|doc| doc.wb.active()).map(|sheet| sheet.zoom).unwrap_or(100);
                let delta: i32 = if id == "preview.zoomIn" { 10 } else { -10 };
                let percent = (i32::from(current) + delta).clamp(10, 400);
                self.app.run("view.zoom", serde_json::json!({"percent": percent}))?;
            }
            "preview.zoomToFit" => {
                self.app.run("view.zoomToSelection", params)?;
            }
            "preview.showXlsImportNotice" => {
                self.app.show_xls_import_notice = params.get("show").and_then(serde_json::Value::as_bool).unwrap_or(false);
            }
            "preview.setEditable" => {
                let editable = params.get("editable").and_then(serde_json::Value::as_bool).unwrap_or(false);
                self.app.set_read_only(!editable);
            }
            _ => {
                self.app.run(id, params)?;
            }
        }
        ctx.request_repaint();
        Ok(())
    }
}

impl eframe::App for PreviewShell {
    fn logic(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let commands = PREVIEW_BRIDGE.lock().map(|mut bridge| bridge.commands.drain(..).collect::<Vec<_>>()).unwrap_or_default();
        for (id, params) in commands {
            if let Err(error) = self.run_preview_command(ctx, &id, params) {
                log::warn!("preview command {id} failed: {error}");
            }
        }

        let dropped = ctx.input_mut(|i| std::mem::take(&mut i.raw.dropped_files));
        for f in dropped {
            let inbox = self.inbox.clone();
            let ctx = ctx.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let name = f.path().file_name().map(|n| n.to_string_lossy().to_string()).unwrap_or_else(|| "dropped.xlsx".into());
                match f.bytes_async().await {
                    Ok(bytes) => {
                        inbox.lock().unwrap_or_else(|e| e.into_inner()).push((name, bytes));
                        ctx.request_repaint();
                    }
                    Err(e) => log::error!("couldn't read dropped file {name}: {e}"),
                }
            });
        }

        self.app.logic(ctx);
    }

    fn raw_input_hook(&mut self, ctx: &egui::Context, raw: &mut egui::RawInput) {
        self.app.raw_input_hook(ctx, raw);
    }

    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        self.app.ui(ui);
    }
}

/// Copy HTML to clipboard (if browser supports it).
fn copy_html(html: &str, text: &str) -> Result<(), String> {
    let js = |e: wasm_bindgen::JsValue| format!("{e:?}");
    let window = web_sys::window().ok_or("no window")?;
    let clipboard = window.navigator().clipboard();

    if clipboard.is_undefined() || clipboard.is_null() {
        return Err("browser clipboard is unavailable (a secure context is required)".into());
    }

    if !js_sys::Reflect::get(&clipboard, &"write".into()).map_err(js)?.is_function() {
        return Err("browser does not support rich clipboard writes".into());
    }

    let data = js_sys::Object::new();
    for (mime, value) in [("text/html", html), ("text/plain", text)] {
        let opts = web_sys::BlobPropertyBag::new();
        opts.set_type(mime);
        let parts = js_sys::Array::of1(&wasm_bindgen::JsValue::from_str(value));
        let blob = web_sys::Blob::new_with_str_sequence_and_options(&parts, &opts).map_err(js)?;
        js_sys::Reflect::set(&data, &mime.into(), &blob).map_err(js)?;
    }

    let item = web_sys::ClipboardItem::new_with_record_from_str_to_blob_promise(&data).map_err(js)?;
    let promise = clipboard.write(&js_sys::Array::of1(&item));
    let text = text.to_owned();

    wasm_bindgen_futures::spawn_local(async move {
        if let Err(e) = wasm_bindgen_futures::JsFuture::from(promise).await {
            log::warn!("Rich clipboard write failed; trying plain text: {e:?}");
            if let Err(e) = wasm_bindgen_futures::JsFuture::from(clipboard.write_text(&text)).await {
                log::error!("Plain-text clipboard fallback also failed: {e:?}");
            }
        }
    });

    Ok(())
}
