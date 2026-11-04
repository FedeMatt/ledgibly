---
name: keep-implementation-ledger
description: Keep a repo's plan, implementation ledger (task table + UTC log), docs, and README in sync with one status source. Use when starting or finishing a coding task, choosing the next task, recording evidence, adding a task ID, converting an approved plan into rows, or asking what is done versus pending, in any repo that has an implementation ledger or an "Agent profile" section in AGENTS.md.
---

# Keep the implementation ledger

Per-repo facts (paths, ID prefixes, verify commands, vocabulary, docs map, picking rules) live in the `## Agent profile` section of the repo's AGENTS.md. Read it first. If there is no profile or no ledger, bootstrap both from [reference.md](reference.md) before writing code. Prefer the repo's `init.sh` from project-blueprint when that kit is available.

## Document roles

- **Plan**: scope and architecture decisions. Change only when scope or architecture changes, and only with user agreement. Do not list per-feature task IDs here.
- **Ledger**: the task table and the chronological UTC log. It is the **only** place task status is asserted.
- **Docs**: how the system works. Link task IDs inline; never restate their status. Do not keep a hand-maintained `Tasks:` header or coverage table; coverage is `rg -l '<ID>' docs/`.
- **README**: install and run. Change only when those steps change.

## Plan to rows

When the user approves a plan (Cursor plan todos, or a scope change in `plan.md`), add a `pending` row for each todo **before writing code**:

1. Next free ID for the matching prefix (recipe in [reference.md](reference.md)).
2. Dependencies follow the plan order.
3. Acceptance condition is testable (≤200 chars).
4. Then pick work as usual.

## Read the ledger cheaply

Never read the whole ledger. Use the recipes in [reference.md](reference.md): one row by ID, open work, next free ID, recent log headings, then one log entry with offset/limit.

## Before changing code

1. Name the task ID, in this order: the ID the user gave; the open `in_progress` row; the first `pending` row whose dependencies are all `done` (check each with the one-row recipe); then any profile picking rule.
2. New work gets a stable ID (next free number for its prefix) **before** implementing.
3. Run the profile's pre-checks (for example, source-docs or Figma skills) when the task touches those areas.
4. Mark the row `in_progress`.

## Statuses

`pending` | `in_progress` | `blocked` | `done` | `superseded`

- `done` requires evidence: repo paths plus commands that passed in this session.
- Never flip `done` back. Add a follow-up ID that depends on the original.
- `superseded` names its replacement ID.
- `blocked` names the blocker in the evidence cell.
- When acceptance needs several environments (hosts, OSes, hardware), the row stays `in_progress` until every listed environment has evidence. Record each environment under a `### <ID>` subsection outside the table.

## Row budgets

| Cell | Budget |
|---|---|
| Description | 120 chars |
| Acceptance condition | 200 chars |
| Evidence | 300 chars: paths, commands, `log <timestamp>` |

Narrative belongs in the log, not in cells.

## After changing code

1. Run the profile's verify commands. Do not run commands the profile marks as not required. If `scripts/ledger_check.py` exists, run it too (`python3 scripts/ledger_check.py`).
2. Update the row: status and evidence.
3. Add one log entry, newest first, directly under the log heading (format in [reference.md](reference.md)). One entry per completed or blocked task; write a "started" entry only when work spans sessions.
4. Update only the docs the change actually affects. Cite the ID inline. Add a docs task ID when a new topic needs its own page.
5. README and plan: only under the conditions in "Document roles".

Use the profile vocabulary for existence claims (for example "in the repo" / "not built"), and only inside the ledger.

## Size guard

When the ledger exceeds the profile threshold (default 100 KB), propose archiving the oldest log entries verbatim to an archive file. Never archive unprompted, and never move the only log entry for an open task.

Do not commit or push unless the user asks. See [checklist.md](checklist.md).
