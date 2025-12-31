{ inputs, ... }:
{
  perSystem =
    {
      self',
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
    in
    {
      apps = {
        sway-vm = runVM ./vm/sway.nix;
      };
    };
}
