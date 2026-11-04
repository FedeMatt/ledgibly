# 01. Framework

[Index](README.md) · [ledger](../implementation_ledger.md)

Ledgibly is a kit, not an application (BP-001, DOC-006). Four files, four jobs:

| File | Job |
|---|---|
| `plan.md` | Scope and architecture |
| `implementation_ledger.md` | Status and evidence (only status source) |
| `docs/` | How the system works; cite IDs inline |
| `README.md` | Install and run |

Per-repo facts live in `AGENTS.md` `## Agent profile`. Skills read only that section.

`init.sh` copies `template/` into a target, overlays `modules/` when `--with` is passed, replaces placeholders only in copied files, and refuses to overwrite (DOC-001, BP-004). Missing `--name` / `--prefixes` / `--verify` / `--with` values are errors. Seed rows use `ART-`; if `--prefixes` omits `ART`, init prepends it (BP-004). ART-001 is seeded `pending` until `plan.md` and `AGENTS.md` have no `<fill me>`. Optional modules: `figma`, `source-docs`, `python-uv`, `formatter`, `database-design`, `rtk` (BP-006). The kit README is install/run (DOC-001, DOC-002, DOC-003, DOC-004). It states that `init.sh` copies `template/`, not this repository's ledger (DOC-005), explains the name as ledger + legible (DOC-007), and leads with a claim, capabilities, and clone-then-init (DOC-008). Motivations include the agent as product owner of the use case (DOC-009). `python3 scripts/ledger_check.py` (after init) or `python3 template/scripts/ledger_check.py` (this repo) checks table shape, IDs, statuses, evidence, dependencies, and log order (QA-001, BP-005, QA-002). `install-user-skills.sh` symlinks the canonical skills into `~/.cursor/skills` and `~/.agents/skills` (BP-002). The kit is MIT-licensed; `init.sh` does not copy `LICENSE` into a target (BP-003).

Ledger rules:

- Stable IDs: `PREFIX-NNN`. Statuses: `pending`, `in_progress`, `blocked`, `done`, `superseded`.
- `done` needs evidence (paths + commands that passed). Never flip `done` back; add a follow-up ID.
- Escape `|` in cells as `\|` (BP-005).
- Cell budgets: description 120, acceptance 200, evidence 300. Narrative goes in the log.
- Log is newest-first UTC. Above 100 KB, propose archiving the oldest log entries verbatim.

Checker errors (nonzero exit): column count, duplicate/missing IDs, bad status, `done`/`blocked` without evidence, `superseded` without a replacement ID outside Dependencies, missing dependencies, log not newest-first.

Checker warnings: cell budgets, size over threshold, `done` IDs not cited in `docs/` (word-boundary match).
