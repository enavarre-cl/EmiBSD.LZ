# Large ports and agent context

Decided by the user on 2026-10-03, after the first pf agent ran out of context with hours of
uncommitted work. Applies to every agent (main session or subagent) that ports or integrates
code, and to every prompt that launches one.

- **One big file, one subagent.** Every `.c` file of more than about 3,000 lines (`pf.c`,
  `tcp_input.c`, `if_pfsync.c`, ...) is ported by a subagent of its own. The coordinating
  agent only splits the work, integrates the results, runs `just ci` and commits.
- **Commit early.** Each piece that compiles is committed at once on the worktree branch.
  Never hold hours of uncommitted work. Intermediate commits must build; the last one of a
  series must be `just ci` green.
- **Handoff note.** The coordinator keeps an up-to-date note in the scratchpad (one file per
  task, e.g. `<scratchpad>/<task>/HANDOFF.md`): what is done (with commit hashes), what is
  left, the pending diffs and where they are, and the notes and `upstream_blob`s for
  `ports.toml`. A fresh agent must be able to resume from it without rereading everything.
- **Read cheaply.** `grep`, `sed -n '<a>,<b>p'` by ranges, and long command output redirected
  to scratchpad files and grepped. Never load a file of thousands of lines in one read. A file
  being ported is still read completely (`porting-workflow.md`), but in ranges, and by the
  subagent that ports it.
- **Stop cleanly.** An agent that feels its context running low stops in a committed state,
  writes the handoff note, and only then hands back, saying where the note is.

## Long runs in the background

Decided by the user on 2026-10-09, after a recipe sat hung for 2 h 30 min in a `ci-full` while
its log stayed still, and a `cargo test -p bsd -- hmac softraid_crypto` an agent had started
was found hung five hours after that agent handed back.

- **Watch every long background run.** `just ci`, `just ci-full`, `just diff-openbsd`,
  `just comp`, the host tests and any other run of minutes goes with a watcher that warns
  when its log has not changed for ten minutes (e.g. a `Monitor` or a loop that compares the
  log's modification time with the clock, `stat -f %m <log>` on macOS). A still log is
  looked at at once: which process is it, is it on CPU (`ps -o pid,etime,%cpu,state,command`),
  what is its last line. `smoke-all` stops a hung recipe by itself (`testing.md`); the rest
  of a run has no such guard.
- **Report progress from the clock, not only from the last line.** Every progress report
  says when the log last changed (`ls -l`/`stat`), so a run that stopped is never reported as
  running.
- **Finish what you started before handing back.** An agent, before its final report, waits
  for or stops every background task it launched (shells, `Monitor`s, `cargo test`, QEMU) and
  checks with `ps` that none is left; the report names any it had to kill. The coordinator
  checks the same for its subagents after each hand-back.
- Every prompt that launches an agent which runs these commands repeats these three points.
