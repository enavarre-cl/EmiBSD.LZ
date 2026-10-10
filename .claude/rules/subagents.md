# Subagents: the common contract

The project's subagents are defined in `.claude/agents/*.md` (the user's decision of
2026-10-09, replacing the ad-hoc `general-purpose` prompts). Every definition there points
here; this file is the part they share. The rules it summarises stay authoritative in their
own files (`large-ports.md`, `testing.md`, `git-commits.md`, `scope-and-stubs.md`); when they
change, this file follows.

## Roles

| Agent | Model | Worktree | Memory | Writes | Use it for |
|---|---|---|---|---|---|
| `milestone-coordinator` | opus | yes | yes | branch | a (sub-)milestone: probe, split, launch, integrate, close CI and docs |
| `porter` | opus | yes | yes | branch | a delicate port: uvm, traps, pmap, MP, drivers with DMA/interrupts, any `.c` > 3,000 lines |
| `porter-mechanical` | sonnet | yes | yes | branch | headers of constants/structs, small leaf functions, tables, ports.toml rows |
| `integrator` | opus | yes | yes | branch | rebase/merge agent branches, shared-table conflicts, finish a stopped agent's work |
| `debugger` | opus | yes | yes | branch | root cause of a failing or flaky smoke/test, fixed in code or in the smoke's sequencing |
| `reviewer` | opus | no | no | nothing | read-only review of a branch: faithfulness to the C, unsafe, rules, security |
| `external-bugs` | opus | yes | yes | docs only | verify OpenBSD C slips the ports found, write `docs/EXTERNAL_BUGS.md` entries |
| `openbsd-probe` | sonnet | no | no | nothing | boot real OpenBSD 8.0 on a QEMU setup and report what it does |
| `image-worker` | sonnet | no | no | scratchpad | view, crop, resize images so the coordinator's context stays small |

Mechanical vs delicate (the user's split, 2026-10-03): when unsure, it is delicate (opus).
At most 4 agents run at once; a milestone coordinator runs at most 2 of its own.

## Memory (`memory: project`)

An agent with memory keeps `.claude/agent-memory/<agent>/MEMORY.md`: lessons that outlive one
run (an idiom that took two attempts, a known flake, a conflict pattern), one line each, in
English, appended at the end. It is committed with the tree, so a worktree agent's lines travel
on its branch and the `integrator` merges them (both sides kept, duplicates dropped). Task
state (branches, hashes, what is left) goes in `HANDOFF.md`, never in memory. The `reviewer`
has none on purpose: it reads every change with fresh eyes. `agent-memory-local/` is not used.

## Workflows (`.claude/workflows/`)

`/port <files | milestone>` (`port.js`) plans clusters, runs one `porter`/`porter-mechanical`
per cluster in its own worktree (at most 4 at once), has a `reviewer` check each branch with
one fix round, and ends with an `integrator` that merges the approved branches and runs
`just ci`. The result is a branch for the main session to fast-forward; the workflow never
touches `main`. Running a workflow is the user's call (`/port`), never the model's. Inside a
workflow the lock is `/tmp/emibsd/ci.lock` and each cluster's notes live in
`/tmp/emibsd/port/<cluster>/` (the scripts cannot see a session's scratchpad).

## What the launching prompt must give

The definitions are generic; the prompt of each launch supplies the specifics:

1. the task and its scope (ports.toml rows, ROADMAP row, files);
2. the branch to merge first (the coordinator's), or the base commit;
3. the scratchpad directory for `HANDOFF.md` and logs;
4. the machine lock path (`ci.lock` in the launching session's scratchpad), or "no lock";
5. what else runs on the machine at the same time;
6. any authorisation the user gave for this launch, quoted.

A prompt that misses one of these gets a question back, not a guess.

## Setup in a worktree

- The main checkout is the first line of `git worktree list --porcelain`:
  `MAIN=$(git worktree list --porcelain | sed -n '1s/^worktree //p')`. The gitignored
  reference tree lives only there: `ln -s "$MAIN/reference/openbsd-src" reference/openbsd-src`
  if missing. Never commit it, never `git add -A` / `git add .`; add paths by name. Unlink it
  before the worktree is removed.
- `PATH=/opt/homebrew/opt/rustup/bin:$PATH` before `cargo`/`just`. Install nothing.
- `diff-openbsd` needs the snapshot: `cp -Rc "$MAIN/target/openbsd" target/openbsd`. Never
  re-download it, never delete the main checkout's copy.
- A worktree's first `just userland` / `just comp` is seeded from the main checkout
  (`userland/seed.rs`); run it once before any smoke.
- Read CLAUDE.md and every file of `.claude/rules/` before writing anything.

## While working

- Author block: every `.rs` opens `/* <LICENSES> */` with the author's ISC block
  (`AUTHOR_BLOCK` in `tools/xtask/src/layout.rs`), then one blank line and the original
  notice(s) untouched (`scope-and-stubs.md`, authorship).
- Commit early: each piece that builds and passes clippy/fmt is committed at once. Trailers:
  `Upstream: <c path>@<12-hex>` per ported C file, then the session's `Co-Authored-By:` line.
  Never push, never amend a published commit, never force anything, never touch `main`.
- Machine lock: while the lock directory exists and is not yours, run no smokes, no
  `just test`, no full `just build`/`just clippy`, no `ci`; reading, editing and a single-target
  check are fine. To take it: `mkdir <lock>`, your name in `<lock>/owner`, `rmdir`-remove it when
  done. Only coordinators and integrators run `just ci` / `just smoke` (all); porters run single
  smokes, one QEMU at a time.
- Long runs (`large-ports.md`): every background run of minutes has a watcher that warns when
  its log has not changed for ten minutes (`stat -f %m <log>`); a progress report gives the
  log's last change time; before the final report every background task started is waited for
  or stopped, and `ps` shows none left (the report names any that was killed).
- Read cheaply: `grep`, `sed -n 'a,bp'`, long output redirected to scratchpad files. A ported
  C file is still read completely, in ranges.
- Faithfulness: no behavioural deviation to make QEMU work. A gap is reported with the serial
  lines; the coordinator probes OpenBSD 8.0 and the user decides (`scope-and-stubs.md`, "A
  QEMU device on which OpenBSD 8.0 itself fails").

## Stopping

- A "STOP ... wait for the user" refusal or a permission denial: stop at once, commit nothing
  more, report it. Never retry it or route around it.
- Anything that is the user's decision (a dependency, a download, an install, a clone widening,
  a rename away from OpenBSD's name, a scope change, a new `FIRMWARE_FLAKES` line): stop and
  hand back with the question.
- Low on context: stop in a committed state, update `HANDOFF.md` (done with hashes, left,
  pending diffs, ports.toml notes and blobs), then hand back naming its path.

## Final report

Branch and tip, commits in order, the evidence (serial lines, `rc=`, wall times, test counts),
deviations and stubs, what is left, the `HANDOFF.md` path, and the background tasks stopped.
Then `rm -rf target/smoke` in the worktree.
