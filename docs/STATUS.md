# Status

Milestone: **M16b, M16c, M16d, M16e and M16f done** (USB, network, console/virtio/legacy,
platform drivers, arm64 platform); the rest of M16 (M16a, M16g: storage, install images)
under way, then M17 (real hardware, vmm). Updated: 2026-10-09.

Done:
- M16d: pckbc, pckbd, pms; viogpu, viomb, viornd with rnd(4)'s pool (rnd.c whole), viocon
  (feature); lpt; pcppi, spkr; eap with midi. Every criterion as on OpenBSD 8.0 (probed).
- M16c: pcn, ne (ne2000, dp8390, rtl80x9), fxp with loadfirmware(9) and its microcode, dc;
  inphy, lxtphy, dcphy. tulip (dc) and igb (em) pass no traffic, as on OpenBSD 8.0.

Next:
- M16a, M16g (`ci-full` once at M16's close); then M17.
- Left by M14: GPL parts of the sets, lldb, efi(4), base programs (sort, find, ...), `__thrsleep`.

Unsafe (`cargo xtask unsafe-report`): kernel 8546 blocks, 1081 fn, 794 impl, 28 trait, 336 other; tests 824 more.

Blockers:
- amd64 kernel stacks are tight: about 4.9 KB stay free under softraid I/O (M10f measure).
- A `diagnostic` MP kernel panics at boot (`uvm_page_physload: page size not set!`).
- Statistics counters the C bumps unlocked stay `Cell`s (docs/ARCHITECTURE.md, M11e).
- Open flakes: amd64 serial cut at QEMU exit (M16f); host test `no idleproc set on CPU0`, ~1 in
  80 loaded runs; EDK2's UhciDxe ASSERT before the kernel on arm64 (~1 in 5 uhci boots; M16b; booted once more, EXT-1).

Decisions pending (the user's): the scope section; a SeaBIOS path for vga(4)'s text mode;
Limine's retirement; the PC's CPU for vmm (M17); the Raspberry Pi 4 model; networking in M17;
reporting QEMU's `sev` and EXTERNAL_BUGS upstream (EXT-181/182 reproduced on OpenBSD 8.0);
the restated M16b ehci/ohci and M16c tulip/igb criteria.
