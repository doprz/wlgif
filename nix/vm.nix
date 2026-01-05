{ inputs, ... }:
{
  perSystem =
    {
      self',
      pkgs,
      lib,
      system,
      ...
    }:
    let
      runVM =
        module:
        let
          vm = inputs.nixpkgs.lib.nixosSystem {
            inherit system;
            specialArgs = {
              wlgif = self'.packages.default;
            };
            modules = [
              ./vm/base.nix
              module
            ];
          };
        in
        {
          type = "app";
          program = lib.getExe vm.config.system.build.vm;
        };

      runVMSpice =
        module:
        let
          vm = inputs.nixpkgs.lib.nixosSystem {
            inherit system;
            specialArgs = {
              wlgif = self'.packages.default;
            };
            modules = [
              ./vm/base.nix
              ./vm/spice.nix
              module
            ];
          };

          program = pkgs.writeShellScript "run-vm" ''
            SHARED_DIR=$(pwd)
            export SHARED_DIR

            # Start qemu vm in the background
            ${pkgs.lib.getExe vm.config.system.build.vm} "$@" & VM_PID=$!

            # Wait for SPICE port to be available
            echo "Waiting for SPICE server on port 5930..."
            timeout=30
            while ! ${pkgs.netcat}/bin/nc -z localhost 5930 2>/dev/null; do
              sleep 0.5
              timeout=$((timeout - 1))
              if [ $timeout -le 0 ]; then
                echo "Timeout waiting for SPICE port"
                exit 1
              fi
            done

            echo "Launching virt-viewer..."
            ${pkgs.virt-viewer}/bin/remote-viewer spice://localhost:5930

            # When remote-viewer closes, kill the VM
            kill $VM_PID 2>/dev/null
          '';
        in
        {
          type = "app";
          program = "${program}";
        };
    in
    {
      apps = {
        sway-vm = runVM ./vm/sway.nix;
        sway-vm-spice = runVMSpice ./vm/sway.nix;

        gnome-vm = runVM ./vm/gnome.nix;
        gnome-vm-spice = runVMSpice ./vm/gnome.nix;
      };
    };
}
