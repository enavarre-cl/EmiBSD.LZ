# Scope and stubs

The kernel is ported incrementally; most of OpenBSD is not here yet. These rules keep that honest.

- Never `todo!()`, `unimplemented!()` or an empty body standing in for real work. Both lints are
  denied workspace-wide; do not `#[allow]` them.
- When a ported file calls into an unported subsystem, make the gap explicit and visible, either:
  - `return Err(unported!("uvm_map"))` (`sys/kern/unported.rs`), a macro that prints the gap once
    per site and yields `Errno::ENOSYS`; or
  - gate the whole path behind a cargo feature mirroring the OpenBSD `option(4)`
    (`diagnostic`, `multiprocessor`, ...), documented in `sys/Cargo.toml`.
- Every stub is recorded twice: in the file's `//! ## Deviations` list and in the entry's `notes`
  in `ports.toml`.
- Never delete a code path because "we don't need it yet". Port it or stub it visibly.
- Drivers for hardware QEMU does not expose: `status = "skipped"`, `notes = "deferred-driver: ..."`.
- A QEMU device on which OpenBSD 8.0 itself fails (the user's rule of 2026-10-09): probe it
  first (`cargo xtask diff-openbsd --arch A probe ...` with the same QEMU options); if OpenBSD
  fails there too, its driver is not ported in a QEMU milestone. The failure goes into
  `docs/EXTERNAL_BUGS.md` (the probe lines, the QEMU options, QEMU's model or the driver "to
  analyse"), its rows stay `todo` with `notes = "M17: deferred ..."`, and the device moves to
  M17's row (real hardware, where it can be tested). A faithful port could only reproduce the
  failure, and fixing it would need a behavioural deviation or a change to QEMU. A cheap fix on
  our side that stays faithful (an xtask or QEMU option, as the floppy in drive B) is still taken.
- Licences (the user's rule of 2026-10-04, replacing the list of per-licence decisions): any
  licence or notice on a file in the pinned OpenBSD tree (`reference/openbsd-src`) is accepted
  without asking, for kernel ports and for the compiled userland alike; OpenBSD already accepted
  it into its tree. That covers ISC, BSD (2, 3, 4 clauses), MIT, Mach, beerware, Dyson, zlib,
  public domain, the HPND-style notices (M.I.T., Carnegie Mellon, OSF, TRW, the IPsec authors),
  LibreSSL's OpenSSL/SSLeay, Apache-2.0 WITH LLVM-exception, files with no licence text, and any
  other. The conditions:
  - The original notice is kept whole between `/* <LICENSES> */` and `/* </LICENSES> */`; a file
    without licence text keeps its copyright lines as they are. Never shorten, reword or
    relicense it. A zlib port says it is an altered version (zlib clause 2).
  - `ports.toml` `notes` names the licence when it is not ISC, BSD or MIT
    (e.g. `license: OSF notice, kept whole; ...`).
  - `LICENSE` lists the licence families present; a new family is added there in the same commit.
- Authorship (the user's rule of 2026-10-09): Emilio Navarrete Lineros is the author of the
  Rust translation and of every Rust file of this repository. Every `.rs` under `sys/` and
  `tools/` carries, inside its `/* <LICENSES> */` zone, this ISC block, whole (the `init/` files
  carry it as their first comment):
  ```
  /*
   * Copyright (c) 2026 Emilio Navarrete Lineros <enavarre@outlook.com>
   *
   * Permission to use, copy, modify, and distribute this software for any
   * purpose with or without fee is hereby granted, provided that the above
   * copyright notice and this permission notice appear in all copies.
   *
   * THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES
   * WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
   * MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR
   * ANY SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
   * WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN
   * ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF
   * OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.
   */
  ```
  - The author's block is the first block of the zone: the latest change of a file goes
    first, as every notice added to an OpenBSD file does. In a port of a C file with licence
    text the original block(s) follow it after one blank line, and are never touched: never
    shortened, reworded, reordered or replaced.
  - Any other file (`license = "none"`, an `[[extra]]`, a crate root, `mod.rs`, `sys/machine/`,
    `sys/stand/`, `sys/arch/host/`, `tools/`, a generated file): the author's block is the only
    block of the zone.
  - New files carry it from their first commit, generated files from their generator
    (`gen-syscalls`), and every agent prompt that writes Rust says so. The canonical text is
    `AUTHOR_BLOCK` in `tools/xtask/src/layout.rs`; `cargo xtask ports check` requires it.
  - Derivative works and migrations: any repository, branch, rewrite, refactor, translation or
    tool that takes code from this one (a module moved, redesigned, split, merged or
    regenerated) keeps the block, with this name, in every file that derives from it. Never
    remove it, never move it below the original notices, never replace it with another name
    (an agent's, a project's, a company's), and never present the work as someone else's.
    Another contributor's notice is added only with the user's OK.
- Code that does not come from the pinned OpenBSD tree (ZFS, XFS, external libraries, ...) is
  outside that rule: its licence is decided by the user when the milestone that brings it is
  proposed. Until then: stop, tell the user, do not port it (`status = "skipped"`,
  `notes = "license: <which>"`). A translation is still a derivative work, so rewriting does not
  escape a licence.
- Code replaced by a project-level decision (bootloader, build system, `config(8)`):
  `status = "skipped"`, `notes = "replaced-by-<what>: ..."`, and the decision is in `docs/ARCHITECTURE.md`.
- Scope changes (dropping an arch, skipping a subsystem, changing the boot protocol) are the
  user's decision. Propose; do not decide.
