mod cli;
mod converter;
mod deps;
mod error;
mod output;
mod recorder;
mod region;

use anyhow::{bail, Context, Result};
use cli::Args;
use error::Error;
use region::Region;
use std::fs;
use tempfile::TempDir;

fn validate_output(args: &Args) -> Result<()> {
    // TODO: Improve extension check via file type
    match args.output.extension().and_then(|e| e.to_str()) {
        Some("gif") => Ok(()),
        Some(ext) => bail!("output must be a .gif file, got .{}", ext),
        None => bail!("output must be a .gif file"),
    }
}

fn get_region(args: &Args) -> Result<Region> {
    match &args.geometry {
        Some(g) => Region::from_geometry(g),
        None => region::select_interactive(args.quiet),
    }
}

fn main() -> Result<()> {
    let args = Args::parse_args();

    validate_output(&args)?;
    deps::check()?;

    let region = get_region(&args)?;

    if !args.quiet {
        output::status(&format!("Region: {}", region));
    }

    let temp = TempDir::new().context("failed to create temp directory")?;
    let video = temp.path().join("capture.mp4");

    recorder::record(&region, &video, args.fps, args.duration, args.quiet)?;

    if !video.exists() {
        return Err(Error::EmptyRecording.into());
    }

    converter::to_gif(&video, &args.output, args.fps, args.width, !args.fast, args.quiet)?;

    if args.keep_video {
        let kept = args.output.with_extension("mp4");
        fs::copy(&video, &kept).context("failed to save video")?;
        if !args.quiet {
            output::info(&format!("Video: {}", kept.display()));
        }
    }

    let size = fs::metadata(&args.output)
        .map(|m| m.len())
        .unwrap_or(0);

    if !args.quiet {
        output::summary(&args.output, size);
    }

    Ok(())
}
