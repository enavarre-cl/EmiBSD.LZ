export const meta = {
  name: 'port',
  description: 'Port OpenBSD C files to Rust: plan clusters, one porter per cluster in its own worktree, an independent review of each branch with one fix round, then one integrator merges the approved branches and runs just ci',
  phases: [
    { title: 'Plan', detail: 'group the requested files into clusters that port alone from main, leaf dependencies folded in' },
    { title: 'Port', detail: 'one porter or porter-mechanical per cluster, in its own worktree, at most 4 at once' },
    { title: 'Review', detail: 'an independent reviewer per branch; one fix round, then a second look' },
    { title: 'Integrate', detail: 'merge the approved branches, just ci under the machine lock' },
  ],
}

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

// /port: the porting loop of CLAUDE.md run as a workflow over the agents of .claude/agents/.
//
//   /port sys/dev/ic/mfi.c sys/dev/pci/mfi_pci.c      C paths as in ports.toml
//   /port M17                                        every todo row of a milestone
//   /port                                            what `cargo xtask ports next` lists
//   /port {"files": [...], "max": 2, "base": "<branch>", "note": "<the user's words>"}
//
// The result is a branch with every approved port merged and `just ci` green; the main session
// fast-forwards main after the user's OK. The workflow never touches main and never pushes.
// Rules every agent follows: .claude/rules/subagents.md (its "Workflows" section names the
// lock and the notes directory used here).

const opts = (args && typeof args === 'object' && !Array.isArray(args)) ? args : {}
const requested = Array.isArray(args) ? args.map(String)
  : (typeof args === 'string' && args.trim()) ? args.trim().split(/\s+/)
  : Array.isArray(opts.files) ? opts.files.map(String) : []
const MAX = Number(opts.max) > 0 ? Number(opts.max) : 4 // the project's cap on agents that boot QEMU
const BASE = opts.base ? String(opts.base) : ''
const NOTE = opts.note ? String(opts.note) : `the user invoked /port ${requested.join(' ') || '(no arguments)'}`
const LOCK = '/tmp/emibsd/ci.lock'
const NOTES = '/tmp/emibsd/port'

const COMMON = [
  'Context of this run (the rest is in .claude/rules/subagents.md and in your definition):',
  `- base: your worktree is branched from the main checkout's HEAD${BASE ? `; first \`git merge ${BASE}\`` : ''}.`,
  `- machine lock: ${LOCK} (mkdir takes it; a porter never runs just ci, just smoke or just ci-full).`,
  `- who else runs: up to ${MAX} porters of this /port workflow, each in its own worktree, single smokes only.`,
  `- authorisation: ${NOTE}.`,
  '- the main checkout is the first line of `git worktree list --porcelain` (reference tree, target/openbsd).',
  '- a "STOP ... wait for the user" refusal or a permission denial: stop, commit nothing more, report it as blocked.',
].join('\n')

// At most MAX agents that may boot QEMU at once; reviewers run outside the limit (no QEMU).
function limiter(n) {
  let active = 0
  const queue = []
  const pump = () => {
    while (active < n && queue.length) {
      active++
      queue.shift()()
    }
  }
  return fn => new Promise((resolve, reject) => {
    queue.push(() => fn().then(resolve, reject).finally(() => { active--; pump() }))
    pump()
  })
}
const slot = limiter(MAX)

const PLAN = {
  type: 'object',
  properties: {
    clusters: {
      type: 'array',
      items: {
        type: 'object',
        properties: {
          name: { type: 'string', description: 'a short slug, e.g. mfi' },
          files: { type: 'array', items: { type: 'string' }, description: 'C paths as in ports.toml: the .c, its headers, its todo leaf dependencies' },
          delicate: { type: 'boolean', description: 'true = porter (opus); false = porter-mechanical (sonnet)' },
          lines: { type: 'integer', description: 'wc -l of the C files together' },
          why: { type: 'string', description: 'why this grouping and this class, one line' },
          study: { type: 'array', items: { type: 'string' }, description: 'existing Rust files to copy conventions from' },
        },
        required: ['name', 'files', 'delicate', 'lines', 'why', 'study'],
      },
    },
    excluded: { type: 'array', items: { type: 'string' }, description: 'requested files left out, each with its reason' },
    notes: { type: 'string', description: 'anything the porters or the user should know first' },
  },
  required: ['clusters', 'excluded', 'notes'],
}

const PORT_RESULT = {
  type: 'object',
  properties: {
    branch: { type: 'string' },
    tip: { type: 'string', description: 'the commit hash at the tip of the branch' },
    commits: { type: 'array', items: { type: 'string' }, description: 'hash and subject, oldest first' },
    status: { type: 'string', enum: ['done', 'partial', 'blocked'] },
    evidence: { type: 'array', items: { type: 'string' }, description: 'exact serial lines, test counts, smoke recipe names' },
    deviations: { type: 'array', items: { type: 'string' } },
    left: { type: 'array', items: { type: 'string' } },
    question: { type: 'string', description: 'what needs the user, if anything; empty otherwise' },
    handoff: { type: 'string', description: 'the path of HANDOFF.md' },
  },
  required: ['branch', 'tip', 'commits', 'status', 'evidence', 'deviations', 'left', 'question', 'handoff'],
}

const REVIEW = {
  type: 'object',
  properties: {
    verdict: { type: 'string', enum: ['approve', 'changes'] },
    defects: {
      type: 'array',
      items: {
        type: 'object',
        properties: {
          file: { type: 'string' },
          line: { type: 'integer' },
          summary: { type: 'string' },
          c_ref: { type: 'string', description: 'reference/openbsd-src/<path>:<line> the Rust must match' },
          fix: { type: 'string' },
        },
        required: ['file', 'line', 'summary', 'c_ref', 'fix'],
      },
    },
    notes: { type: 'array', items: { type: 'string' }, description: 'lesser remarks, no action required' },
    unchecked: { type: 'string', description: 'what the review did not cover' },
  },
  required: ['verdict', 'defects', 'notes', 'unchecked'],
}

const INTEGRATION = {
  type: 'object',
  properties: {
    branch: { type: 'string' },
    tip: { type: 'string' },
    ci_rc: { type: 'integer', description: 'exit code of just ci; -1 if it did not run' },
    ci_minutes: { type: 'number' },
    conflicts: { type: 'array', items: { type: 'string' }, description: 'each conflict and how it was resolved' },
    userland_rebuilt: { type: 'boolean' },
    left: { type: 'array', items: { type: 'string' }, description: 'anything not green or not merged, with the reason' },
  },
  required: ['branch', 'tip', 'ci_rc', 'ci_minutes', 'conflicts', 'userland_rebuilt', 'left'],
}

const portPrompt = c => [
  `Port cluster "${c.name}" of EmiBSD: ${c.files.join(', ')} (${c.lines} lines of C). ${c.why}`,
  `Study first: ${c.study.length ? c.study.join(', ') : 'the nearest sibling in the tree'}.`,
  `Notes and HANDOFF.md: ${NOTES}/${c.name}/ (mkdir -p).`,
  COMMON,
  'Your final answer is the structured result: branch, tip, commits, status (done | partial | blocked), evidence lines, deviations, what is left, any question for the user, the HANDOFF.md path.',
].join('\n')

const reviewPrompt = (c, r) => [
  `Review branch ${r.branch} (tip ${r.tip}) of EmiBSD against main: the port of ${c.files.join(', ')}.`,
  `Work in the main checkout, read-only: \`git log --oneline main..${r.branch}\`, \`git diff main ${r.branch} -- <path>\`, \`git show ${r.branch}:<path>\`, the C under reference/openbsd-src.`,
  `The porter reported: status ${r.status}; evidence ${JSON.stringify(r.evidence)}; deviations ${JSON.stringify(r.deviations)}; left ${JSON.stringify(r.left)}.`,
  "Apply your definition's checklist. 'changes' only for a defect with evidence on both sides (Rust path:line, C path:line) and a concrete failing input or sequence; style alone is a note.",
].join('\n')

const fixPrompt = (c, r, rev) => [
  `Fix the reviewed defects of EmiBSD branch ${r.branch} (the port of ${c.files.join(', ')}).`,
  `In your worktree first: \`git merge ${r.branch}\` (a fast-forward from main). Then fix each defect, with a host test where the logic allows, run the checks your definition lists, and commit on your branch (trailers as in git-commits.md). Do not rewrite the port.`,
  `Defects:\n${rev.defects.map(d => `- ${d.file}:${d.line}: ${d.summary} (C: ${d.c_ref}). Fix: ${d.fix}`).join('\n')}`,
  `Reviewer notes: ${rev.notes.length ? rev.notes.join(' | ') : 'none'}.`,
  `Notes and HANDOFF.md: ${NOTES}/${c.name}/.`,
  COMMON,
  'Your final answer is the structured result with your own branch and tip (they differ from the reviewed ones).',
].join('\n')

// --- Plan -----------------------------------------------------------------------------------

phase('Plan')
const plan = await agent([
  'You plan a batch of EmiBSD ports (a faithful Rust port of the OpenBSD kernel). You write nothing, commit nothing, build nothing, boot nothing.',
  `Requested: ${requested.length ? requested.join(', ') : 'nothing by name: use what `PATH=/opt/homebrew/opt/rustup/bin:$PATH cargo xtask ports next` lists'}.`,
  'A single token like "M17" names a milestone: take every todo row of ports.toml whose notes or section comment name it.',
  'For each requested C file: its ports.toml row (status, deps, notes), its size (`wc -l reference/openbsd-src/<path>`), the headers it owns, and the todo leaf dependencies it needs (`deps`, `cargo xtask ports next`).',
  'Group into clusters that each port alone from main: one .c with its own headers plus its todo leaf dependencies; a .c over about 3,000 lines alone (.claude/rules/large-ports.md); two clusters that would need each other become one.',
  'Class per .claude/rules/subagents.md: delicate = uvm, pmap, traps, context switch, MP, signals, exec, network stacks, drivers with DMA, interrupts or MMIO, anything with locking; mechanical = headers of constants and structs, tables, small self-contained leaves. Unsure = delicate.',
  'Exclude, each with its reason: rows already ported or wip on another branch (`git branch -a`, the row notes), skipped rows, files outside reference/openbsd-src/sys, a licence or scope decision the user has not made (.claude/rules/scope-and-stubs.md), a QEMU device OpenBSD 8.0 itself fails on unless the user said to port it anyway.',
  'For each cluster name the existing Rust files to copy conventions from (grep the tree for the sibling driver or subsystem).',
].join('\n'), { phase: 'Plan', label: 'plan', schema: PLAN })

if (!plan || !plan.clusters.length) {
  log('nothing to port')
  return { planned: 0, excluded: plan ? plan.excluded : ['the planner returned nothing'], notes: plan ? plan.notes : '' }
}
log(`${plan.clusters.length} clusters (${plan.clusters.filter(c => c.delicate).length} delicate), ${plan.excluded.length} excluded, at most ${MAX} porters at once`)
plan.excluded.forEach(x => log(`excluded: ${x}`))

// --- Port, then review each branch as soon as its porter is done (no barrier) -------------------

const results = await pipeline(
  plan.clusters,
  c => slot(() => agent(portPrompt(c), {
    agentType: c.delicate ? 'porter' : 'porter-mechanical', isolation: 'worktree',
    phase: 'Port', label: `port:${c.name}`, schema: PORT_RESULT,
  })),
  async (r, c) => {
    if (!r) {
      log(`${c.name}: the porter returned nothing`)
      return null
    }
    if (r.status === 'blocked') {
      log(`${c.name}: blocked: ${r.question || r.left.join('; ') || 'no reason given'}`)
      return { cluster: c, port: r, review: null, verdict: 'blocked' }
    }
    let port = r
    let review = await agent(reviewPrompt(c, port), { agentType: 'reviewer', phase: 'Review', label: `review:${c.name}`, schema: REVIEW })
    if (review && review.verdict === 'changes' && review.defects.length) {
      log(`${c.name}: ${review.defects.length} defect(s), one fix round`)
      const fixed = await slot(() => agent(fixPrompt(c, port, review), {
        agentType: c.delicate ? 'porter' : 'porter-mechanical', isolation: 'worktree',
        phase: 'Review', label: `fix:${c.name}`, schema: PORT_RESULT,
      }))
      if (fixed && fixed.status !== 'blocked') {
        port = fixed
        review = await agent(reviewPrompt(c, port), { agentType: 'reviewer', phase: 'Review', label: `re-review:${c.name}`, schema: REVIEW })
      }
    }
    const verdict = review ? review.verdict : 'unreviewed'
    log(`${c.name}: ${verdict}${port.status === 'partial' ? ' (partial port)' : ''}`)
    return { cluster: c, port, review, verdict }
  },
)

const done = results.filter(Boolean)
const approved = done.filter(x => x.verdict === 'approve')
const held = done.filter(x => x.verdict !== 'approve')
held.forEach(x => log(`not integrated: ${x.cluster.name} (${x.verdict})`))

const summary = {
  planned: plan.clusters.length,
  excluded: plan.excluded,
  planner_notes: plan.notes,
  approved: approved.map(x => ({
    cluster: x.cluster.name, branch: x.port.branch, tip: x.port.tip, status: x.port.status,
    commits: x.port.commits, evidence: x.port.evidence, deviations: x.port.deviations,
    left: x.port.left, review_notes: x.review.notes, unchecked: x.review.unchecked,
  })),
  held: held.map(x => ({
    cluster: x.cluster.name, verdict: x.verdict, branch: x.port.branch, tip: x.port.tip,
    question: x.port.question, defects: x.review ? x.review.defects : [], left: x.port.left,
    handoff: x.port.handoff,
  })),
  integrated: null,
  next: '',
}

if (!approved.length) {
  summary.next = 'nothing approved: read the held list; the branches and HANDOFF.md files are still there'
  return summary
}

// --- Integrate: one agent needs every approved branch, so this barrier is the real one -------------

phase('Integrate')
const integ = await agent([
  `Integrate these reviewed EmiBSD branches into one: ${approved.map(x => `${x.port.branch} (${x.cluster.name}: ${x.cluster.files.join(', ')})`).join('; ')}.`,
  'In your worktree: `git merge` each branch in that order, resolving the shared tables as your definition says (ports.toml with one notes key per row, ioconf renumbering on both archs, the justfile smokes list, xtask option tables, agent-memory files with both sides kept).',
  `Then, under the lock ${LOCK}: \`just userland\` if any branch touched tools/xtask/src/userland*, then \`just jobs=3 ci\`, gated on rc=0 exactly. Fix only what the merge broke; a port's own defect goes in your report, not under the rug.`,
  `Never touch main, never push. Notes: ${NOTES}/integrate/.`,
  COMMON,
  'Your final answer is the structured result: branch, tip, ci rc and minutes, each conflict and its resolution, whether the userland was rebuilt, what is left.',
].join('\n'), { agentType: 'integrator', isolation: 'worktree', phase: 'Integrate', label: 'integrate', schema: INTEGRATION })

summary.integrated = integ
summary.next = integ && integ.ci_rc === 0
  ? `ci green on ${integ.branch} (${integ.tip}): with the user's OK, fast-forward main to it, then remove the merged worktrees and branches (disk hygiene)`
  : 'ci not green or the integrator returned nothing: read integrated.left and the held list before anything reaches main'
return summary
