//! Lightweight dialog suite for preview-only builds (`not(feature = "full")`): Find,
//! Go To, Zoom, encrypted-workbook password entry, and external-link confirmation.

use egui::{Key, Sense, vec2};
use gridcraft_engine::core::CellRef;
use serde_json::{Map, Value as Json, json};

use crate::SheetApp;

#[derive(Clone, Debug)]
pub struct Dialog {
    pub name: String,
    pub title: String,
    pub values: Map<String, Json>,
    pub result: Option<Json>,
    link: Option<String>,
    focus_initialized: bool,
}

impl Dialog {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub(crate) fn home_sheet_protected(&self, _app: &SheetApp) -> Option<bool> {
        None
    }

    pub(crate) fn end_pick(&mut self, _app: &mut SheetApp) -> Result<(), String> {
        Ok(())
    }

    pub(crate) fn note_selecting_press(&mut self) {}

    pub(crate) fn ref_box_picks(&self) -> bool {
        false
    }

    pub fn picks_references(&self) -> bool {
        false
    }

    pub fn pick_reference(&mut self, _cell: CellRef, _active_sheet: &str, _extend: bool) {}

    pub fn open(app: &mut SheetApp, name: &str, params: Json) -> Option<Dialog> {
        let p = |k: &str| params.get(k).cloned().unwrap_or(Json::Null);
        match name {
            "find" => Some(Self::custom("find", "Find", json!({"what": "", "within": "sheet", "lookIn": "formulas"}))),
            "goTo" => Some(Self::custom("goTo", "Go To", json!({"reference": ""}))),
            "zoom" => {
                let current = app.session.active().and_then(|d| d.wb.active()).map_or(100, |s| s.zoom);
                let mut d = Self::custom("zoom", "Zoom", json!({"preset": zoom_preset(f64::from(current)), "percent": current}));
                d.values.insert("preset".into(), json!(zoom_preset(f64::from(current))));
                Some(d)
            }
            "openPassword" => {
                let source = p("path").as_str().or(p("name").as_str()).map(str::to_string).unwrap_or_default();
                let file = std::path::Path::new(&source).file_name().and_then(std::ffi::OsStr::to_str).unwrap_or("This workbook").to_string();
                let mut values = params.as_object().cloned().unwrap_or_default();
                values.insert("password".into(), json!(""));
                values.insert("_note".into(), json!(format!("'{file}' is protected.")));
                Some(Dialog { name: "openPassword".into(), title: "Password".into(), values, result: None, link: None, focus_initialized: false })
            }
            "openLink" => {
                let url = params.get("url").and_then(Json::as_str).unwrap_or_default().to_string();
                let mut d = Self::custom("openLink", "Open Hyperlink", json!({"url": url}));
                d.link = Some(url);
                Some(d)
            }
            _ => None,
        }
    }

    fn custom(name: &str, title: &str, defaults: Json) -> Dialog {
        Dialog {
            name: name.into(),
            title: title.into(),
            values: defaults.as_object().cloned().unwrap_or_default(),
            result: None,
            link: None,
            focus_initialized: false,
        }
    }
}

pub(crate) fn grid_sense(_app: &SheetApp) -> Sense {
    Sense::click_and_drag()
}

pub(crate) fn refuse_sheet_change(_app: &SheetApp, _id: &str) -> Option<String> {
    None
}

pub(crate) fn insert_name(_app: &mut SheetApp, _name: &str) {}

const ZOOM_PRESETS: [(&str, &str); 5] = [("200", "200%"), ("100", "100%"), ("75", "75%"), ("50", "50%"), ("25", "25%")];

fn zoom_preset(percent: f64) -> &'static str {
    ZOOM_PRESETS.iter().map(|(val, _)| *val).find(|val| val.parse::<f64>().ok() == Some(percent)).unwrap_or("custom")
}

fn ok_cancel(ui: &mut egui::Ui, confirm: &mut bool, open: &mut bool) {
    ui.add_space(8.0);
    ui.horizontal(|ui| {
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            let lang = crate::i18n::current(ui.ctx());
            let t = crate::theme::Tokens::get(ui.ctx());
            let ok = ui.add(egui::Button::new(egui::RichText::new(format!("   {}   ", lang.tr("OK"))).color(t.on_accent)).fill(t.accent));
            if ok.clicked() || ui.input(|i| i.key_pressed(Key::Enter)) {
                *confirm = true;
            }
            if ui.button(lang.tr("Cancel")).clicked() {
                *open = false;
            }
        });
    });
}

fn show_find(app: &mut SheetApp, ui: &mut egui::Ui, d: &mut Dialog) {
    let lang = crate::i18n::current(ui.ctx());
    let mut what = d.values.get("what").and_then(Json::as_str).unwrap_or("").to_string();
    ui.horizontal(|ui| {
        let label = ui.label(lang.tr("Find what:"));
        let r = ui.add(egui::TextEdit::singleline(&mut what).desired_width(260.0)).labelled_by(label.id);
        if !d.focus_initialized {
            r.request_focus();
            d.focus_initialized = true;
        }
    });
    let mut case = d.values.get("matchCase").and_then(Json::as_bool).unwrap_or(false);
    let mut whole = d.values.get("wholeCell").and_then(Json::as_bool).unwrap_or(false);
    let mut wb = d.values.get("within").and_then(Json::as_str) == Some("workbook");
    ui.horizontal(|ui| {
        ui.checkbox(&mut case, lang.tr("Match case"));
        ui.checkbox(&mut whole, lang.tr("Match entire cell contents"));
        ui.checkbox(&mut wb, lang.tr("Within workbook"));
    });
    d.values.insert("what".into(), json!(what));
    d.values.insert("matchCase".into(), json!(case));
    d.values.insert("wholeCell".into(), json!(whole));
    d.values.insert("within".into(), json!(if wb { "workbook" } else { "sheet" }));

    let within = if wb { "workbook" } else { "sheet" };
    let base = json!({"what": what, "matchCase": case, "wholeCell": whole, "within": within});
    ui.horizontal(|ui| {
        if ui.button(lang.tr("Find All")).clicked() {
            let mut p = base.clone();
            p["all"] = json!(true);
            d.result = app.run("edit.find", p).ok();
        }
        if ui.button(lang.tr("Find Next")).clicked() || ui.input(|i| i.key_pressed(Key::Enter)) {
            if let Err(e) = app.run("edit.find", base.clone()) {
                app.message = Some((lang.tr("Find").into(), crate::clean_error(&e)));
            }
            app.grid.ensure_visible = true;
        }
    });
    if let Some(r) = &d.result
        && let Some(list) = r["results"].as_array()
    {
        ui.separator();
        egui::ScrollArea::vertical().max_height(180.0).show(ui, |ui| {
            for item in list {
                let label = format!(
                    "{}  {}  {}",
                    item["sheet"].as_str().unwrap_or(""),
                    item["cell"].as_str().unwrap_or(""),
                    item["value"].as_str().unwrap_or("")
                );
                if ui.selectable_label(false, label).clicked() {
                    if let Some(s) = item["sheet"].as_str() {
                        let _ = app.run("sheet.activate", json!({"sheet": s}));
                    }
                    let _ = app.run("selection.set", json!({"cell": item["cell"]}));
                    app.grid.ensure_visible = true;
                }
            }
        });
    }
}

fn show_zoom(ui: &mut egui::Ui, d: &mut Dialog, confirm: &mut bool, open: &mut bool) {
    let lang = crate::i18n::current(ui.ctx());
    let cur = d.values.get("preset").and_then(Json::as_str).unwrap_or("custom").to_string();
    ui.label(egui::RichText::new(lang.tr("Magnification")).strong());
    for (val, text) in ZOOM_PRESETS {
        if ui.radio(cur == val, text).clicked() {
            d.values.insert("preset".into(), json!(val));
            if let Ok(p) = val.parse::<f64>() {
                d.values.insert("percent".into(), json!(p));
            }
        }
    }
    if ui.radio(cur == "fit", lang.tr("Fit selection")).clicked() {
        d.values.insert("preset".into(), json!("fit"));
    }
    ui.horizontal(|ui| {
        if ui.radio(cur == "custom", format!("{}:", lang.tr("Custom"))).clicked() {
            d.values.insert("preset".into(), json!("custom"));
        }
        let mut percent = d.values.get("percent").and_then(Json::as_f64).unwrap_or(100.0).round() as i64;
        if ui.add(egui::DragValue::new(&mut percent).range(10..=400).speed(1.0)).changed() {
            d.values.insert("percent".into(), json!(percent));
            d.values.insert("preset".into(), json!(zoom_preset(percent as f64)));
        }
        ui.label("%");
    });
    ok_cancel(ui, confirm, open);
}

pub fn show(app: &mut SheetApp, ctx: &egui::Context) {
    let Some(mut d) = app.dialog.take() else { return };
    if app.message.is_some() {
        app.dialog = Some(d);
        return;
    }
    let mut open = true;
    let mut win_open = true;
    let mut confirm = false;
    let width = if d.name == "zoom" { 220.0 } else { 380.0 };
    let window_id = egui::Id::unique("dialog").with(&d.name);
    let lang = app.ui.language;
    egui::Window::new(lang.tr(&d.title))
        .id(window_id)
        .collapsible(false)
        .resizable(false)
        .default_width(width)
        .pivot(egui::Align2::CENTER_CENTER)
        .default_pos(ctx.content_rect().center() + vec2(0.0, -40.0))
        .open(&mut win_open)
        .show(ctx, |ui| {
            ui.set_min_width(width - 20.0);
            match d.name.as_str() {
                "find" => show_find(app, ui, &mut d),
                "goTo" => {
                    let label = ui.label(lang.tr("Reference:"));
                    let mut reference = d.values.get("reference").and_then(Json::as_str).unwrap_or("").to_string();
                    let r = ui.add(egui::TextEdit::singleline(&mut reference).desired_width(240.0)).labelled_by(label.id);
                    if !d.focus_initialized {
                        r.request_focus();
                        d.focus_initialized = true;
                    }
                    d.values.insert("reference".into(), json!(reference));
                    ok_cancel(ui, &mut confirm, &mut open);
                }
                "zoom" => show_zoom(ui, &mut d, &mut confirm, &mut open),
                "openPassword" => {
                    if let Some(note) = d.values.get("_note").and_then(Json::as_str) {
                        ui.label(note);
                    }
                    let label = ui.label(lang.tr("Password:"));
                    let mut password = d.values.get("password").and_then(Json::as_str).unwrap_or("").to_string();
                    let r = ui.add(egui::TextEdit::singleline(&mut password).password(true).desired_width(240.0)).labelled_by(label.id);
                    if !d.focus_initialized {
                        r.request_focus();
                        d.focus_initialized = true;
                    }
                    d.values.insert("password".into(), json!(password));
                    ok_cancel(ui, &mut confirm, &mut open);
                }
                "openLink" => {
                    if let Some(url) = &d.link {
                        ui.label(url);
                    }
                    ok_cancel(ui, &mut confirm, &mut open);
                }
                _ => {}
            }
        });
    if ctx.input(|i| i.key_pressed(Key::Escape)) || !win_open {
        open = false;
    }
    if confirm {
        match d.name.as_str() {
            "goTo" => {
                let reference = d.values.get("reference").and_then(Json::as_str).unwrap_or("").to_string();
                match app.run("edit.goTo", json!({"reference": reference})) {
                    Ok(_) => {
                        app.grid.ensure_visible = true;
                        open = false;
                    }
                    Err(e) => app.message = Some((lang.tr(&d.title).into(), crate::clean_error(&e))),
                }
            }
            "zoom" => {
                let r = if d.values.get("preset").and_then(Json::as_str) == Some("fit") {
                    let size = app.grid.rect.map(|r| (r.width(), r.height())).unwrap_or((1000.0, 600.0));
                    app.run("view.zoomToSelection", json!({"viewWidth": size.0, "viewHeight": size.1}))
                } else {
                    let percent = d.values.get("percent").and_then(Json::as_f64).unwrap_or(100.0);
                    app.run("view.zoom", json!({"percent": percent}))
                };
                match r {
                    Ok(_) => open = false,
                    Err(e) => app.message = Some((lang.tr(&d.title).into(), crate::clean_error(&e))),
                }
            }
            "openPassword" => {
                let mut params = d.values.clone();
                params.remove("_note");
                match app.run("file.open", Json::Object(params)) {
                    Ok(r) => {
                        let path = d.values.get("path").and_then(Json::as_str).map(str::to_string);
                        app.after_open(path.as_deref(), &r);
                        open = false;
                    }
                    Err(e) => {
                        d.values.insert("password".into(), json!(""));
                        app.message = Some((lang.tr(&d.title).into(), crate::clean_error(&e)));
                    }
                }
            }
            "openLink" => {
                let url = d.link.as_deref().unwrap_or_default().to_string();
                if let Err(e) = app.session.open_confirmed_link(&url) {
                    app.message = Some((lang.tr(&d.title).into(), crate::clean_error(&e.to_string())));
                } else {
                    app.after_engine();
                    open = false;
                }
            }
            _ => open = false,
        }
    }
    if open {
        app.dialog = Some(d);
    }
}
