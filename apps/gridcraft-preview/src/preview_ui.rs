//! UI configuration for preview mode.
//!
//! Customizes the UI to show only read-only features:
//! - Hides title bar, File tab, and the entire ribbon bar
//! - Keeps formula bar for viewing formulas (non-editable)
//! - Removes file operations (Save, Export)

use gridcraft_ui_egui::{Chrome, Services, SheetApp};

/// Configure a SheetApp for preview mode.
pub fn configure_preview_app(app: &mut SheetApp) {
    // The embedding page owns the import flow, so the engine's verbose legacy-XLS compatibility
    // notice would duplicate information the host can present itself.
    app.show_xls_import_notice = false;

    // Hide title bar, File tab, and ribbon bar in preview mode
    app.chrome = Chrome {
        title_bar: false, // The embedding page owns the toolbar and window controls.
        file_tab: false,  // No File tab in preview mode
        ribbon: false,    // Hide the entire top ribbon bar (Home..Automate)
    };

    // Collapse ribbon by default if a host ever turns ribbon chrome back on
    app.ui.ribbon_collapsed = true;

    // Default tab if ribbon is ever enabled
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
    Services::default()
}
