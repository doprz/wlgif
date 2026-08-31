# wlgif

[![crates.io](https://img.shields.io/crates/v/wlgif)](https://crates.io/crates/wlgif)

Lightweight screen recorder for Wayland that captures regions as GIFs

## About

Screen-to-GIF on Wayland has historically been painful. `wlgif` solves this with a simple, seamless workflow: select a region, record, and get a GIF. No configuration, no complexity.

### Backends

`wlgif` supports two recording backends:

| Backend | Compositors | How it works |
|---------|-------------|--------------|
| **portal** | Any (GNOME, KDE, etc...) | XDG Desktop Portal + PipeWire + GStreamer |
| **wlr** | wlroots-based (Sway, Hyprland, etc...) | slurp + wf-recorder + ffmpeg |

The backend is auto-detected, preferring `portal` for broader compatibility. Use `--backend` to override.

## Unix Philosophy

`wlgif` follows core Unix principles:

- **Do one thing well** - Screen region to GIF
- **Compose, don't reinvent** - Leverages battle-tested tools instead of reimplementing capture/encoding

**What this means for features:** We welcome additions that make screen-to-GIF better. What we won't become: an image editor, video editor, or general-purpose media tool.

## Contributing

Contributions are welcome! See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines and [HACKING.md](HACKING.md) for development.


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

#### XDG Desktop Portal Backend

Works on **any** Wayland compositor with portal support.

| Dependency | Purpose |
|------------|---------|
| xdg-desktop-portal | Screen capture API |
| pipewire | Media streaming |
| gstreamer | Video encoding |
| ffmpeg | GIF encoding |

**GStreamer Requirements:** Requires GStreamer >= 1.14 with base plugins and additional plugin packages.

See https://crates.io/crates/gstreamer for more info.

On Debian/Ubuntu:
```sh
apt-get install libgstreamer1.0-dev libgstreamer-plugins-base1.0-dev \
      gstreamer1.0-plugins-base gstreamer1.0-plugins-good \
      gstreamer1.0-plugins-bad gstreamer1.0-plugins-ugly \
      gstreamer1.0-libav libgstrtspserver-1.0-dev libges-1.0-dev
```

On Fedora:
```sh
dnf install gstreamer1-devel gstreamer1-plugins-base-devel \
      gstreamer1-plugins-good gstreamer1-plugins-bad-free \
      gstreamer1-plugin-libav gstreamer1-rtsp-server-devel \
      gst-editing-services-devel

```

Additional Fedora packages (RPMFusion):
```sh
dnf install gstreamer1-plugins-bad-freeworld gstreamer1-plugins-ugly
```

#### wlroots Backend

For wlroots-based compositors (Sway, Hyprland, Niri, dwl, etc.)

| Dependency | Purpose |
|------------|---------|
| slurp | Region selection |
| wf-recorder | Screen capture |
| ffmpeg | GIF encoding |

### Usage

```
Usage: wlgif [OPTIONS]

Options:
  -b, --backend <NAME>      Recording backend (auto-detected if not specified)
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

Examples:
  wlgif                     Select region, record for 5s (default behavior)
  wlgif -d 10               Record for 10 seconds
  wlgif -d 0                Manual stop with Ctrl+C
  wlgif -g 800x600+100+100  Skip selection, use geometry
  wlgif --fps 30 -w 640     30fps, scaled to 640px wide
  wlgif --backend xdg-desktop-portal    Use XDG portal backend (cross-compositor)
  wlgif --backend wlroots               Use wlroots backend (supports slurp region selection)

Dependencies:
  portal:  xdg-desktop-portal, pipewire, gstreamer
  wlr:     slurp, wf-recorder, ffmpeg
```

### Tips

| Goal | Command |
|------|---------|
| Smaller files | `wlgif -w 480 --fps 10` |
| Higher quality | `wlgif --fps 30` |
| Quick capture | `wlgif --fast` |
| Scripting | `wlgif -q -g 800x600+0+0` |

## How It Works

### XDG Desktop Portal Backend

1. **Portal request**: Asks the compositor for screen access via D-Bus
2. **Source selection**: Compositor shows its native picker (window/monitor)
3. **PipeWire capture**: Receives video stream from compositor
4. **GStreamer encoding**: Encodes stream to MP4

### wlroots Backend

1. **Region selection**: `slurp` draws a selection overlay
2. **Capture**: `wf-recorder` records via wlroots screencopy protocol

### Backend Abstraction Layer to Gif

1. **Palette generation**: ffmpeg analyzes video for optimal 256-color palette
2. **Encoding**: ffmpeg applies Floyd-Steinberg dithering -> GIF

The two-pass encoding is why `wlgif` produces smaller, better-looking GIFs than naive single-pass conversion.

## Acknowledgements

- [Ghostty](https://github.com/ghostty-org/ghostty/blob/main/HACKING.md#nix-virtual-machines) for inspiration on NixOS VM testing infrastructure
- [nix.dev](https://nix.dev/tutorials/nixos/nixos-configuration-on-vm.html) for the NixOS VM configuration tutorial

## License

SPDX-License-Identifier: MIT

Licensed under the MIT License. See [LICENSE](LICENSE) for full details.
