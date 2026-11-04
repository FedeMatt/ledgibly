# 01. Framework

[Index](README.md) · [ledger](../implementation_ledger.md)

project-blueprint is a kit, not an application (BP-001). Four files, four jobs:

| File | Job |
|---|---|
| `plan.md` | Scope and architecture |
| `implementation_ledger.md` | Status and evidence (only status source) |
| `docs/` | How the system works; cite IDs inline |
| `README.md` | Install and run |

Per-repo facts live in `AGENTS.md` `## Agent profile`. Skills read only that section.

`init.sh` copies `template/` into a target, overlays `modules/` when `--with` is passed, replaces placeholders, and refuses to overwrite (DOC-001). Public-facing files name no private repos or product stacks (DOC-002). `python3 scripts/ledger_check.py` (after init) or `python3 template/scripts/ledger_check.py` (this repo) checks table shape, IDs, statuses, evidence, dependencies, and log order (QA-001). `install-user-skills.sh` symlinks the canonical skills into `~/.cursor/skills` and `~/.agents/skills` (BP-002). The kit is MIT-licensed; `init.sh` does not copy `LICENSE` into a target (BP-003).
