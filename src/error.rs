use thiserror::Error;

/// Domain-specific errors for wlgif operations.
#[derive(Debug, Error)]
pub enum Error {
    #[error("missing dependency: {cmd}\n  install: {hint}")]
    MissingDependency {
        cmd: &'static str,
        hint: &'static str,
    },

    #[error("region selection cancelled")]
    SelectionCancelled,

    #[error("invalid region format: {0}\n  expected: WxH+X+Y (e.g., 800x600+100+100)")]
    InvalidRegion(String),

    #[error("recording failed: {0}")]
    Recording(String),

    #[error("conversion failed: {0}")]
    Conversion(String),

    #[error("no video data captured")]
    EmptyRecording,
}
