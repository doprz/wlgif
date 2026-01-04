# Hacking on `wlgif`

Development guide for contributors.

## Development

### Using Cargo
```sh
# Build (debug)
cargo build

# Build (release)
cargo build --release

# Run with arguments
cargo run -- output.gif
cargo run -- -d 10 --fps 30

# Run tests
cargo test

# Format code
cargo fmt

# Lint
cargo clippy

# Check without building
cargo check
```

#### Environment Variables
```sh
# Enable debug logging
RUST_LOG=debug cargo run

# Backtrace on panic
RUST_BACKTRACE=1 cargo run

# GStreamer debug output (useful for xdg desktop portal backend issues)
GST_DEBUG=3 cargo run -- --backend xdg-portal output.gif
```

### Using Nix
```sh
# Enter dev shell with all dependencies
nix develop

# Or use direnv for auto dev shell
direnv allow

# Build
nix build

# Run
nix run
```

## Testing Backends

### XDG Desktop Portal Backend

The portal backend requires:
- A running D-Bus session
- `xdg-desktop-portal` and a compositor-specific implementation
- PipeWire
- GStreamer with `pipewiresrc`, `x264enc`, `filesink` elements

Verify GStreamer elements are available:

```sh
gst-inspect-1.0 pipewiresrc
gst-inspect-1.0 x264enc
gst-inspect-1.0 filesink
```

### wlroots Backend

The wlr backend requires a wlroots-based compositor and:

```sh
which slurp wf-recorder ffmpeg
```

## Nix Flake Integration

### Overlay

```nix
{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    wlgif.url = "github:doprz/wlgif";
  };

  outputs = { nixpkgs, wlgif, ... }: {
    nixosConfigurations.host = nixpkgs.lib.nixosSystem {
      system = "x86_64-linux";
      modules = [{
        nixpkgs.overlays = [ wlgif.overlays.default ];
        environment.systemPackages = [ pkgs.wlgif ];
      }];
    };
  };
}
```

### Direct Package

```nix
{
  inputs.wlgif.url = "github:doprz/wlgif";

  # In your configuration:
  environment.systemPackages = [ inputs.wlgif.packages.${system}.default ];
}
```

## NixOS Virtual Machines

NixOS VM definitions are provided for testing `wlgif` against different Wayland compositors in isolation.
`wlgif` is pre-installed and ready to use and test. 

### VM Configuration

- 4 CPU cores, 4GB RAM, 4GB disk (`wlgif-test-vm-qcow2`)
- PipeWire enabled
- Test user: `wlgif-dev` (empty password)
- SPICE support + `remote-viewer` (auto connects)
- `qemu-guest-agent` and `spice-vdagentd` services enabled

> [!WARNING]
> The `wlgif` source directory is auto-mounted to `/tmp/shared/` and may be writable by the VM's user

### Available VMs

| Command | Compositor |
|---------|------------|
| `nix run .#sway-vm` | Sway |
| `nix run .#gnome-vm` | Gnome (Wayland) |
| `nix run .#gnome-vm-spice` | Gnome (Wayland) with SPICE + remote-viewer |

Check `nix/vm.nix` for the entire list of available VMs.

## Debugging

### Portal Backend Issues

```sh
# Check portal is running
systemctl --user status xdg-desktop-portal

# Monitor D-Bus traffic
dbus-monitor --session "interface='org.freedesktop.portal.ScreenCast'"

# GStreamer element debugging
GST_DEBUG=3 wlgif --backend portal test.gif 2>&1 | head -100
```