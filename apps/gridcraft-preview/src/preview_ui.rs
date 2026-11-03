//! UI configuration for preview mode.
//!
//! Customizes the UI to show only read-only features:
//! - Hides editing ribbon tabs (Home, Insert, Data, etc.)
//! - Shows only View tab
//! - Disables formula bar editing
//! - Removes file operations (Save, Export)
//! - Simplifies title bar

use gridcraft_ui_egui::{Chrome, Services, SheetApp};

/// Configure a SheetApp for preview mode.
pub fn configure_preview_app(app: &mut SheetApp) {
    // The embedding page owns the import flow, so the engine's verbose legacy-XLS compatibility
    // notice would duplicate information the host can present itself.
    app.show_xls_import_notice = false;

    // Set chrome to hide file tab and simplify UI
    app.chrome = Chrome {
        title_bar: false, // The embedding page owns the toolbar and window controls.
        file_tab: false,  // No File tab in preview mode
    };

    // Collapse ribbon by default to maximize viewing area
    app.ui.ribbon_collapsed = true;

    // Start on View tab (the only useful tab in preview mode)
    app.ui.ribbon_tab = "View".to_string();

    // Disable status bar by default (can be toggled via View menu)
    app.ui.status_bar = true;

    // Keep formula bar for viewing formulas (but not editing)
    app.ui.formula_bar_expanded = false;

    // Disable autosave (no saving in preview mode)
    app.ui.autosave = false;

    // Enable animations for smooth experience
    app.ui.animations = true;
}

/// Get preview-specific services configuration.
pub fn preview_services() -> Services {
    Services {
        // No file operations in preview mode
        pick_open: None,
        pick_save: None,
        pick_picture: None,
        pick_picture_async: None,
        download: None,
        open_async: None,

        // Keep read-only services
        open_url: None,   // Could enable if links should work
        copy_html: None,  // Will be set by web.rs if browser supports it
        paste_html: None, // Preview mode doesn't need paste
        paste_text: None,
        print: None, // No printing in preview mode
        printers: None,
        print_native: None,
        inbox: None, // Will be set by web.rs for async file loading
    }
}
