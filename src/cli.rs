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
  wlgif                     Select region, record for 5s (default behavior)
  wlgif -d 10               Record for 10 seconds
  wlgif -d 0                Manual stop with Ctrl+C
  wlgif -g 800x600+100+100  Skip selection, use geometry
  wlgif --fps 30 -w 640     30fps, scaled to 640px wide
  wlgif --backend xdg-desktop-portal    Use XDG portal backend (cross-compositor)
  wlgif --backend wlroots               Use wlroots backend (supports slurp region selection)

\x1b[1mDependencies:\x1b[0m
  portal:  xdg-desktop-portal, pipewire, gstreamer
  wlr:     slurp, wf-recorder, ffmpeg")]
pub struct Args {
    /// Recording backend (auto-detected if not specified)
    #[arg(short, long, value_name = "NAME")]
    pub backend: Option<String>,

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
