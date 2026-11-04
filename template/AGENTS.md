# {{PROJECT}} agent notes

Product architecture: [`plan.md`](plan.md). Tasks, status, and evidence: [`implementation_ledger.md`](implementation_ledger.md) (the only status source). How the software is structured: [`docs/README.md`](docs/README.md). Install and run: [`README.md`](README.md).

## Agent profile

Ledger
- plan `plan.md`; ledger `implementation_ledger.md` (Tasks table, "Chronological UTC Log"); archive `implementation_ledger_archive.md`; docs `docs/README.md`; run guide `README.md`
- ID prefixes: {{PREFIXES}}; archive threshold 100 KB
- verify: `{{VERIFY}}`. Not required: <fill me>
- vocabulary: **in the repo** (exists here) / **not built** (pending)
- docs map: cite IDs inline; coverage is `rg -l '<ID>' docs/`
- pre-checks: <fill me: area -> skill>
- picking: first `pending` row whose dependencies are `done`

Review
- exclude: <fill me: generated output, vendored deps, lockfiles, local databases>
- trace: <fill me: UI action -> service -> persistence -> returned state>
- extra priorities: <fill me, or omit>

<!-- end:agent-profile -->

Project skills live in [`.agents/skills/`](.agents/skills/). `.cursor/skills` is a symlink to that directory. User-level skills (`keep-implementation-ledger`, `code-review`, `figma-ui`) come from the Ledgibly kit.

Do not commit or push unless the user asks.
