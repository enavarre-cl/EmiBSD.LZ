# Status

Milestone: **M16b, M16c, M16e and M16f done** (USB, network, platform drivers, arm64 platform);
the rest of M16 (M16a, M16d, M16g: storage, console/virtio/legacy, install images) under way,
then M17 (real hardware, vmm). Updated: 2026-10-09.

Done:
- M16c: pcn, ne (ne2000, dp8390, rtl80x9), fxp with loadfirmware(9) and its microcode, dc;
  inphy, lxtphy, dcphy. tulip (dc) and igb (em) pass no traffic, as on OpenBSD 8.0.
- M16b: ehci, uhci, ohci; ums/uwacom over hidms, uhid, ugen and usbdevs(8), cdce, ucom with
  uftdi, uaudio. ehci, and a write through ohci, behave as on OpenBSD 8.0 in QEMU.
- 2026-10-09: `smoke-all`'s outer watchdog (`TIMEOUT`); long runs watched (large-ports.md).

Next:
- M16d, M16a, M16g, in that order (`ci-full` once at M16's close); then M17.
- Left by M14: GPL parts of the sets, lldb, efi(4), base programs (sort, find, ...), `__thrsleep`.

Unsafe (`cargo xtask unsafe-report`): kernel 8313 blocks, 1046 fn, 776 impl, 27 trait, 323 other; tests 816 more.

Blockers:
- amd64 kernel stacks are tight: about 4.9 KB stay free under softraid I/O (M10f measure).
- A `diagnostic` MP kernel panics at boot (`uvm_page_physload: page size not set!`).
- Statistics counters the C bumps unlocked stay `Cell`s (docs/ARCHITECTURE.md, M11e).
- Open flakes: amd64 serial cut at QEMU exit (M16f); host test `no idleproc set on CPU0`, ~1 in
  80 loaded runs; EDK2's UhciDxe ASSERT before the kernel on arm64 (~1 in 5 uhci boots; M16b).

Decisions pending (the user's): the scope section; a SeaBIOS path for vga(4)'s text mode;
Limine's retirement; the PC's CPU for vmm (M17); the Raspberry Pi 4 model; networking in M17;
reporting QEMU's lost `sev` upstream; the restated M16b ehci/ohci and M16c tulip/igb criteria.
