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

## Nix Flake Overlay
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

## NixOS Virtual Machines

NixOS VM definitions are provided for testing `wlgif` against different Wayland compositors in isolation.
`wlgif` is pre-installed and ready to use and test.

### Available VMs

| Command | Compositor |
|---------|------------|
| `nix run .#sway-vm` | Sway |
| `nix run .#hyprland-vm` | Hyprland |

### VM Configuration

- 2 CPU cores, 2GB RAM, 4GB disk
- Changes are saved to `wlgif-test-vm.qcow2`