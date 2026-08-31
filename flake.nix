{
  description = "Lightweight screen recorder for Wayland compositors that captures regions as GIFs";

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
          ...
        }:
        let
          craneLib = crane.mkLib pkgs;

          # Runtime dependencies that wlgif calls via Command::new
          runtimeDeps = with pkgs; [
            # wlroots backend
            slurp
            wf-recorder
            ffmpeg

            # xdg-portal backend
            xdg-desktop-portal
            pipewire
          ];

          gstPlugins = with pkgs.gst_all_1; [
            gstreamer.out # Provides libgstcoreelements.so
            pkgs.pipewire # Provides pipewiresrc element
            gst-libav
            gst-plugins-base
            gst-plugins-good
            gst-plugins-bad
            gst-plugins-ugly
          ];

          # Common arguments can be set here to avoid repeating them later
          # Note: changes here will rebuild all dependency crates
          commonArgs = {
            # TODO: add version
            src = craneLib.cleanCargoSource ./.;
            strictDeps = true;

            nativeBuildInputs = with pkgs; [
              makeWrapper
              pkg-config
            ];

            buildInputs =
              with pkgs;
              [
                # ashpd/zbus deps
                dbus

                # GStreamer deps
                gst_all_1.gstreamer
                gst_all_1.gst-plugins-base

                # Provides pipewiresrc element
                pipewire
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

              preConfigurePhases = [
                "buildRevision"
              ];

              buildRevision = ''
                export BUILD_REVISION=${inputs.self.shortRev or inputs.self.dirtyShortRev or "unknown"}
              '';

              # Wrap the binary to include runtime dependencies in PATH
              # and set GST_PLUGIN_PATH for GStreamer plugins
              postInstall =
                let
                  gstPluginPath = lib.makeSearchPath "lib/gstreamer-1.0" gstPlugins;
                in
                ''
                  wrapProgram $out/bin/wlgif \
                    --prefix PATH : ${lib.makeBinPath runtimeDeps} \
                    --set GST_PLUGIN_SYSTEM_PATH_1_0 "${gstPluginPath}" \
                    --set GST_PLUGIN_PATH_1_0 "${gstPluginPath}"
                '';

              meta = {
                description = "Lightweight screen recorder for Wayland that captures regions as GIFs";
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

            # Set GST_PLUGIN_PATH for development
            GST_PLUGIN_SYSTEM_PATH_1_0 = lib.makeSearchPath "lib/gstreamer-1.0" gstPlugins;

            # cargo and rustc are provided by default
            packages =
              with pkgs;
              [
                just
                pkg-config
              ]
              ++ runtimeDeps
              ++ gstPlugins;
          };
        };
    };
}
