use crate::error::Error;
use crate::output;
use anyhow::{Context, Result};
use std::path::Path;
use std::process::{Command, Stdio};
use tempfile::TempDir;

/// Convert a video file to an optimized GIF.
pub fn to_gif(
    input: &Path,
    output: &Path,
    fps: u32,
    width: Option<u32>,
    optimize: bool,
    quiet: bool,
) -> Result<()> {
    if !quiet {
        output::status("Converting to GIF...");
    }

    let scale = match width {
        Some(w) => format!("scale={}:-1:flags=lanczos", w),
        None => "scale=trunc(iw/2)*2:trunc(ih/2)*2".into(),
    };

    let base_filter = format!("fps={},{}", fps, scale);

    if optimize {
        convert_optimized(input, output, &base_filter, quiet)?;
    } else {
        convert_fast(input, output, &base_filter)?;
    }

    Ok(())
}

/// Two-pass conversion with palette optimization.
fn convert_optimized(input: &Path, output: &Path, base_filter: &str, quiet: bool) -> Result<()> {
    let temp = TempDir::new().context("failed to create temp directory")?;
    let palette = temp.path().join("palette.png");

    if !quiet {
        output::info("Generating optimal palette...");
    }

    // Pass 1: Generate palette
    let status = Command::new("ffmpeg")
        .args(["-y", "-i"])
        .arg(input)
        .args(["-vf", &format!("{}[x];[x]palettegen=stats_mode=diff", base_filter)])
        .arg(&palette)
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .context("ffmpeg palette generation failed")?;

    if !status.success() {
        return Err(Error::Conversion("palette generation failed".into()).into());
    }

    if !quiet {
        output::info("Encoding GIF...");
    }

    // Pass 2: Apply palette with dithering
    let status = Command::new("ffmpeg")
        .args(["-y", "-i"])
        .arg(input)
        .args(["-i", palette.to_str().unwrap()])
        .args([
            "-lavfi",
            &format!("{}[x];[x][1:v]paletteuse=dither=floyd_steinberg", base_filter),
        ])
        .arg(output)
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .context("ffmpeg GIF encoding failed")?;

    if !status.success() {
        return Err(Error::Conversion("GIF encoding failed".into()).into());
    }

    Ok(())
}

/// Single-pass fast conversion.
fn convert_fast(input: &Path, output: &Path, base_filter: &str) -> Result<()> {
    let status = Command::new("ffmpeg")
        .args(["-y", "-i"])
        .arg(input)
        .args(["-vf", base_filter])
        .arg(output)
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .context("ffmpeg conversion failed")?;

    if !status.success() {
        return Err(Error::Conversion("conversion failed".into()).into());
    }

    Ok(())
}
