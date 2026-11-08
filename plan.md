# Ledgibly approved plan

This file is the product and architecture baseline. Task status and evidence live in [`implementation_ledger.md`](implementation_ledger.md). How the kit works is in [`docs/README.md`](docs/README.md). Install and run: [`README.md`](README.md).

Do not turn Architecture into a feature changelog. Per-feature work belongs in the ledger as rows. Cite a task ID here only when it names a scope decision.

## Product

A reusable implementation-ledger framework for Cursor-driven repos: templates, canonical user-level skills, optional modules, a Rust CLI installer, and a ledger checker. The same plan / ledger / docs / README workflow can start any future project.

## Architecture

- `skills/` — canonical user-level skills (`keep-implementation-ledger`, `code-review`, `figma-ui`). Installed into `~/.cursor/skills` and `~/.agents/skills` by `ledgibly install-skills`.
- `template/` — files `ledgibly init` copies into a target repo (AGENTS.md profile, plan, ledger, docs, ponytail, checker).
- `modules/` — opt-in overlays (`figma`, `source-docs`, `python-uv`, `formatter`, `database-design`, `rtk`).
- Rust CLI (`ledgibly`) — maturin `bindings = "bin"` wheel published to PyPI as platform wheels; embeds `template/`, `modules/`, and `skills/`; subcommands `init` and `install-skills`.
- `template/scripts/ledger_check.py` — structural ledger checks (stdlib only). Copied into targets; product repos do not depend on the `ledgibly` package at runtime.
- This repository tracks kit work with the same document roles and prefixes BP / DOC / QA.

## Development policy

Keep documentation roles strict: plan = scope, ledger = status, docs = behavior, README = install/run.

Verify with `cargo test`, `bash tests/selftest.sh`, and `python3 template/scripts/ledger_check.py`.

Follow `.cursor/rules/ponytail.mdc`: understand the flow, then write the minimum that works. Target repos have no runtime dependencies (the copied checker is stdlib-only). The kit CLI is a Rust binary with build-time crates only.

## Not in the product

- Migrating an existing product repo onto this kit
- Hosting this as a GitHub template marketplace listing
- A GUI for init
- Target repos importing or depending on `ledgibly` for verify

## Out of scope until a ledger task admits them

- Multi-language ledgers
- Automatic Cursor-plan-to-row import beyond the skill's "Plan to rows" step
