//! GridCraft Preview - Read-only XLSX/XLS viewer for the web.
//!
//! A WASM-based preview library that provides read-only viewing of spreadsheet files.
//! All editing commands are disabled at the engine level.
//!
//! Build with `trunk build --release` from this directory.
#![deny(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::unimplemented, clippy::todo, clippy::unreachable)]

#[cfg(target_arch = "wasm32")]
mod preview_api;
#[cfg(target_arch = "wasm32")]
mod preview_ui;
#[cfg(target_arch = "wasm32")]
mod web;

#[cfg(target_arch = "wasm32")]
fn main() {
    web::start();
}

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    eprintln!("gridcraft-preview only runs in the browser: build it with `trunk build --release` in apps/gridcraft-preview");
}
