{ pkgs, wlgif, ... }:
{
  boot.loader.systemd-boot.enable = true;
  boot.loader.efi.canTouchEfiVariables = true;

  networking.hostName = "wlgif-test-vm";
  networking.networkmanager.enable = true;

  services.qemuGuest.enable = true;
  services.spice-vdagentd.enable = true;

  virtualisation.vmVariant = {
    virtualisation.cores = 4;
    virtualisation.memorySize = 4096;
    virtualisation.diskSize = 4096;
    virtualisation.graphics = true;

    # Use SPICE with QXL for resizable display
    virtualisation.qemu.options = [
      "-vga qxl"
      # "-vga none"
      # "-device virtio-vga"
      "-display none"
      "-device virtio-serial-pci"
      # SPICE agent (for resize/clipboard)
      "-chardev spicevmc,id=vdagent,debug=0,name=vdagent"
      "-device virtserialport,chardev=vdagent,name=com.redhat.spice.0"
      # QEMU guest agent (for host communication)
      "-chardev socket,path=/tmp/qga.sock,server=on,wait=off,id=qga0"
      "-device virtserialport,chardev=qga0,name=org.qemu.guest_agent.0"
      # SPICE server for remote-viewer connection
      "-spice port=5930,disable-ticketing=on"
    ];
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
