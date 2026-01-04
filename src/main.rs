mod backend;
mod cli;
mod converter;
mod error;
mod output;
mod region;

use anyhow::{Context, Result, bail};
use cli::Args;
use error::Error;
use region::Region;
use std::fs;
use tempfile::TempDir;

use crate::backend::RecordConfig;

fn validate_output(args: &Args) -> Result<()> {
    // TODO: Improve extension check via file type
    match args.output.extension().and_then(|e| e.to_str()) {
        Some("gif") => Ok(()),
        Some(ext) => bail!("output must be a .gif file, got .{}", ext),
        None => bail!("output must be a .gif file"),
    }
}

fn get_region(args: &Args) -> Result<Option<Region>> {
    match &args.geometry {
        Some(g) => Ok(Some(Region::from_geometry(g)?)),
        // TODO: Improve matching
        None if args.backend.as_deref() == Some("xdg-desktop-portal") => {
            // Portal backend uses its own selection UI
            Ok(None)
        }
        None => Ok(Some(region::select_interactive(args.quiet)?)),
    }
}

fn main() -> Result<()> {
    let args = Args::parse_args();

    validate_output(&args)?;

    let backend = match &args.backend {
        Some(name) => backend::by_name(name)?,
        None => backend::detect()?,
    };

    if !args.quiet {
        output::info(&format!("Using {} backend", backend.name()));
    }

    if backend.is_available().is_err() {
        bail!(
            "backend '{}' is not available on this system",
            backend.name()
        );
    }

    let region = get_region(&args)?;

    if !args.quiet
        && let Some(ref r) = region
    {
        output::status(&format!("Region: {}", r));
    }

    let temp = TempDir::new().context("failed to create temp directory")?;
    let video = temp.path().join("capture.mp4");
    let config = RecordConfig {
        fps: args.fps,
        duration: args.duration,
        quiet: args.quiet,
    };

    backend.record(region.as_ref(), &video, &config)?;

    if !video.exists() {
        return Err(Error::EmptyRecording.into());
    }

    converter::to_gif(
        &video,
        &args.output,
        args.fps,
        args.width,
        !args.fast,
        args.quiet,
    )?;

    if args.keep_video {
        let kept = args.output.with_extension("mp4");
        fs::copy(&video, &kept).context("failed to save video")?;
        if !args.quiet {
            output::info(&format!("Video: {}", kept.display()));
        }
    }

    let size = fs::metadata(&args.output).map(|m| m.len()).unwrap_or(0);

    if !args.quiet {
        output::summary(&args.output, size);
    }

    Ok(())
}
