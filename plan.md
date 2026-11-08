# Ledgibly approved plan

This file is the product and architecture baseline. Task status and evidence live in [`implementation_ledger.md`](implementation_ledger.md). How the kit works is in [`docs/README.md`](docs/README.md). Install and run: [`README.md`](README.md).

Do not turn Architecture into a feature changelog. Per-feature work belongs in the ledger as rows. Cite a task ID here only when it names a scope decision.

## Product

A reusable implementation-ledger framework for coding-agent repos: templates, canonical user-level skills, optional modules, a Rust CLI installer, and a ledger checker. The same plan / ledger / docs / README workflow can start any future project. **Cursor is the default adapter**; OpenCode, Codex, Claude Code, and GitHub Copilot are admitted as opt-in adapters (see [`docs/02-harnesses.md`](docs/02-harnesses.md)).

## Architecture

- **Core** — `plan.md`, `implementation_ledger.md`, `docs/`, `AGENTS.md`, copied stdlib `scripts/ledger_check.py`, and canonical `skills/` (one tree, not forked per product).
- **Skill hub** — `~/.agents/skills` (user) and `.agents/skills` (project). Several agents already load this path. `install-skills` writes only the user hub; `--claude` is the only extra user home (`~/.claude/skills`), because Claude Code does not read the hub (DOC-019).
- `template/` — files `ledgibly init` copies into a target repo (Agent profile, plan, ledger, docs, ponytail, checker).
- `modules/` — opt-in overlays (`figma`, `source-docs`, `python-uv`, `formatter`, `database-design`, `rtk`, and future harness overlays).
- Rust CLI (`ledgibly`) — maturin `bindings = "bin"` wheel published to PyPI as platform wheels; embeds `template/`, `modules/`, and `skills/`; subcommands `init` and `install-skills`.
- **Cursor adapter (default)** — copies `.cursor/rules`. Does not write `~/.cursor/skills` or symlink `.cursor/skills`; Cursor already reads the hub (DOC-019). `--with claude` still links `.claude/skills` → `.agents/skills`.
- This repository tracks kit work with the same document roles and prefixes BP / DOC / QA.

## Development policy

Keep documentation roles strict: plan = scope, ledger = status, docs = behavior, README = install/run.

Verify with `cargo test`, `bash tests/selftest.sh`, and `python3 template/scripts/ledger_check.py`.

Follow `.cursor/rules/ponytail.mdc`: understand the flow, then write the minimum that works. Target repos have no runtime dependencies (the copied checker is stdlib-only). The kit CLI is a Rust binary with build-time crates only. Do not duplicate skill content per harness.

## Not in the product

- Migrating an existing product repo onto this kit
- Hosting this as a GitHub template marketplace listing
- A GUI for init
- Target repos importing or depending on `ledgibly` for verify

## Out of scope until a ledger task admits them

- Multi-language ledgers
- Automatic Cursor-plan-to-row import beyond the skill's "Plan to rows" step
