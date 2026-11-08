# 01. Framework

[Index](README.md) · [ledger](../implementation_ledger.md)

Ledgibly is a kit distributed as a Rust CLI on PyPI (maturin `bindings = "bin"`), not a library product repos import (BP-001, DOC-006, BP-007, BP-010). Four files, four jobs:

| File | Job |
|---|---|
| `plan.md` | Scope and architecture |
| `implementation_ledger.md` | Status and evidence (only status source) |
| `docs/` | How the system works; cite IDs inline |
| `README.md` | Install and run |

Per-repo facts live in `AGENTS.md` `## Agent profile`. Skills read only that section.

`ledgibly init` embeds `template/` into a target, overlays `modules/` when `--with` is passed, replaces placeholders only in copied files, and refuses to overwrite (DOC-001, BP-004, BP-008). Missing `--name` / `--prefixes` / `--verify` / `--with` values are errors. Seed rows use `ART-`; if `--prefixes` omits `ART`, init prepends it (BP-004). ART-001 is seeded `pending` until `plan.md` and `AGENTS.md` have no `<fill me>`. Optional modules: `figma`, `source-docs`, `python-uv`, `formatter`, `database-design`, `rtk` (BP-006). The kit README is install/run (DOC-001, DOC-002, DOC-003, DOC-004). It states that init copies `template/`, not this repository's ledger (DOC-005), explains the name as ledger + legible (DOC-007), and leads with a claim, capabilities, and `uv tool install` then init (DOC-008, DOC-013). Motivations include the agent as product owner of the use case (DOC-009). The README shows `brand/ledgibly.png` above the title (DOC-010), plus Docs/Template/Skills/Modules nav and an MIT badge (DOC-011). The opener prose was grammar-corrected without changing meaning (DOC-012). `python3 scripts/ledger_check.py` (after init) or `python3 template/scripts/ledger_check.py` (this repo) checks table shape, IDs, statuses, evidence, dependencies, and log order (QA-001, BP-005, QA-002). Targets keep a copied stdlib checker and do not depend on the `ledgibly` package at runtime (DOC-013). `ledgibly install-skills` copies the canonical skills into `~/.cursor/skills` and `~/.agents/skills` (BP-002, BP-009). CI runs `cargo test` and `bash tests/selftest.sh` on PRs; tag builds produce maturin platform wheels without auto-publish (QA-003). The kit is MIT-licensed; init does not copy `LICENSE` into a target (BP-003).

Ledger rules:

- Stable IDs: `PREFIX-NNN`. Statuses: `pending`, `in_progress`, `blocked`, `done`, `superseded`.
- `done` needs evidence (paths + commands that passed). Never flip `done` back; add a follow-up ID.
- Escape `|` in cells as `\|` (BP-005).
- Cell budgets: description 120, acceptance 200, evidence 300. Narrative goes in the log.
- Log is newest-first UTC. Above 100 KB, propose archiving the oldest log entries verbatim.

Checker errors (nonzero exit): column count, duplicate/missing IDs, bad status, `done`/`blocked` without evidence, `superseded` without a replacement ID outside Dependencies, missing dependencies, log not newest-first.

Checker warnings: cell budgets, size over threshold, `done` IDs not cited in `docs/` (word-boundary match).
