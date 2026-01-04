{ ... }:
{
  services.qemuGuest.enable = true;
  services.spice-vdagentd.enable = true;

  services.xserver.videoDrivers = [ "qxl" ];

  virtualisation.vmVariant = {
    virtualisation.qemu.options = [
      "-vga qxl"
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
}
