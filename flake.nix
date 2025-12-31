{
  description = "Lightweight screen recorder for wlroots-based Wayland compositors that captures regions as GIFs";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-parts.url = "github:hercules-ci/flake-parts";
    flake-utils.url = "github:numtide/flake-utils";
    crane.url = "github:ipetkov/crane";
  };

  outputs =
    inputs@{
      flake-parts,
      crane,
      ...
    }:
    flake-parts.lib.mkFlake { inherit inputs; } {
      systems = inputs.nixpkgs.lib.systems.flakeExposed;

      imports = [ ./nix/vm.nix ];

      perSystem =
        {
          self',
          pkgs,
          lib,
          system,
          ...
        }:
        let
          craneLib = crane.mkLib pkgs;

          # Runtime dependencies that wlgif calls via Command::new
          runtimeDeps = with pkgs; [
            slurp
            wf-recorder
            ffmpeg
          ];

          # Common arguments can be set here to avoid repeating them later
          # Note: changes here will rebuild all dependency crates
          commonArgs = {
            src = craneLib.cleanCargoSource ./.;
            strictDeps = true;

            nativeBuildInputs = [
              pkgs.makeWrapper
            ];

            buildInputs = [
            ]
            ++ pkgs.lib.optionals pkgs.stdenv.isDarwin [
              # Additional darwin specific inputs can be set here
              pkgs.libiconv
            ];
          };

          wlgif = craneLib.buildPackage (
            commonArgs
            // {
              cargoArtifacts = craneLib.buildDepsOnly commonArgs;

              # Additional environment variables or build phases/hooks can be set
              # here *without* rebuilding all dependency crates
              # MY_CUSTOM_VAR = "some value";

              # Wrap the binary to include runtime dependencies in PATH
              postInstall = ''
                wrapProgram $out/bin/wlgif \
                  --prefix PATH : ${pkgs.lib.makeBinPath runtimeDeps}
              '';

              meta = {
                description = "Lightweight screen recorder for wlroots-based Wayland compositors that captures regions as GIFs";
                homepage = "https://github.com/doprz/wlgif";
                license = pkgs.lib.licenses.mit;
                maintainers = pkgs.lib.maintainers.doprz;
                platforms = pkgs.lib.platforms.unix;
                mainProgram = "wlgif";
              };
            }
          );
        in
        {
          checks = {
            inherit wlgif;
          };

          packages.default = wlgif;

          apps.default = {
            type = "app";
            program = lib.getExe wlgif;
          };

          devShells.default = craneLib.devShell {
            name = "wlgif-dev";
            checks = self'.checks;

            # Additional dev-shell environment variables can be set directly
            # MY_CUSTOM_DEVELOPMENT_VAR = "something else";

            # cargo and rustc are provided by default
            packages = [
            ]
            ++ runtimeDeps;
          };
        };
    };
}
