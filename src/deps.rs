use crate::error::Error;
use anyhow::Result;

/// Required external tools and their install hints.
const REQUIRED: &[(&str, &str)] = &[
    (
        "slurp",
        "pacman -S slurp | apt install slurp | dnf install slurp",
    ),
    (
        "wf-recorder",
        "pacman -S wf-recorder | apt install wf-recorder | dnf install wf-recorder",
    ),
    (
        "ffmpeg",
        "pacman -S ffmpeg | apt install ffmpeg | dnf install ffmpeg",
    ),
];

/// Verify all required external tools are available in PATH.
pub fn check() -> Result<()> {
    for (cmd, hint) in REQUIRED {
        if which::which(cmd).is_err() {
            return Err(Error::MissingDependency { cmd, hint }.into());
        }
    }
    Ok(())
}
