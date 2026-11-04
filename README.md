# project-blueprint

A reusable **implementation ledger** framework for Cursor-driven repos. Licensed under [MIT](LICENSE).

Run [`init.sh`](init.sh) to start a new repo with a plan, ledger, docs, agent profile, and checker. The files copied into that repo come from [`template/`](template/), not from this repository's own ledger.

This repository uses the same document roles (`plan.md`, `implementation_ledger.md`, `docs/`) to track work on the kit itself.

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
  --with figma,source-docs,python-uv,formatter,database-design,rtk
```

`--with` is optional. Valid modules: `figma`, `source-docs`, `python-uv`, `formatter`, `database-design`, `rtk`.

`init.sh` refuses to overwrite existing files. It copies `template/`, overlays modules, and replaces `{{PROJECT}}` / `{{PREFIXES}}` / `{{VERIFY}}` / `{{DATE}}` only in those copied files. It creates `.cursor/skills -> ../.agents/skills`, and prints remaining `<fill me>` lines in `AGENTS.md`. Seed rows use `ART-`; if `--prefixes` omits `ART`, init prepends it.

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
.agents/skills/                   Project skills (modules add more)
.cursor/skills -> ../.agents/skills
scripts/ledger_check.py           Structural checks
```

Numbered docs chapters use `[Index](README.md) · [ledger](...)`. Cite IDs inline. Do **not** keep a `Tasks:` header list or a coverage table; `rg -l '<ID>' docs/` and the checker cover that.

## Ledger mechanics (short)

Statuses, cell budgets, and `ledger_check.py` errors and warnings: [docs/01-framework.md](docs/01-framework.md).

```bash
python3 scripts/ledger_check.py
```

## Development of this kit

```bash
bash tests/selftest.sh
python3 template/scripts/ledger_check.py
```

Status: [`implementation_ledger.md`](implementation_ledger.md). Scope: [`plan.md`](plan.md).
