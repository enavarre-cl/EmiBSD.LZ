---
name: progress
description: Porting progress of EmiBSD per ROADMAP milestone, measured live - files done and left, C lines done and left, Rust lines, and a time estimate from the project's own rate. Use when the user asks how far the port is, how much is left, how long a milestone will take, or wants status numbers.
argument-hint: [milestone]
allowed-tools: Bash(python3 -I ${CLAUDE_SKILL_DIR}/scripts/progress.py*)
---

The table below was computed just now from `ports.toml`, `docs/ROADMAP.md`, the reference
tree and git by `${CLAUDE_SKILL_DIR}/scripts/progress.py` (its docstring defines every column).

```!
python3 -I ${CLAUDE_SKILL_DIR}/scripts/progress.py $ARGUMENTS
```

How to answer:

- Show the table as it is; never retype or round its numbers. Reply in the user's language,
  keeping the table's English headers (artifacts stay in English).
- Under it, three to six lines: the last milestone met and the one under way, what is left and
  where it concentrates, and the caveats that matter for the question asked: a `wip` row counts
  whole as left; "unclaimed" is prose from the ROADMAP resolved in the reference tree, not rows;
  the estimate is a linear extrapolation of the project's own average and promises nothing.
- With a milestone argument (`/progress M16a`) the row list follows the table: say what is
  left in it and which rows are `wip`.
- If the table is missing or starts with `progress: error`, run the script with Bash and
  report the error. Never estimate by hand.
- These numbers are for the conversation. The docs (README, STATUS, JOURNAL) take theirs from
  `cargo xtask ports status` and the git commands `docs/JOURNAL.md` names.
