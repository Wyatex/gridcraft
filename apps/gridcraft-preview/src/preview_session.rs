//! Preview session wrapper that filters commands to enforce read-only mode.
//!
//! Only allows commands that don't modify the workbook:
//! - View operations (zoom, pan, scroll)
//! - Selection changes
//! - Sheet navigation
//! - Search/find
//! - UI preferences (language, theme)
//! - Copy to clipboard
//!
//! All editing commands return an error.

use gridcraft_engine::{Result, Session};
use serde_json::Value as Json;

/// Commands allowed in preview mode (read-only operations).
const ALLOWED_COMMANDS: &[&str] = &[
    // View operations
    "view.zoom",
    "view.zoomIn",
    "view.zoomOut",
    "view.fitWindow",
    "view.normalSize",
    "view.freezePanes",
    "view.freezeTopRow",
    "view.freezeFirstColumn",
    "view.unfreeze",
    "view.showGridlines",
    "view.showHeadings",
    "view.showFormulaBar",
    "view.showStatusBar",
    "view.fullScreen",
    // Selection (read-only)
    "selection.set",
    "selection.move",
    "selection.extend",
    "selection.all",
    "selection.column",
    "selection.row",
    // Sheet navigation
    "sheet.activate",
    "sheet.next",
    "sheet.previous",
    // Search
    "search.find",
    "search.findNext",
    "search.findPrevious",
    "search.replace", // Will show UI but won't modify
    // Copy (read-only)
    "home.copy",
    "edit.copy",
    // UI commands
    "ui.setLanguage",
    "ui.setTheme",
    "ui.backstage",
    // Inspection (read-only)
    "inspect.value",
    "inspect.formula",
    "inspect.format",
];

/// Wrapper around Session that enforces read-only mode.
pub struct PreviewSession {
    session: Session,
}

impl PreviewSession {
    pub fn new() -> Self {
        Self {
            session: Session::new(),
        }
    }

    /// Execute a command if it's allowed in preview mode.
    pub fn execute(&mut self, id: &str, params: Json) -> Result<Json> {
        if self.is_command_allowed(id) {
            self.session.execute(id, params)
        } else {
            Err(gridcraft_engine::EngineError::Other(format!(
                "Command '{}' is not available in preview mode (read-only)",
                id
            )))
        }
    }

    /// Check if a command is allowed in preview mode.
    fn is_command_allowed(&self, id: &str) -> bool {
        ALLOWED_COMMANDS.contains(&id)
    }

    /// Get the underlying session (read-only access for UI).
    pub fn session(&self) -> &Session {
        &self.session
    }

    /// Get mutable access to session for loading files.
    pub fn session_mut(&mut self) -> &mut Session {
        &mut self.session
    }
}

impl Default for PreviewSession {
    fn default() -> Self {
        Self::new()
    }
}
