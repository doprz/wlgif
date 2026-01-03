use anyhow::Result;
use std::path::Path;

use crate::region::Region;
mod wlr;

pub struct RecordConfig {
    pub fps: u32,
    pub duration: f32,
    pub quiet: bool,
}

/// A screencast backend
pub trait Backend {
    /// Backend name
    fn name(&self) -> &'static str;

    /// Check if this backend is available
    fn is_available(&self) -> Result<()>;

    /// Record a screen region to a video file.
    ///
    /// For backends that don't support region selection (like xdg-portal),
    /// the region parameter may be ignored and full-screen capture used.
    fn record(&self, region: Option<&Region>, output: &Path, config: &RecordConfig) -> Result<()>;
}

/// Detect and return the best available backend
pub fn detect() -> Result<Box<dyn Backend>> {
    let wlr = wlr::WlrBackend::new();
    if wlr.is_available().is_ok() {
        return Ok(Box::new(wlr));
    }

    // TODO: Add xdg-portal support

    anyhow::bail!(
        "no recording backend available\n  \
         install either:\n    \
         - xdg-desktop-portal + pipewire + gstreamer (recommended)\n    \
         - wf-recorder (wlroots compositors only)"
    )
}

/// Get a specific backend by name
pub fn by_name(name: &str) -> Result<Box<dyn Backend>> {
    match name {
        // "xdg-portal" | "xdg" | "portal" => Ok(Box::new()),
        "wlr" | "wlroots" => Ok(Box::new(wlr::WlrBackend::new())),
        _ => anyhow::bail!("unknown backend: {}", name),
    }
}
