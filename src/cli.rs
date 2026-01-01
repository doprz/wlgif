use clap::Parser;
use std::path::PathBuf;

const BUILD_REVISION: &str = env!("BUILD_REVISION");
const PKG_VERSION: &str = env!("CARGO_PKG_VERSION");

pub const CUSTOM_VERSION: &str = const_format::formatcp!("{PKG_VERSION}+{BUILD_REVISION}");

#[derive(Parser, Debug)]
#[command(name = "wlgif")]
#[command(version = CUSTOM_VERSION)]
#[command(about = "Record a region of your Wayland screen as a GIF")]
#[command(after_help = "\x1b[1mExamples:\x1b[0m
  wlgif output.gif                  Select region, record for 5s
  wlgif -d 10 output.gif            Record for 10 seconds
  wlgif -d 0 output.gif             Manual stop with Ctrl+C
  wlgif -g 800x600+100+100 out.gif  Skip selection, use geometry
  wlgif --fps 30 -w 640 output.gif  30fps, scaled to 640px wide

\x1b[1mDependencies:\x1b[0m
  slurp, wf-recorder, ffmpeg")]
pub struct Args {
    /// Output GIF file path
    #[arg(short, long, default_value = "output.gif")]
    pub output: PathBuf,

    /// Recording duration in seconds (0 = manual stop with Ctrl+C)
    #[arg(short, long, default_value = "5", value_name = "SECS")]
    pub duration: f32,

    /// Frames per second (10-30 recommended)
    #[arg(short, long, default_value = "15")]
    pub fps: u32,

    /// Region geometry, skip interactive selection (WxH+X+Y)
    #[arg(short, long, value_name = "WxH+X+Y")]
    pub geometry: Option<String>,

    /// Scale output width in pixels (height auto-calculated)
    #[arg(short, long, value_name = "PX")]
    pub width: Option<u32>,

    /// Skip palette optimization (faster, larger file)
    #[arg(long)]
    pub fast: bool,

    /// Keep intermediate video file
    #[arg(long)]
    pub keep_video: bool,

    /// Suppress status output
    #[arg(short, long)]
    pub quiet: bool,
}

impl Args {
    pub fn parse_args() -> Self {
        Self::parse()
    }
}
