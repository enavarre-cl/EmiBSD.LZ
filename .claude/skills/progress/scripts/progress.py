#!/usr/bin/env python3
#
# Copyright (c) 2026 Emilio Navarrete Lineros <enavarre@outlook.com>
#
# Permission to use, copy, modify, and distribute this software for any
# purpose with or without fee is hereby granted, provided that the above
# copyright notice and this permission notice appear in all copies.
#
# THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES
# WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
# MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR
# ANY SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
# WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN
# ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF
# OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.
#
"""Porting progress of EmiBSD, one row per milestone of docs/ROADMAP.md.

Sources, all read live (nothing is typed in by hand):
  - docs/ROADMAP.md: the milestone table (id, title, scope, exit criterion);
  - ports.toml: every [[file]]/[[extra]] row, assigned to a milestone by the
    `Mxx:` / `Mxx-b:` / `Mxx,` prefix of its `notes`, else by the `# Mxx ...`
    section comment above it;
  - reference/openbsd-src/<c>: `wc -l` of the C at the pin (done = ported rows,
    left = todo + wip rows, a wip row counted whole; skipped rows count nowhere);
  - the Rust files the rows reference: `wc -l`, tests included, each file once;
  - git: the `docs: Mxx met` close commits, docs/JOURNAL.md's boundary table and
    the ROADMAP's own `done`/`Met` dates; the project's active days (distinct
    commit dates) give the rate used for the estimate.

For a milestone not yet met, the C files its ROADMAP scope cell names in
backticks (`foo.c`, `dir/`) that have no ports.toml row yet are resolved in the
reference tree and shown as "~N unclaimed": an estimate from prose, marked so.

The estimate is a linear extrapolation at the project's average rate of C lines
per active day. It measures the past; it promises nothing.

Usage: progress.py [MILESTONE]   (e.g. M16a: adds that milestone's row list)
Always exits 0; errors are printed, so a skill preamble never aborts on them.
"""

import datetime as _dt
import os
import re
import subprocess
import sys
from pathlib import Path

try:
    import tomllib
except ModuleNotFoundError:  # macOS's /usr/bin/python3 is older than 3.11
    print(f"progress: Python 3.11+ is needed (tomllib); this is {sys.version.split()[0]}")
    sys.exit(0)

MS_ID = re.compile(r"M\d+[a-z]*\+?")
# A milestone id that has no ROADMAP row of its own goes to this one.
ALIASES = {"M7b": "M7+"}
SRC_SUFFIXES = (".c", ".h", ".S")


def root_dir() -> Path:
    env = os.environ.get("CLAUDE_PROJECT_DIR")
    if env and (Path(env) / "ports.toml").is_file():
        return Path(env)
    for p in Path(__file__).resolve().parents:
        if (p / "ports.toml").is_file() and (p / "docs" / "ROADMAP.md").is_file():
            return p
    raise SystemExit("progress: cannot find the repository root (ports.toml)")


def count_lines(path: Path, cache: dict) -> int | None:
    if path in cache:
        return cache[path]
    try:
        with open(path, "rb") as f:
            n = f.read().count(b"\n")
    except OSError:
        n = None
    cache[path] = n
    return n


# --- ROADMAP ---------------------------------------------------------------

def parse_roadmap(root: Path) -> list[dict]:
    """The milestone table: [{id, title, scope, criterion, row_date}] in file order."""
    rows = []
    for line in (root / "docs" / "ROADMAP.md").read_text().splitlines():
        if not line.startswith("| **M"):
            continue
        cells = [c.strip() for c in re.split(r"(?<!\\)\|", line)][1:-1]
        if len(cells) < 3:
            continue
        m = re.match(r"\*\*(M\d+[a-z]*\+?)\s*(.*?)\*\*", cells[0])
        if not m:
            continue
        dm = re.search(r"\b(?:done|Met|met)\**\s+(\d{4}-\d{2}-\d{2})", line)
        rows.append({"id": m.group(1), "title": m.group(2).strip(), "scope": cells[1],
                     "criterion": cells[2], "row_date": dm.group(1) if dm else None})
    return rows


# --- ports.toml ------------------------------------------------------------

def parse_ports(root: Path) -> list[dict]:
    """Every row with `_ms`, its milestone id (notes prefix, else section comment)."""
    entries: list[dict] = []
    sec_re = re.compile(r"^#\s*(M\d+[a-z]*\+?)")
    note_re = re.compile(r"^(M\d+[a-z]*\+?)(?:-[a-z0-9]+)?\s*[:,]")
    section = None
    blocks: list[tuple[str, str | None, list[str]]] = []
    for line in (root / "ports.toml").read_text().splitlines(keepends=True):
        sm = sec_re.match(line)
        if sm:
            section = sm.group(1)
        if line.startswith("[[file]]") or line.startswith("[[extra]]"):
            blocks.append(("file" if line.startswith("[[file]]") else "extra", section, [line]))
        elif blocks:
            blocks[-1][2].append(line)
    for kind, sec, text in blocks:
        e = tomllib.loads("".join(text))[kind][0]
        e["_kind"] = kind
        nm = note_re.match(e.get("notes", "") or "")
        e["_ms"] = nm.group(1) if nm else sec
        entries.append(e)
    return entries


# --- milestone status --------------------------------------------------------

def git(root: Path, *args: str) -> str:
    try:
        return subprocess.run(["git", "-C", str(root), *args], check=True,
                              capture_output=True, text=True).stdout
    except (OSError, subprocess.CalledProcessError) as e:
        print(f"progress: git {' '.join(args)} failed: {e}")
        return ""


def close_dates(root: Path) -> dict[str, str]:
    """Milestone id -> the date it was declared met (git first, then the JOURNAL table)."""
    dates: dict[str, str] = {}
    # "docs: M16a met (...)", "docs: M11e and M11 met", "docs: ... met ..., and with it M10".
    for line in reversed(git(root, "log", "--format=%cd%x09%s", "--date=short").splitlines()):
        date, _, subject = line.partition("\t")
        if not subject.startswith("docs:") or not re.search(r"\bmet\b", subject):
            continue
        if re.search(r"\b(left|blocked|partial|pending)\b", subject):
            continue  # a partial criterion, not the close
        for mid in MS_ID.findall(subject.split("(")[0]):
            dates.setdefault(mid, date)
    journal = root / "docs" / "JOURNAL.md"
    if journal.is_file():  # M0..M12+ have no close commit of their own
        for line in journal.read_text().splitlines():
            m = re.match(r"^\| ([^|]+) \| [^|]+ \| (\d{4}-\d{2}-\d{2}) \|", line)
            if m:
                for mid in MS_ID.findall(m.group(1)):
                    dates.setdefault(mid, m.group(2))
    return dates


def under_way(root: Path) -> set[str]:
    status = root / "docs" / "STATUS.md"
    if not status.is_file():
        return set()
    return {m.group(1) for m in
            re.finditer(r"(M\d+[a-z]*\+?)\b[^.;]*?\bunder way", status.read_text())}


# --- scope estimate for files no ports.toml row claims yet --------------------

def build_index(ref: Path) -> dict[str, list[Path]]:
    index: dict[str, list[Path]] = {}
    sysdir = ref / "sys"
    if sysdir.is_dir():
        for p in sysdir.rglob("*"):
            if p.is_dir():
                index.setdefault("dir:" + p.name, []).append(p)
            elif p.suffix in SRC_SUFFIXES:
                index.setdefault(p.name, []).append(p)
    return index


def scope_estimate(scope: str, ref: Path, index: dict, claimed: set[Path], cache: dict):
    """(lines, files, unresolved names) of the C the scope cell names that no row claims."""
    paths: set[Path] = set()
    unresolved = 0
    for tok in re.findall(r"`([^`]+)`", scope):
        tok = tok.strip()
        if "{" in tok or " " in tok:
            continue
        if tok.endswith("/"):
            name = tok.rstrip("/")
            cands = [ref / "sys" / name] + index.get("dir:" + name.split("/")[-1], [])
            d = next((c for c in cands if c.is_dir()), None)
            if d is None:
                unresolved += 1
                continue
            paths.update(p for p in d.rglob("*") if p.suffix in SRC_SUFFIXES and p.is_file())
        elif tok.endswith(SRC_SUFFIXES):
            p = ref / "sys" / tok if "/" in tok else None
            if p is None or not p.is_file():
                hits = index.get(Path(tok).name, [])
                p = hits[0] if len(hits) == 1 else None
            if p is None:
                unresolved += 1
                continue
            paths.add(p)
    fresh = [p for p in paths if p.resolve() not in claimed]
    return sum(count_lines(p, cache) or 0 for p in fresh), len(fresh), unresolved


# --- main -----------------------------------------------------------------------

def fmt(n) -> str:
    return "—" if n is None else f"{n:,}"


def new_tally() -> dict:
    return {"done": 0, "todo": 0, "wip": 0, "skipped": 0, "c_done": 0, "c_left": 0,
            "c_wip": 0, "c_unknown": 0, "rust": 0, "rust_files": 0}


def main() -> None:
    root = root_dir()
    want = sys.argv[1] if len(sys.argv) > 1 and sys.argv[1] else None
    ref = root / "reference" / "openbsd-src"
    cache: dict = {}

    rows = parse_roadmap(root)
    entries = parse_ports(root)
    dates = close_dates(root)
    wip_ms = under_way(root)
    commit_dates = sorted(set(git(root, "log", "--format=%cd", "--date=short").split()))
    days = len(commit_dates) or 1
    try:
        pinned = tomllib.loads((root / "ports.toml").read_text())["meta"]["pinned"][:12]
    except (OSError, KeyError, tomllib.TOMLDecodeError):
        pinned = "?"

    row_ids = [r["id"] for r in rows]
    children = {rid: [c for c in row_ids if re.fullmatch(re.escape(rid) + r"[a-z]", c)]
                for rid in row_ids}
    is_child = {c for cs in children.values() for c in cs}

    def owner(mid: str | None) -> str:
        """The ROADMAP row an entry's milestone id belongs to."""
        if mid is None:
            return "unassigned"
        mid = ALIASES.get(mid, mid)
        if mid in row_ids:
            return mid
        parent = re.sub(r"[a-z]+$", "", mid)
        return parent if parent in row_ids else "unassigned"

    own = {rid: new_tally() for rid in row_ids + ["unassigned"]}
    rows_of: dict[str, list[dict]] = {rid: [] for rid in own}
    seen_rust: set[str] = set()
    claimed: set[Path] = set()
    missing_ref = 0
    for e in entries:
        rid = owner(e["_ms"])
        rows_of[rid].append(e)
        t = own[rid]
        if e["_kind"] == "file":
            cpath = ref / e["c"]
            claimed.add(cpath.resolve())
            n = count_lines(cpath, cache)
            if n is None:
                missing_ref += 1
                t["c_unknown"] += 1
                n = 0
            e["_c_lines"] = n
            st = e.get("status", "todo")
            if st == "ported":
                t["done"] += 1
                t["c_done"] += n
            elif st == "skipped":
                t["skipped"] += 1
            else:
                key = "wip" if st == "wip" else "todo"
                t[key] += 1
                t["c_left"] += n
                if key == "wip":
                    t["c_wip"] += n
        rs = e.get("rust", "")
        if rs and rs not in seen_rust:
            seen_rust.add(rs)
            rl = count_lines(root / rs, cache)
            if rl is not None:
                t["rust"] += rl
                t["rust_files"] += 1

    # A parent row (M9, M10, M11, M16, M8) shows its own rows plus its children's.
    agg = {rid: dict(t) for rid, t in own.items()}
    for rid in row_ids:
        for cid in children[rid]:
            for k, v in own[cid].items():
                agg[rid][k] += v

    total = new_tally()
    for t in own.values():
        for k, v in t.items():
            total[k] += v
    rate = total["c_done"] / days

    def status_of(rid: str) -> str:
        if children[rid] and all(status_of(c).startswith("met") for c in children[rid]):
            return "met " + (dates.get(rid) or max(dates.get(c, "") for c in children[rid]))
        if rid in dates:
            return "met " + dates[rid]
        r = next(x for x in rows if x["id"] == rid)
        if r["row_date"] and not agg[rid]["todo"] and not agg[rid]["wip"]:
            return "met " + r["row_date"]
        if rid in wip_ms or agg[rid]["wip"]:
            return "under way"
        return "pending"

    def est_days(lines: int) -> str:
        if not rate:
            return "—"
        d = lines / rate
        return "<0.1 d" if d < 0.1 else f"~{d:.1f} d"

    index = None
    print(f"EmiBSD porting progress — {_dt.date.today()} — OpenBSD pin {pinned} — "
          f"{len(entries)} ports.toml rows — {days} active days since {commit_dates[0] if commit_dates else '?'}")
    print()
    print("| Milestone | Title | Status | Files done | Files left | C lines done | C lines left | Rust lines | Est. left |")
    print("|---|---|---|---:|---:|---:|---:|---:|---:|")
    for r in rows:
        rid = r["id"]
        if want and not (rid == want or re.fullmatch(re.escape(want) + r"[a-z]", rid)):
            continue
        t = agg[rid]
        st = status_of(rid)
        has_rows = any(t[k] for k in ("done", "todo", "wip", "skipped"))
        left_files = t["todo"] + t["wip"]
        left_txt = fmt(left_files) + (f" ({t['wip']} wip)" if t["wip"] else "")
        c_left_txt = fmt(t["c_left"]) + (f" ({t['c_wip']:,} wip)" if t["c_wip"] else "")
        left_total = t["c_left"]
        if not st.startswith("met") and not children[rid]:
            if index is None:
                index = build_index(ref)
            lines, files, unres = scope_estimate(r["scope"], ref, index, claimed, cache)
            if lines:
                left_total += lines
                c_left_txt = (f"{c_left_txt} + ~{lines:,} unclaimed ({files} files)"
                              if has_rows else f"~{lines:,} unclaimed ({files} files)")
            elif not has_rows:
                c_left_txt = "no rows yet"
            if unres:
                c_left_txt += f"; {unres} name{'s' if unres > 1 else ''} unresolved"
        if left_total:
            est = est_days(left_total)
        else:
            est = "done" if st.startswith("met") else "—"
        indent = "&nbsp;&nbsp;" if rid in is_child else ""
        files_done = fmt(t["done"]) if has_rows else "—"
        print(f"| {indent}{rid} | {r['title']} | {st} | {files_done} | {left_txt} | "
              f"{fmt(t['c_done'])} | {c_left_txt} | {fmt(t['rust']) if t['rust'] else '—'} | {est} |")
    u = own["unassigned"]
    if (u["done"] + u["todo"] + u["wip"]) and not want:
        print(f"| (no milestone) | rows whose section and notes name no ROADMAP milestone | — | "
              f"{u['done']} | {u['todo'] + u['wip']} | {fmt(u['c_done'])} | {fmt(u['c_left'])} | {fmt(u['rust'])} | — |")
    print()

    if want:
        ids = [rid for rid in row_ids if rid == want or re.fullmatch(re.escape(want) + r"[a-z]", rid)]
        sel = [e for rid in ids for e in rows_of.get(rid, [])]
        if sel:
            print(f"Rows of {want} ({len(sel)}):")
            print()
            print("| C file | Status | C lines | Rust file | Notes |")
            print("|---|---|---:|---|---|")
            for e in sorted(sel, key=lambda e: (e.get("status", ""), e.get("c", e.get("rust", "")))):
                notes = (e.get("notes") or e.get("reason") or "").replace("|", "\\|")
                if len(notes) > 70:
                    notes = notes[:67] + "..."
                print(f"| {e.get('c', '(extra)')} | {e.get('status', 'extra')} | "
                      f"{fmt(e.get('_c_lines'))} | {e.get('rust', '')} | {notes} |")
            print()

    rust_total = 0
    for sub in ("sys", "tools"):
        for p in (root / sub).rglob("*.rs"):
            rust_total += count_lines(p, cache) or 0
    print(f"Totals: {total['done']} files ported, {total['todo'] + total['wip']} left "
          f"({total['todo']} todo, {total['wip']} wip), {total['skipped']} skipped; "
          f"C lines ported {total['c_done']:,}, left {total['c_left']:,} ({total['c_wip']:,} of them in wip rows); "
          f"Rust lines in sys/ and tools/: {rust_total:,}.")
    if rate:
        print(f"Rate: {rate:,.0f} C lines per active day over {days} active days. "
              f"Every row still open, at that rate: {est_days(total['c_left'])}.")
    if missing_ref:
        print(f"Warning: {missing_ref} C files not found under reference/openbsd-src "
              f"(is the reference tree checked out?); their lines count as 0.")
    print()
    print("Method: files = ports.toml rows (skipped rows count nowhere; a wip row counts whole as left). "
          "C lines = wc -l of the file at the pin; Rust lines = wc -l of the referenced .rs, tests included, "
          "each file once. A parent milestone (M8, M9, M10, M11, M16) shows its own rows plus its lettered "
          "children's. 'unclaimed' = C the ROADMAP scope names in backticks with no ports.toml row yet, "
          "resolved in the reference tree: an estimate from prose. The time column is a linear "
          "extrapolation of the project's own average; it measures the past and promises nothing.")


if __name__ == "__main__":
    try:
        main()
    except Exception as exc:  # the skill preamble must never abort on this script
        print(f"progress: error: {exc!r}")
    sys.exit(0)
