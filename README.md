# wlgif

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

### Usage

```
Usage: wlgif [OPTIONS] <OUTPUT>

Arguments:
  <OUTPUT>  Output GIF file path

Options:
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

## How It Works

1. **Region selection**: `slurp` draws a selection overlay on your compositor
2. **Capture**: `wf-recorder` records using the wlroots screencopy protocol
3. **Palette generation**: ffmpeg analyzes the video to create an optimal 256-color palette
4. **Encoding**: ffmpeg applies Floyd-Steinberg dithering for high-quality output

The two-pass encoding is why `wlgif` produces smaller, better-looking GIFs than naive single-pass conversion.

## Contributing

Contributions are welcome! Whether it's bug reports, feature requests, or code contributions.

### Guidelines

- **Issues:** Found a bug or have a feature idea? [Open an issue](https://github.com/doprz/dipc/issues)
- **Pull Requests:** Fork, create a branch, make your changes, and submit a PR
- **Commit Messages:** This project uses [Conventional Commits](https://www.conventionalcommits.org/)
  - `feat:` for new features
  - `fix:` for bug fixes
  - `docs:` for documentation changes
  - `refactor:` for code refactoring
  - Example: `feat: add GPU support`

## Contributing

Contributions are welcome! This project aims to stay focused on its core purpose while remaining open to improvements whether it's bug reports, feature requests, or code contributions.

### Guidelines

**Before submitting:**
1. **Check existing issues** - Your idea might already be discussed
2. **Open an issue first** - For non-trivial/large changes, discuss the approach before coding
3. **Keep it focused** - New features should enhance the screen-to-GIF workflow, not add unrelated functionality
4. **Maintain composability** - Don't break scripting/piping workflows
5. **Test thoroughly** - Run `cargo test` and test on an actual Wayland session
6. **Format your code** - Run `cargo fmt` before committing
7. **Use Conventional Commits** - This project uses [Conventional Commits](https://www.conventionalcommits.org/)
7. **Write a clear PR description** - Explain *why*, not just *what*

**Good fit for contribution:**
- Recording controls (pause, countdown, visual feedback)
- Performance improvements
- Better error messages
- Shell completions (bash, zsh, fish)
- GUI (feature coming soon)

**Not a good fit:**
- Image editing features (crop, rotate, filters, etc...) - use existing image tools
- Video editing features (trim, concatenate, etc...) - use existing video tools
- Format conversion unrelated to screen capture - use ffmpeg directly

When in doubt, open an issue to discuss!

### Testing

Test on a real Wayland session with your compositor of choice. The core workflow to verify:
1. Region selection works and cancels cleanly
2. Recording captures the correct region
3. GIF output is properly encoded

## License

SPDX-License-Identifier: MIT

Licensed under the MIT License. See [LICENSE](LICENSE) for full details.
