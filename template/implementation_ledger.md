# {{PROJECT}} Implementation Ledger

Living task list. Scope: [plan.md](plan.md). Statuses: `pending`, `in_progress`, `blocked`, `done`, `superseded`. Older log entries and trimmed evidence: [archive](implementation_ledger_archive.md).

## Tasks

| ID | Description | Dependencies | Status | Acceptance condition | Evidence |
|---|---|---|---|---|---|
| ART-001 | Approve product and architecture baseline | | pending | plan.md and AGENTS.md have no `<fill me>` | |
| ART-002 | Operational implementation ledger | ART-001 | done | Ledger has Tasks table, UTC log, and checker passes | implementation_ledger.md; `python3 scripts/ledger_check.py`; log {{DATE}} |

## Environment evidence

Multi-host or multi-OS notes live here as `### <ID>` subsections, not in the table cell.

## Milestones

| Gate | Tasks |
|---|---|
| 1. Approved plan | ART-001 |
| 2. Operational ledger | ART-002 |

## Chronological UTC Log

Newest first. Older entries: [archive](implementation_ledger_archive.md).

### {{DATE}} — Operational ledger bootstrapped (ART-002)

- Seeded ART-001 and ART-002 from Ledgibly `init.sh`.
- Checker: `python3 scripts/ledger_check.py`.
- Next: fill `<fill me>` lines in `AGENTS.md` and `plan.md`, then add product rows.
