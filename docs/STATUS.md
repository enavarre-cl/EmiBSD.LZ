# Status

Milestone: **M16 done** (QEMU drivers, M16a..M16g); next M17 (real hardware, vmm), with
M16a's deferred storage drivers (mfi, mfii, pcscp, ufshci). Updated: 2026-10-09.

Done:
- M16g: install80.img (both archs) and cd80.iso (amd64) as distrib/ makes them; the ISO 9660
  writer in xtask; the install from the stick with no HTTP server; cd80 boots to the installer.
- M16a: pciide, wdc, wd; mpi; vmwpvs (as OpenBSD 8.0); sdhc and sdmmc; isadma, fdc, fd.
- M16d: pckbc, pckbd, pms; viogpu, viomb, viornd with rnd(4); viocon (feature); lpt; pcppi, spkr; eap.

Next:
- M17 (the user's decisions below first); a 2-CPU `just ci` on main after M16's close.
- Left by M14: GPL parts of the sets, lldb, efi(4), base programs (sort, find, ...), `__thrsleep`.

Unsafe (`cargo xtask unsafe-report`): kernel 8992 blocks, 1143 fn, 822 impl, 31 trait, 340 other; tests 826 more.

Blockers:
- amd64 kernel stacks are tight: about 4.9 KB stay free under softraid I/O (M10f measure).
- A `diagnostic` MP kernel panics at boot (`uvm_page_physload: page size not set!`).
- Statistics counters the C bumps unlocked stay `Cell`s (docs/ARCHITECTURE.md, M11e).
- Open flakes: amd64 serial cut at QEMU exit (M16f); host test `no idleproc set on CPU0`, ~1 in
  80 loaded runs; EDK2's UhciDxe ASSERT on arm64 (~1 in 5 uhci boots; booted once more, EXT-1).
- main's target/comp holds paths of the pre-rename checkout: a seeded `just comp` fails.

Decisions pending (the user's): the scope section; a SeaBIOS path for vga(4)'s text mode;
Limine's retirement; the PC's CPU for vmm (M17); the Raspberry Pi 4 model; networking in M17;
reporting QEMU's `sev` and EXTERNAL_BUGS upstream; the restated M16 criteria (ehci/ohci,
tulip/igb, pvscsi, and mfi/mfii/pcscp/ufshci moved to M17).
