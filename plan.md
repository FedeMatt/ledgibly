# project-blueprint approved plan

This file is the product and architecture baseline. Task status and evidence live in [`implementation_ledger.md`](implementation_ledger.md). How the kit works is in [`docs/README.md`](docs/README.md). Install and run: [`README.md`](README.md).

Do not turn Architecture into a feature changelog. Per-feature work belongs in the ledger as rows. Cite a task ID here only when it names a scope decision.

## Product

A reusable implementation-ledger framework for Cursor-driven repos: templates, canonical user-level skills, optional modules, an init script, and a ledger checker. The same plan / ledger / docs / README workflow can start any future project.

## Architecture

- `skills/` — canonical user-level skills (`keep-implementation-ledger`, `code-review`, `figma-ui`). Installed into `~/.cursor/skills` and `~/.agents/skills`.
- `template/` — files `init.sh` copies into a target repo (AGENTS.md profile, plan, ledger, docs, ponytail, rtk, checker).
- `modules/` — opt-in overlays (`figma`, `source-docs`, `python-uv`, `formatter`, `database-design`).
- `init.sh` — copy, overlay, placeholder replace, `.cursor/skills` symlink, no overwrite.
- `template/scripts/ledger_check.py` — structural ledger checks (stdlib only).
- This repo dogfoods the same document roles with prefixes BP / DOC / QA.

## Development policy

Keep documentation roles strict: plan = scope, ledger = status, docs = behavior, README = install/run.

Verify with `bash tests/selftest.sh` and `python3 template/scripts/ledger_check.py`.

Follow `.cursor/rules/ponytail.mdc`: understand the flow, then write the minimum that works. No runtime dependencies.

## Not in the product

- Migrating an existing product repo onto this kit
- Hosting this as a GitHub template marketplace listing
- A GUI for init

## Out of scope until a ledger task admits them

- Multi-language ledgers
- Automatic Cursor-plan-to-row import beyond the skill's "Plan to rows" step
