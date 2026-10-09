# External bugs

Bugs found while porting and testing that belong to someone else: OpenBSD's C at the pin
(`reference/PINNED.md`), QEMU and its bundled firmware (EDK2), and the host tools (macOS,
LLVM). Started by the user on 2026-10-09: "un log de bugs externos, donde también documentes
errores que encuentres en OpenBSD que debamos comunicarles".

This file records them and how the port lives with each; it reports nothing. Reporting
upstream (OpenBSD's bugs@ list or tech@, QEMU's GitLab, EDK2's GitHub) is outward-facing and
done only by the user or with the user's explicit OK, entry by entry.

## Rules

- Every bug found in OpenBSD's C (a slip the port fixes, bounds or works around), in QEMU,
  EDK2 or a host tool gets an entry here in the same commit that meets it (the port's
  `## Deviations` and `ports.toml` note say what the port does; this file says what is wrong
  upstream and what to tell them). Agents list them in their hand-back.
- An entry is evidence, not opinion: the file and line at the pin (`path:line`, 12-hex pin),
  or the QEMU version and command line, what happens, the lines seen, and what was checked.
  `Verified` says how sure we are: `reproduced` (seen run, e.g. on OpenBSD 8.0 through
  `diff-openbsd probe`), `read` (the C was read and the slip is plain), or `claimed` (an
  agent's note, not checked yet).
- Before an OpenBSD entry is proposed for reporting, check it against OpenBSD -current (the
  pin may be behind) and against the man pages and the hardware's specification it relies on.
- `Status`: `open` (not reported), `to report` (checked, the user may send it), `reported
  <link>`, `fixed upstream <commit>`, `not a bug` (kept, with why).
- An entry the port works around in its test environment (a retry, a restated criterion)
  names that workaround, so it is removed when upstream fixes the bug.

## Summary

| Id | Where | What | Verified | Port's handling | Status |
|---|---|---|---|---|---|
| EXT-1 | EDK2 (QEMU 11.1.2's `edk2-aarch64-code.fd`) | `UhciDxe` ASSERT before any OS loads, ~1 in 5 arm64 boots with a UHCI controller | reproduced | `xtask smoke` boots once more | open |
| EXT-2 | macOS (host) | an `xtask` process freezes before `main` (`_dyld_start`) | reproduced, cause unknown | `smoke-all`'s watchdog | open, investigating |
| EXT-3 | QEMU TCG (arm64, MTTCG, 2+ vCPUs) | a `sev` can be lost, so a `wfe` never wakes | reproduced | generic timer event stream on (deviation, M11e) | open (the user's decision pending) |
| EXT-4 | OpenBSD `sys/dev/pci/ehci_pci.c:128` | 16-bit write to the 32-bit EHCI `USBINTR` register; OpenBSD 8.0 panics on arm64 QEMU | reproduced | faithful; criterion restated (M16b) | open |
| EXT-5 | QEMU I/O APIC with OpenBSD's cold routing (amd64) | ehci's INTx raised while its pin is masked and edge-triggered is dropped, the controller never interrupts | reproduced | faithful; criterion restated (M16b) | open, to analyse |
| EXT-6 | QEMU `pci-ohci` or OpenBSD ohci(4) | a write to a stick through ohci halts the controller | reproduced | faithful; criterion restated (M16b) | open, to analyse |
| EXT-7 | QEMU `tulip` or OpenBSD dc(4) | `failed to force tx to idle state`, `watchdog timeout`, no traffic | reproduced | faithful; criterion restated (M16c) | open, to analyse |
| EXT-8 | QEMU `igb` or OpenBSD em(4) | the 82576 attaches and is active but receives nothing, both archs | reproduced | faithful; criterion restated (M16c) | open, to analyse |
| EXT-9.. | OpenBSD C | slips the port fixed or bounded while porting (see "OpenBSD C slips") | claimed, being checked | each file's `## Deviations` | open |

## Entries

### EXT-1: EDK2 UhciDxe ASSERT on arm64 (QEMU firmware)

- Where: the EDK2 build QEMU 11.1.2 ships for `virt` (`edk2-aarch64-code.fd`),
  `MdeModulePkg/Bus/Pci/UhciDxe/UhciSched.c` line 974.
- What: with `piix3-usb-uhci` on the PCI bus, about one boot in five stops in the firmware,
  before any OS loader runs: `ASSERT [UhciDxe] /home/kraxel/projects/qemu/roms/edk2/
  MdeModulePkg/Bus/Pci/UhciDxe/UhciSched.c(974): CR has Bad Signature`. A `CR()` signature
  check failing means the driver followed a pointer to a structure that is not the one it
  expects (freed or never initialised): a firmware bug, independent of the OS.
- Seen: `smoke-uhci`'s arm64 boot (M16b, 2026-10-08; again in `just ci` on 2026-10-09).
- Port's handling: `xtask smoke` boots once more when a failed boot's transcript has that
  exact line and no kernel line yet (`boot::FIRMWARE_FLAKES`, the user's decision of
  2026-10-09); the log says `retrying once after a known firmware bug (EXT-1 ...)` and
  `smoke-all` counts such retries. A second failure of any kind fails the smoke.
- To report: QEMU's EDK2 build (or EDK2's `UhciDxe`), with the QEMU command line of
  `smoke-uhci`'s arm64 boot and the frequency. Remove the retry once fixed.

### EXT-2: xtask frozen before main on macOS (host)

- What: an `xtask` process that `cargo` has just started prints nothing and never ends;
  sampled once in `_dyld_start`, macOS's dynamic loader, before `main`. Any limit inside it
  is never reached.
- Seen: `smoke-softraid` in a `ci-full` run (2026-10-09, 2 h 30 min until noticed);
  `smoke-pcn` in `just ci` (2026-10-09 13:35, stopped by the watchdog after 10 minutes). Two
  in some 300 recipe runs.
- Cause: unknown. Candidates: the loader waiting on the system's code-signing or malware
  assessment of a freshly linked binary while many processes start at once.
- Port's handling: `smoke-all`'s outer watchdog (`TIMEOUT`); since 3bf642a1 it writes the
  stuck processes' state and wait channel to the log, which is what the next case needs.
- To report: Apple (Feedback Assistant), once the listing shows where it waits.

### EXT-3: QEMU TCG loses `sev` on arm64

- What: under MTTCG with two or more vCPUs, QEMU's `sev` helper kicks a halted vCPU without
  the global lock, and the event can be lost; a CPU in `wfe` waiting for it never wakes. An
  application processor waiting for `CPUF_GO`, its GIC CPU interface not enabled yet, then
  hangs the boot.
- Seen: M11e, the arm64 MP boot.
- Port's handling: arm64 turns on the generic timer's event stream (`CNTKCTL_EL1.EVNTEN`, a
  wake-up about every 130 us) on every CPU, which OpenBSD does not (docs/ARCHITECTURE.md,
  "Deviations"); Linux keeps it on for the same reason.
- To report: QEMU (target/arm, the `sev`/`wfe` TCG helpers). The user's decision, pending
  since M11 (docs/STATUS.md).

### EXT-4: ehci_pci writes EHCI_USBINTR with a 16-bit access (OpenBSD)

- Where: `sys/dev/pci/ehci_pci.c:128` at 3ce1f3f79392: `EOWRITE2(&sc->sc, EHCI_USBINTR, 0);`.
  `EHCI_USBINTR` (`sys/dev/usb/ehcireg.h:124`, offset 0x08) is a 32-bit operational register;
  the EHCI specification (section 2.3) asks for 32-bit accesses to them.
- What: QEMU's EHCI refuses the 2-byte access; on arm64 `virt` that is a synchronous
  external abort, so OpenBSD 8.0 panics at attach (the snapshot `diff-openbsd` uses, probe on
  the same QEMU setup, M16b). amd64's port I/O takes it.
- Verified: reproduced on OpenBSD 8.0 and on EmiBSD (the port keeps the 16-bit write).
- Port's handling: faithful; M16b's criterion for ehci was restated as "behaves as OpenBSD
  8.0" (docs/ROADMAP.md, M16b row; docs/ARCHITECTURE.md, ehci(4)).
- To report (after checking -current): OpenBSD bugs@, with the line, the spec reference, the
  arm64 panic and the one-line fix (`EOWRITE4`).

### EXT-5: ehci's INTx lost while cold on amd64 (QEMU I/O APIC and OpenBSD's routing)

- What: `ehci_init` raises INTx while the I/O APIC pin is still masked and edge-triggered
  (`ioapic_addroute` leaves pins to `ioapic_enable` while cold, as in the C), and QEMU's I/O
  APIC drops edges that arrive masked, so the level that never falls is never delivered.
  OpenBSD 8.0 prints `uhub0: device problem, disabling port 1` and the stick is not mounted.
- Verified: reproduced on OpenBSD 8.0 (M16b probe) and EmiBSD.
- To analyse: whether real I/O APICs latch it (a QEMU difference) or OpenBSD should not
  unmask a level source as an edge; then report to the side that is wrong.

### EXT-6: a write through ohci halts the controller (QEMU pci-ohci or ohci(4))

- What: reading the stick through `pci-ohci` works; the first write ends with
  `ohci0: unrecoverable error, controller halted`, on OpenBSD 8.0 as on EmiBSD (M16b).
- To analyse: which side breaks the OHCI transfer descriptors' contract.

### EXT-7: tulip passes no traffic (QEMU tulip or dc(4))

- What: OpenBSD 8.0 on QEMU's `tulip` (DEC 21143): `dc0 at pci0 dev 2 function 0
  "DEC 21142/3" rev 0x00 ...`, `lxtphy0 at dc0 phy 1: LXT970, rev. 0`, `dc0: failed to force
  tx to idle state` (twice), `dc0: watchdog timeout`, `3 packets transmitted, 0 packets
  received, 100.0% packet loss` (M16c probe). EmiBSD's dc(4) does the same (`smoke-dc`).
- To analyse: QEMU's tulip model (the transmit state machine) against the 21143 manual.

### EXT-8: igb receives nothing (QEMU igb or em(4))

- What: OpenBSD 8.0 on QEMU's `igb` (82576): `em0 at pci0 dev 2 function 0 "Intel 82576"
  rev 0x01: msi, address 52:54:00:12:34:56` (`dev 1` on arm64), status active,
  `3 packets transmitted, 0 packets received, 100.0% packet loss`, both archs (M16c probe).
  EmiBSD's em(4) does the same (`smoke-igb`). em(4) drives the 82576 with legacy
  descriptors.
- To analyse: whether QEMU's igb implements the legacy descriptor format the 82576 still
  supports.

## OpenBSD C slips

Slips in OpenBSD's C that ports fixed or bounded, as their `## Deviations` and `ports.toml`
notes record (an out-of-bounds read or write, a division by zero, a NULL dereference, a
wrong size). Being collected and checked against the C from the notes of every ported file;
each becomes an entry above (EXT-9 on) once verified.
