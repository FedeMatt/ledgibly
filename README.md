# project-blueprint

A reusable **implementation ledger** framework for Cursor-driven repos. Licensed under [MIT](LICENSE).

This repo is two things:

1. A **kit** you copy into a target repo with `init.sh`.
2. A **dogfood** of that kit (this repo's own `AGENTS.md`, `plan.md`, and `implementation_ledger.md`).

## Document roles

One status source. Four files, four jobs:

| File | Job | Change when |
|---|---|---|
| `plan.md` | Scope and architecture | Scope or architecture changes, with user agreement |
| `implementation_ledger.md` | Task table + UTC log. **Only** place status is asserted | Every task start/finish |
| `docs/` | How the system works. Cite task IDs; never restate status | Behavior or structure changes |
| `README.md` | Install and run | Those steps change |

`AGENTS.md` has a `## Agent profile` section. Skills read **only** that section for per-repo facts (paths, ID prefixes, verify commands, docs map, pre-checks). That is what makes the skills generic.

```
plan.md (scope)
    │
    ▼
implementation_ledger.md (status + evidence)
    │
    ├── docs/  (behavior, cite IDs)
    └── README.md (install / run)

AGENTS.md Agent profile ──► keep-implementation-ledger, code-review, figma-ui
python3 scripts/ledger_check.py ──► table shape, IDs, evidence, log order
```

Cursor plan todos become ledger rows with stable IDs **before** code is written.

## Bootstrap a new repo

```bash
./init.sh /path/to/new-repo \
  --name MyApp \
  --prefixes "ART FE BE DOC QA" \
  --verify "cd frontend && npm test" \
  --with figma,source-docs,python-uv,formatter,database-design
```

`--with` is optional. Valid modules: `figma`, `source-docs`, `python-uv`, `formatter`, `database-design`.

`init.sh` refuses to overwrite existing files. It copies `template/`, overlays modules, replaces `{{PROJECT}}` / `{{PREFIXES}}` / `{{VERIFY}}` / `{{DATE}}`, creates `.cursor/skills -> ../.agents/skills`, and prints remaining `<fill me>` lines in `AGENTS.md`.

Then fill the Agent profile (docs map, pre-checks, Review trace) and start work from the first `pending` row.

## Canonical user-level skills

Install (or refresh) the skills this kit owns into your home directories. Existing copies are moved to a timestamped backup first:

```bash
./install-user-skills.sh
# ./install-user-skills.sh --copy    # if Cursor ignores symlinks
```

| Skill | When |
|---|---|
| `keep-implementation-ledger` | Starting or finishing a task, choosing the next task, recording evidence |
| `code-review` | Auditing a change; driven by the profile `Review` block |
| `figma-ui` | Implementing a named Figma frame; driven by the profile `Figma` block |

Repo-local skills (python-uv, source-docs, database-design) land in the target's `.agents/skills/` when you pass `--with`.

## What a new repo gets

```
AGENTS.md                         Agent profile (Ledger + Review)
plan.md                           Product, architecture, policy, out of scope
implementation_ledger.md          Tasks table, environment evidence, milestones, UTC log
implementation_ledger_archive.md  Verbatim overflow
README.md                         Install and run
docs/README.md                    Chapter index
docs/01-architecture.md           First chapter
.cursor/rules/ponytail.mdc        Lazy-senior-dev ladder
.cursor/rules/rtk.mdc             Compact CLI via rtk
.agents/skills/                   Project skills (modules add more)
.cursor/skills -> ../.agents/skills
scripts/ledger_check.py           Structural checks
```

Numbered docs chapters use `[Index](README.md) · [ledger](...)`. Cite IDs inline. Do **not** keep a `Tasks:` header list or a coverage table; `rg -l '<ID>' docs/` and the checker cover that.

## Ledger mechanics (short)

- Stable IDs: `PREFIX-NNN` (next free number for that prefix).
- Statuses: `pending` \| `in_progress` \| `blocked` \| `done` \| `superseded`.
- `done` needs evidence (paths + commands that passed). Never flip `done` back; add a follow-up ID.
- Cell budgets: description 120, acceptance 200, evidence 300. Narrative goes in the log.
- Log is newest-first UTC, inserted directly under the log heading.
- Agents never read the whole ledger; they `rg` one row, open work, or a log heading.
- Above 100 KB, propose archiving the oldest log entries verbatim. Never archive unprompted.

```bash
python3 scripts/ledger_check.py
```

Errors (nonzero exit): column count, duplicate/missing IDs, bad status, `done`/`blocked` without evidence, `superseded` without a replacement ID, missing dependencies, log heading format, log not newest-first.

Warnings: cell budgets, size over threshold, `done` IDs not cited in `docs/`.

## Design decisions

**Kept.** Document roles; Agent profile as the only per-repo input; five statuses with evidence; no `done` rollback; cell budgets; newest-first UTC log; `rg` recipes; verbatim archive; multi-environment evidence subsections; milestones table; checklists next to skills; ponytail; vendor-docs-bound behavior (here: `source-docs`); review reports that name severity, file:line, and the smallest fix.

**Fixed in this kit.**

| Earlier drift | Blueprint rule |
|---|---|
| `AGENTS.md` referenced `@RTK.md`, which does not exist | RTK lives only as `.cursor/rules/rtk.mdc` |
| `.cursor/skills/` and `.agents/skills/` drifted (missing review skill; formatter stored as a rule copy with no `SKILL.md`) | One source: `.agents/skills/`, plus `.cursor/skills` symlink |
| Ledger header has 6 columns, separator has 7 | Checker errors on column mismatch |
| `plan.md` Architecture became a feature changelog of ~60 IDs | Plan template forbids per-feature task lists |
| Task-to-doc index kept twice by hand (`Tasks:` headers, `docs/06` table) | Cite inline; checker warns on `done` IDs with no `docs/` cite |
| Review skill hardcoded a product stack | `code-review` reads the profile `Review` block |
| Ledger near the 100 KB archive threshold with no warning | Checker warns on size |

## Development of this kit

```bash
bash tests/selftest.sh
python3 template/scripts/ledger_check.py
```

Status: [`implementation_ledger.md`](implementation_ledger.md). Scope: [`plan.md`](plan.md).
