# {{PROJECT}} approved plan

This file is the product and architecture baseline. Task status and evidence live in [`implementation_ledger.md`](implementation_ledger.md). How the system works is in [`docs/README.md`](docs/README.md). Install and run: [`README.md`](README.md).

Do not turn Architecture into a feature changelog. Per-feature work belongs in the ledger as rows. Cite a task ID here only when it names a scope decision.

## Product

<fill me: one paragraph — what it is, who it is for, which platforms>

## Architecture

<fill me: major parts and how they connect. Names and constraints, not a task list.>

## Development policy

Keep documentation roles strict: plan = scope, ledger = status, docs = behavior, README = install/run.

Verify with `{{VERIFY}}`. If `scripts/ledger_check.py` exists, run it too.

Follow `.cursor/rules/ponytail.mdc`: understand the flow, then write the minimum that works.

## Not in the product

<fill me: explicit non-goals that will not ship>

## Out of scope until a ledger task admits them

<fill me: ideas that need a row before anyone implements them>
