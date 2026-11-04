# project-blueprint Implementation Ledger

Living task list. Scope: [plan.md](plan.md). Statuses: `pending`, `in_progress`, `blocked`, `done`, `superseded`. Older log entries and trimmed evidence: [archive](implementation_ledger_archive.md).

## Tasks

| ID | Description | Dependencies | Status | Acceptance condition | Evidence |
|---|---|---|---|---|---|
| BP-003 | Add MIT LICENSE with generic contributors copyright | BP-001 | done | LICENSE is MIT; README links it; init does not copy it into targets | LICENSE; README.md; `bash tests/selftest.sh`; log 2026-11-04T08:54:04Z |
| BP-002 | Install canonical skills into ~/.cursor/skills and ~/.agents/skills | BP-001 | done | Three skills readable via symlink from both homes | `./install-user-skills.sh`; SKILL.md via symlink; backup-20261104T082343Z; log 2026-11-04T08:24:03Z |
| BP-001 | Create reusable ledger kit: skills, template, modules, init, checker | | done | Kit boots a target repo with profile, ledger, docs, and no overwrite | skills/; template/; modules/; init.sh; install-user-skills.sh; log 2026-11-04T08:23:09Z |
| DOC-003 | Remove comparative-history section from the kit README | DOC-002 | done | README has no Design decisions / earlier-drift table; docs cite DOC-003 | README.md; docs/01-framework.md; `bash tests/selftest.sh`; log 2026-11-04T09:01:30Z |
| DOC-002 | Strip private-project names from public-facing docs | DOC-001 | done | README, plan, and DOC-001 log name no private repos or product stacks | README.md; plan.md; docs/01-framework.md; `rtk rg` empty; log 2026-11-04T08:54:04Z |
| DOC-001 | Framework README, plan, and docs/01-framework.md | BP-001 | done | README explains roles, bootstrap, review findings; docs cite IDs inline | README.md; plan.md; docs/01-framework.md; log 2026-11-04T08:23:09Z |
| QA-001 | selftest for init and ledger_check | BP-001 | done | selftest inits with all modules, checker passes, re-init refuses, bad row fails | tests/selftest.sh; `bash tests/selftest.sh`; log 2026-11-04T08:23:09Z |

## Environment evidence

Multi-host or multi-OS notes live here as `### <ID>` subsections, not in the table cell.

## Milestones

| Gate | Tasks |
|---|---|
| 1. Kit in the repo | BP-001, DOC-001, QA-001, BP-002 |

## Chronological UTC Log

Newest first. Older entries: [archive](implementation_ledger_archive.md).

### 2026-11-04T09:01:30Z — Kit README has no comparative-history section (DOC-003)

- Deleted the README Design decisions / earlier-drift table. docs/01-framework.md cites DOC-001, DOC-002, DOC-003 on install/run-only README.

### 2026-11-04T08:54:04Z — MIT LICENSE for the kit (BP-003)

- LICENSE is MIT, copyright 2026 project-blueprint contributors. README links it. init.sh copies template/ only, so targets pick their own license.

### 2026-11-04T08:54:04Z — Public-facing docs name no private projects (DOC-002)

- README design-decision section, plan.md Product/non-goals, and the DOC-001 log bullet no longer name private repos or product stacks. docs/01-framework.md cites DOC-002.

### 2026-11-04T08:24:03Z — Canonical skills installed as user-level symlinks (BP-002)

- `./install-user-skills.sh` backed up existing keep-implementation-ledger and figma-ui, then linked keep-implementation-ledger, code-review, and figma-ui from both ~/.cursor/skills and ~/.agents/skills into skills/.
- SKILL.md is readable through the symlink. If a future Cursor chat ignores the links, rerun with `--copy`.

### 2026-11-04T08:23:09Z — selftest covers init and checker (QA-001)

- `tests/selftest.sh` inits with every module, inits a bare repo, asserts no leftover `{{PLACEHOLDER}}` tokens, symlink `.cursor/skills -> ../.agents/skills`.
- `python3 scripts/ledger_check.py` passes on a fresh init; re-init refuses; a 7-column row is an error.
- `bash tests/selftest.sh` printed `selftest: ok`.

### 2026-11-04T08:23:09Z — Framework docs and README (DOC-001)

- README.md records document roles, bootstrap, canonical skills, design-decision notes.
- docs/01-framework.md cites BP-001, DOC-001, QA-001. plan.md is scope only.

### 2026-11-04T08:23:09Z — project-blueprint kit created (BP-001)

- Canonical skills in skills/: keep-implementation-ledger, code-review, figma-ui.
- template/ plus modules/ (figma, source-docs, python-uv, formatter, database-design).
- init.sh and install-user-skills.sh. Checker in template/scripts/ledger_check.py.
