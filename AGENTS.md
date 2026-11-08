# Ledgibly agent notes

Product architecture: [`plan.md`](plan.md). Tasks, status, and evidence: [`implementation_ledger.md`](implementation_ledger.md) (the only status source). How the software is structured: [`docs/README.md`](docs/README.md). Install and run: [`README.md`](README.md).

## Agent profile

Ledger
- plan `plan.md`; ledger `implementation_ledger.md` (Tasks table, "Chronological UTC Log"); archive `implementation_ledger_archive.md`; docs `docs/README.md`; run guide `README.md`
- ID prefixes: BP DOC QA; archive threshold 100 KB
- verify: `cargo test`; `bash tests/selftest.sh`; `python3 template/scripts/ledger_check.py`. Not required: none
- vocabulary: **in the repo** (exists here) / **not built** (pending)
- docs map: cite IDs inline; coverage is `rg -l '<ID>' docs/`
- pre-checks: none (this repo is the kit, not a product)
- picking: first `pending` row whose dependencies are `done`

Review
- exclude: `.git/`, `target/`, backups under `~/.ledgibly/backups/`, generated `mktemp` dirs from selftest
- trace: `ledgibly init` embeds `template/` + `modules/` → placeholders → `.agents/skills` (optional `.claude/skills` link) → `ledger_check.py` reads the ledger
- extra priorities: no overwrite; skills stay generic (per-repo facts only in Agent profile)

<!-- end:agent-profile -->

Canonical skills live in [`skills/`](skills/). Install with `ledgibly install-skills` (or `cargo run -- install-skills` from this repo).

Do not commit or push unless the user asks.
