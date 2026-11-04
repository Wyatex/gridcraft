//! Preview session wrapper that enables the engine's read-only policy.

use gridcraft_engine::{Result, Session};
use serde_json::Value as Json;

/// Wrapper around a read-only engine session for embedders that prefer an explicit type.
pub struct PreviewSession {
    session: Session,
}

impl PreviewSession {
    pub fn new() -> Self {
        let mut session = Session::new();
        session.set_read_only(true);
        Self {
            session,
        }
    }

    /// Execute a command under the engine's read-only policy.
    pub fn execute(&mut self, id: &str, params: Json) -> Result<Json> {
        self.session.execute(id, params)
    }

    /// Get the underlying session (read-only access for UI).
    pub fn session(&self) -> &Session {
        &self.session
    }

}

impl Default for PreviewSession {
    fn default() -> Self {
        Self::new()
    }
}
