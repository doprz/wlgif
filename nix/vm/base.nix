{ pkgs, wlgif, ... }:
{
  boot.loader.systemd-boot.enable = true;
  boot.loader.efi.canTouchEfiVariables = true;

  networking.hostName = "wlgif-test-vm";
  networking.networkmanager.enable = true;

  virtualisation.vmVariant = {
    virtualisation.cores = 4;
    virtualisation.memorySize = 4096;
    virtualisation.diskSize = 4096;
    virtualisation.graphics = true;
  };

  system.stateVersion = "25.11";
  documentation.nixos.enable = false;

  nix = {
    settings.trusted-users = [
      "root"
      "wlgif-dev"
    ];
    extraOptions = "experimental-features = nix-command flakes";
  };

  environment.systemPackages = with pkgs; [
    vim
    git
    tmux
    ghostty
    wlgif
  ];

  services.xserver.enable = true;
  services.dbus.enable = true;
  services.libinput.enable = true;
  security.polkit.enable = true;

  # rtkit (optional, recommended) allows Pipewire to use the realtime scheduler for increased performance.
  security.rtkit.enable = true;
  services.pipewire = {
    enable = true;
    alsa.enable = true;
    alsa.support32Bit = true;
    pulse.enable = true;
  };

  programs.fish.enable = true;

  users.users.wlgif-dev = {
    description = "wlgif dev user";
    isNormalUser = true;
    extraGroups = [
      "wheel"
      "networkmanager"
    ];
    hashedPassword = "";
    shell = pkgs.fish;
  };
}
