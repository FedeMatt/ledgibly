# Ledgibly Implementation Ledger

Living task list. Scope: [plan.md](plan.md). Statuses: `pending`, `in_progress`, `blocked`, `done`, `superseded`. Older log entries and trimmed evidence: [archive](implementation_ledger_archive.md).

## Tasks

| ID | Description | Dependencies | Status | Acceptance condition | Evidence |
|---|---|---|---|---|---|
| DOC-009 | README motivations: agents track work, tokens, bugs, product-owner role | DOC-008 | done | Motivations say the ledger is how the agent tracks the use case | README.md; docs/01-framework.md; `python3 template/scripts/ledger_check.py`; log 2026-11-04T14:07:57Z |
| DOC-008 | Rewrite kit README: claim, what you can do, start here | DOC-007 | done | README leads with claim, capabilities, motivations, then clone and init | README.md; docs/01-framework.md; `python3 template/scripts/ledger_check.py`; log 2026-11-04T14:04:11Z |
| DOC-007 | README opener: name meaning and how the kit works | DOC-006 | done | README opener explains Ledgibly (ledger + legible) and one-status-source workflow | README.md; docs/01-framework.md; `python3 template/scripts/ledger_check.py`; log 2026-11-04T13:57:54Z |
| DOC-006 | Rename kit to Ledgibly in current docs, LICENSE, and GitHub repo | DOC-005 | done | Titles and LICENSE say Ledgibly; GitHub repo is ledgibly; selftest temp dir uses ledgibly | README.md; LICENSE; docs/01-framework.md; `gh repo view` FedeMatt/ledgibly; `bash tests/selftest.sh`; log 2026-11-04T13:56:12Z |
| DOC-005 | Rewrite kit README opener; kit vs template vs this repo's ledger | DOC-004 | done | README opener names kit vs template vs this repo's ledger; README and plan.md do not call this repo a second product | README.md; plan.md; docs/01-framework.md; `python3 template/scripts/ledger_check.py`; log 2026-11-04T13:47:11Z |
| BP-004 | init.sh: placeholders only in copied files; option values; prepend ART | BP-001 | done | init does not rewrite pre-existing files; missing option values error; PREFIXES always includes ART | init.sh; template/implementation_ledger.md; `bash tests/selftest.sh`; log 2026-11-04T09:17:23Z |
| BP-005 | ledger_check: escaped pipes, superseded replacement, docs word-boundary | BP-001 | done | escaped pipes in cells parse; superseded ignores Dependencies; docs citations use word boundaries | template/scripts/ledger_check.py; skills/keep-implementation-ledger/reference.md; `bash tests/selftest.sh`; log 2026-11-04T09:17:23Z |
| BP-006 | Move rtk.mdc to modules/rtk; add Use-when to python-uv and database-design | BP-001 | done | Bare init has no rtk.mdc; --with rtk adds it; those two skills have Use when in description | modules/rtk/; python-uv and database-design SKILL.md; `bash tests/selftest.sh`; log 2026-11-04T09:17:23Z |
| QA-002 | selftest covers existing-dir init, escaped pipes, failure fixtures, rtk | BP-004, BP-005, BP-006 | done | selftest asserts byte-identical user file, escaped pipe pass, three checker failures, rtk opt-in | tests/selftest.sh; `bash tests/selftest.sh`; log 2026-11-04T09:17:23Z |
| DOC-004 | Move checker error list into docs; README links it; cite new IDs | BP-004, BP-005, BP-006 | done | docs/01-framework.md lists checker errors/warnings and rtk module; README has no error list | docs/01-framework.md; README.md; plan.md; `bash tests/selftest.sh`; log 2026-11-04T09:17:23Z |
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

### 2026-11-04T14:07:57Z — Motivations: agent as product owner of the use case (DOC-009)

- README.md Motivations: one row and the recent log, fewer tokens, fewer invented stories, the assistant owns the use case.
- docs/01-framework.md cites DOC-009.

### 2026-11-04T14:04:11Z — README rewritten in ds4 voice (DOC-008)

- README.md leads with a claim, what you can do, motivations, how to use the kit, then clone and init. Spec tables come after. ASCII diagram removed.
- docs/01-framework.md cites DOC-008.

### 2026-11-04T13:57:54Z — README opener names the idea and the workflow (DOC-007)

- README.md opener: Ledgibly is ledger + legible; one ledger table is the only status source; `init.sh` copies `template/`.
- docs/01-framework.md cites DOC-007.

### 2026-11-04T13:56:12Z — Kit renamed to Ledgibly (DOC-006)

- Current titles, LICENSE, template, and keep-implementation-ledger skill say Ledgibly. selftest temp dirs use `ledgibly`.
- GitHub repo is `FedeMatt/ledgibly` (`gh repo rename ledgibly --yes`). Older log entries still say project-blueprint.

### 2026-11-04T13:47:11Z — README opener names kit vs template (DOC-005)

- README.md opener: `init.sh` copies `template/`, not this repository's ledger. Same document roles track kit work.
- plan.md Architecture uses the same wording. docs/01-framework.md cites DOC-005.

### 2026-11-04T09:17:23Z — Checker error list lives in docs (DOC-004)

- docs/01-framework.md lists checker errors/warnings, escaped pipes, rtk module, and init placeholder scope.
- README.md Bootstrap lists rtk; Ledger mechanics links the docs chapter. plan.md Architecture lists rtk as a module.

### 2026-11-04T09:17:23Z — selftest covers init and checker edge cases (QA-002)

- tests/selftest.sh: existing-dir `src/t.hbs` stays byte-identical; escaped pipe evidence passes; missing `--name` value fails; ART prepended to prefixes.
- Failure fixtures: 7-column row, `done` without evidence, superseded with only a dependency, log not newest-first.
- Bare init has no rtk.mdc; `--with rtk` adds it. `bash tests/selftest.sh` printed `selftest: ok`.

### 2026-11-04T09:17:23Z — rtk is an opt-in module (BP-006)

- Moved template/.cursor/rules/rtk.mdc to modules/rtk/.cursor/rules/rtk.mdc.
- python-uv-workflow and database-design skill descriptions include "Use when".

### 2026-11-04T09:17:23Z — ledger_check handles escaped pipes and docs IDs (BP-005)

- split_row splits on unescaped `|`; superseded replacement IDs ignore Dependencies; docs citation uses word boundaries.
- reference.md: escape `|` in cells as `\|`.

### 2026-11-04T09:17:23Z — init.sh only rewrites copied files (BP-004)

- Placeholders run only on paths copy_tree wrote. Missing option values print `needs a value`. PREFIXES without ART gets ART prepended.
- Seeded ART-001 is pending until plan.md and AGENTS.md have no `<fill me>`.

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
