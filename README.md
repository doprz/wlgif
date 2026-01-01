# wlgif

[![crates.io](https://img.shields.io/crates/v/wlgif)](https://crates.io/crates/wlgif)

Lightweight screen recorder for wlroots-based Wayland compositors that captures regions as GIFs

## About

Screen-to-GIF on Wayland has historically been painful. `wlgif` solves this by composing three excellent tools into a single, seamless workflow:

1. **slurp** -> Interactive region selection
2. **wf-recorder** -> Native Wayland capture
3. **ffmpeg** -> Optimized GIF encoding

The result: select a region, record, and get a GIF. No configuration, no complexity.

## Unix Philosophy

`wlgif` follows core Unix principles:

- **Do one thing well** - Screen region to GIF. New features should enhance this core workflow
- **Compose, don't reinvent** - Leverages battle-tested tools instead of reimplementing capture/encoding

**What this means for features:** We welcome additions like recording controls, format options, or even a GUI (feature coming soon) that makes screen-to-GIF better. What we won't become: an image editor, video editor, or general-purpose media tool. Those already exist and do their jobs well.

## Installation

### Cargo

```sh
cargo install wlgif
```

### Nix

`wlgif` is available as a nix flake via GitHub

```sh
# From GitHub
nix run github:doprz/wlgif
```

### From Source

To build and install from source, first checkout the tag or branch you want to install, then run

```sh
cargo install --path .
```

This will build and install `wlgif` in your `~/.cargo/bin`. Make sure that `~/.cargo/bin` is in your `$PATH` variable.

### Dependencies

`wlgif` requires these tools in your PATH:

| Tool | Purpose | Install |
|------|---------|---------|
| slurp | Region selection | `pacman -S slurp` / `apt install slurp` / `dnf install slurp` |
| wf-recorder | Screen capture | `pacman -S wf-recorder` / `apt install wf-recorder` / `dnf install wf-recorder` |
| ffmpeg | GIF encoding | `pacman -S ffmpeg` / `apt install ffmpeg` / `dnf install ffmpeg` |

**Compositor support:** Any wlroots-based compositor (Sway, Hyprland, Niri, dwl, etc...)

## How It Works

1. **Region selection**: `slurp` draws a selection overlay on your compositor
2. **Capture**: `wf-recorder` records using the wlroots screencopy protocol
3. **Palette generation**: ffmpeg analyzes the video to create an optimal 256-color palette
4. **Encoding**: ffmpeg applies Floyd-Steinberg dithering for high-quality output

The two-pass encoding is why `wlgif` produces smaller, better-looking GIFs than naive single-pass conversion.

## Contributing

Contributions are welcome! See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines and [HACKING.md](HACKING.md) for development.

### Usage

```
Usage: wlgif [OPTIONS]

Options:
  -o, --output <OUTPUT>     Output GIF file path [default: output.gif]
  -d, --duration <SECS>     Recording duration in seconds (0 = manual stop with Ctrl+C) [default: 5]
  -f, --fps <FPS>           Frames per second (10-30 recommended) [default: 15]
  -g, --geometry <WxH+X+Y>  Region geometry, skip interactive selection (WxH+X+Y)
  -w, --width <PX>          Scale output width in pixels (height auto-calculated)
      --fast                Skip palette optimization (faster, larger file)
      --keep-video          Keep intermediate video file
  -q, --quiet               Suppress status output
  -h, --help                Print help
  -V, --version             Print version
```

## Examples

```bash
# Basic: select region interactively, record 5 seconds
wlgif output.gif

# Record for 10 seconds
wlgif -d 10 output.gif

# Record until Ctrl+C
wlgif -d 0 output.gif

# Skip interactive selection, specify geometry directly
wlgif -g 800x600+100+100 output.gif

# Higher framerate (smoother, larger file)
wlgif --fps 30 output.gif

# Scale output width (maintains aspect ratio)
wlgif -w 640 output.gif

# Fast mode: skip optimization (quicker, larger file)
wlgif --fast output.gif

# Quiet mode: no status output
wlgif -q output.gif

# Keep the intermediate video file
wlgif --keep-video output.gif
```


### Tips

| Goal | Command |
|------|---------|
| Smaller files | `wlgif -w 480 --fps 10 out.gif` |
| Higher quality | `wlgif --fps 30 out.gif` |
| Quick capture | `wlgif --fast out.gif` |
| Scripting | `wlgif -q -g 800x600+0+0 out.gif` |

## License

SPDX-License-Identifier: MIT

Licensed under the MIT License. See [LICENSE](LICENSE) for full details.
