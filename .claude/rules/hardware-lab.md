# Hardware lab (M17): Raspberry Pi 4 and PC on the bench

Set up with the user on 2026-10-09. Applies to every session or agent that boots EmiBSD on real
hardware. QEMU stays the first place to reproduce a bug; the lab is for what QEMU cannot show.

## What is on the bench

| Thing | Where | Notes |
|---|---|---|
| Raspberry Pi 4B rev 1.4, 8 GB (`boardrev d03114`, MAC `dc:a6:32:f9:12:c0`) | `10.77.0.2` (static, in its EEPROM) | no SD card: it boots from the network only |
| Pi console (PL011, GPIO 14/15, CP2102) | `/dev/cu.usbserial-0001`, 115200 8N1 | read-only for agents unless the task needs input |
| Relay box (Arduino Nano, `tools/lab/relayctl`) | `/dev/cu.usbserial-*` other than `-0001` (CH340, the name follows the USB socket; it was `-10`, then `-210`) | relay 1 = PC RESET_SW, relay 2 = Pi RUN |
| PC: Gigabyte Z390 AORUS ULTRA, i9-9900K, 64 GB | not cabled yet | its serial port needs a COM bracket (to buy); `/dev/cu.PL2303G-USBtoUART*` waits for it |
| Lab network | isolated switch, `10.77.0.0/24`, no internet | the Mac is `10.77.0.1` on `en6` ("USB 10/100 LAN", the WOZ hub) |

## Boot chain of the Pi (every power-on, nothing on the board changes)

```
EEPROM (BOOT_ORDER 0xf12, BOOT_UART=1)  --TFTP-->  /private/tftpboot/rpi4/{start4.elf,fixup4.dat,
  config.txt,bcm2711-rpi-4-b.dtb,overlays/,RPI_EFI.fd}       (Raspberry Pi firmware + pftf EDK2 v1.53)
EDK2 PXE  --DHCP (dnsmasq)-->  NBP arm64/BOOTAA64.EFI  --TFTP-->  EmiBSD efiboot
efiboot   --TFTP-->  /etc/boot.conf (optional), /bsd  -->  kernel
```

Services on the Mac, both started by the user (they need root):
- DHCP: `sudo /opt/homebrew/sbin/dnsmasq -k -C tools/lab/netboot/dnsmasq.conf`, DHCP only, bound
  to `en6`. Never enable its TFTP: dnsmasq 2.93 on macOS now and then sends a routing-socket
  message (`rt_msghdr`, `RTM_MISS`) in place of a TFTP data block, and the Pi's bootloader gives
  up 23 ms after a lost block (capture: `target/lab-tftp.pcap`).
- TFTP: macOS tftpd, launchd job `tools/lab/netboot/org.emibsd.lab.tftpd.plist` (installed in
  `/Library/LaunchDaemons`), chrooted to `/private/tftpboot` (so efiboot's `/bsd` resolves there),
  listening on `10.77.0.1` only. `/private/tftpboot` belongs to `enavarre`: agents write it
  without sudo.
- The Pi's EEPROM settings are in `tools/lab/netboot/rpi4-eeprom-boot.conf`. The bootloader also
  asks TFTP for `rpi4/pieeprom.sig`/`pieeprom.upd` at every boot, so an EEPROM update can be
  served the same way (user's OK first).

## The loop: build, deploy, reset, read

```sh
just build-arm64 "--features multiprocessor"     # the MP kernel; no `qemu` feature on hardware
just efiboot-arm64
OC=$(ls ~/.rustup/toolchains/1.98.1-*/lib/rustlib/*/bin/llvm-objcopy | head -1)
$OC --strip-debug target/aarch64-unknown-none-softfloat/debug/bsd /private/tftpboot/bsd   # 97 MB -> 7.5 MB
cp target/efiboot/arm64/BOOTAA64.EFI /private/tftpboot/arm64/
git rev-parse --short HEAD > /private/tftpboot/arm64/VERSION
```

Reset and capture in ONE command, so the reader is open before the reset and always killed:

```sh
L=<scratchpad>/pi-boot-N.log; : > $L
( stty 115200 raw -echo cs8 -parenb -cstopb clocal -crtscts min 1 time 0 && exec cat ) \
    < /dev/cu.usbserial-0001 > $L 2>&1 & CAT=$!
target/lab-relay/debug/relay -p <nano port> sleep 500 reset pi sleep 60000
kill $CAT; wait $CAT
```

- The relay tool: `cargo build --manifest-path tools/lab/relay/Cargo.toml --target-dir
  target/lab-relay`, then `target/lab-relay/debug/relay -p <port> reset pi` (`status`, `ping`,
  `on N`/`off N`, `pulse N MS`, `sleep MS`; `relay` with no step prints the usage).
- `stty ... min 1 time 0`, never `min 0 time 1`: with VMIN 0 a quiet line makes `read` return 0
  and `cat` exits at once (an empty log that looks like a silent board).
- macOS resets a tty's settings at its last close: set them on the descriptor you read from.
- Strip the log for reading: `tr -d '\r' < $L | LC_ALL=C sed 's/[^[:print:]\t]/./g'`.
- One reader per port. Check with `lsof /dev/cu.usbserial-0001` that none is left over.
- A boot to the kernel takes about 20 s (EEPROM 6 s, firmware 13 s, EDK2 + PXE a few s).

## Where it stands (2026-10-09, HEAD 14920801)

The chain works up to `booting tftp0a:/bsd: 4387948+754696+259048+1067576 [...]`. EDK2 is in
ACPI mode (efiboot lists `FACP CSRT DBG2 GTDT APIC PPTT SPCR SSDT SSDT`, no device tree), the
kernel prints nothing, and the Pi resets about 20 s later by the watchdog (`PM_RSTS 00001020`,
`power-on-reset 0`) and loops. Leads, cheapest first:
1. Give the kernel a device tree: `/private/tftpboot/etc/boot.conf` with `machine dtb
   /rpi4/bcm2711-rpi-4-b.dtb` (efiboot's `dtb` command is ported), or switch EDK2's "System
   Table Selection" to DT (needs the user at an HDMI screen and keyboard). OpenBSD runs the Pi 4
   on its device tree.
2. On ACPI, efiboot builds a tree from SPCR (`efiacpi.rs`) and `pluart_acpi.rs` exists: check the
   console the kernel picks there.
3. Reproduce what can be reproduced on QEMU's `raspi4b` before spending hardware cycles.
The Pi needs no Broadcom driver to print: GIC-400 (`ampintc`), PL011 (`pluart`), the generic
timer and PSCI/spin-table are ported. Its Ethernet (`bse`), PCIe/xhci and SD are not.

## Rules

- Resetting the Pi is free. Resetting the PC (relay 1) loses the user's work: never without
  the user's OK in that session.
- Nothing that needs root is run by an agent: ask the user (dnsmasq restarts, launchctl,
  tcpdump, `networksetup`). Never bind anything to Wi-Fi (`en0`) or serve DHCP outside `en6`.
- To stop the Pi's reboot loop: `sudo launchctl bootout system/org.emibsd.lab.tftpd` (the user);
  it then waits on the network.
- A bug found in a host tool, QEMU, EDK2 or the Raspberry Pi firmware goes to
  `docs/EXTERNAL_BUGS.md` like any other.
- The long-run rules hold (`large-ports.md`): every capture has an end, and a session kills its
  readers before handing back.
