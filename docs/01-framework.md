# 01. Framework

[Index](README.md) · [ledger](../implementation_ledger.md)

Ledgibly is a kit distributed as a Rust CLI on PyPI (maturin `bindings = "bin"`), not a library product repos import (BP-001, DOC-006, BP-007, BP-010, BP-011). Four files, four jobs:

| File | Job |
|---|---|
| `plan.md` | Scope and architecture |
| `implementation_ledger.md` | Status and evidence (only status source) |
| `docs/` | How the system works; cite IDs inline |
| `README.md` | Install and run |

Per-repo facts live in `AGENTS.md` `## Agent profile`. Skills read only that section.

`ledgibly init` embeds `template/` into a target, overlays `modules/` when `--with` is passed, replaces placeholders only in copied files, and refuses to overwrite (DOC-001, BP-004, BP-008). Missing `--name` / `--prefixes` / `--verify` / `--with` values are errors. Seed rows use `ART-`; if `--prefixes` omits `ART`, init prepends it (BP-004). ART-001 is seeded `pending` until `plan.md` and `AGENTS.md` have no `<fill me>`. Optional modules: `figma`, `source-docs`, `python-uv`, `formatter`, `database-design`, `rtk`, plus harness overlays `claude` and `copilot` (BP-006, BP-021, BP-024). Cursor is the default adapter: bare init writes `.cursor/rules/ponytail.mdc` and does not symlink `.cursor/skills` (BP-020, BP-030). The kit README is install/run (DOC-001, DOC-002, DOC-003, DOC-004, DOC-018). It states that init copies `template/`, not this repository's ledger (DOC-005), explains the name as ledger + legible (DOC-007), and leads with a claim, capabilities, and `uv tool install` then init (DOC-008, DOC-013). Motivations include the agent as product owner of the use case (DOC-009). The README shows the Ledgibly logo above the title via an absolute GitHub raw URL so PyPI renders it (DOC-010, DOC-016; shipped on PyPI in v0.1.1, BP-018), plus the tagline “A ledger kit for coding agents,” Docs/Template/Skills/Modules nav, a PyPI version badge (DOC-014, DOC-015, BP-035), and an MIT badge (DOC-011). The short opener (ledger + legible) was restored after the Rust rewrite dropped it (DOC-015); earlier opener grammar fixes remain (DOC-012). Harness paths live in [02-harnesses.md](02-harnesses.md) (DOC-017, DOC-018, DOC-019). Architecture: hub-only user skills; Cursor does not get a second home or a `.cursor/skills` symlink (DOC-019). `python3 scripts/ledger_check.py` (after init) or `python3 template/scripts/ledger_check.py` (this repo) checks table shape, IDs, statuses, evidence, dependencies, and log order (QA-001, BP-005, QA-002). Targets keep a copied stdlib checker and do not depend on the `ledgibly` package at runtime (DOC-013). `ledgibly install-skills` copies the canonical skills into `~/.agents/skills` only; `--claude` adds `~/.claude/skills` (BP-002, BP-009, BP-019, BP-028). CI runs `cargo test --locked` on Linux, macOS, and Windows, selftest on Linux and macOS, and a Windows `init --with claude` path check (QA-003, QA-005, QA-006); asserts `cargo package --list` includes `template/.cursor/rules/ponytail.mdc` (QA-005); tag builds produce maturin platform wheels and Trusted Publishing uploads to PyPI (BP-011); the release gate compares the `v*` tag to `cargo metadata` version before wheel jobs (QA-007). Patch 0.1.3 ships hub-only install-skills and the review checker/CI fixes (BP-034). The kit is MIT-licensed; init does not copy `LICENSE` into a target (BP-003).

Ledger rules:

- Stable IDs: `PREFIX-NNN`. Statuses: `pending`, `in_progress`, `blocked`, `done`, `superseded`.
- `done` needs evidence (paths + commands that passed). Never flip `done` back; add a follow-up ID.
- Escape `|` in cells as `\|` (BP-005).
- Cell budgets: description 120, acceptance 200, evidence 300. Narrative goes in the log.
- Log is newest-first UTC. A heading may list several IDs: `(FE-097, FE-098)` (BP-033). Above 100 KB, propose archiving the oldest log entries verbatim.

Checker errors (nonzero exit): column count, duplicate/missing IDs, bad status, `done`/`blocked` without evidence, `superseded` without a replacement ID outside Dependencies, missing dependencies, log not newest-first.

Checker warnings: cell budgets, size over threshold, `done` IDs not cited in `docs/` (word-boundary match), `done` depends on a row that is not `done` (BP-032).
