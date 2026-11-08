# Ledger reference

Replace `<ledger>` with the profile's ledger path and `FE` with the relevant prefix.

## Recipes

```bash
# One row by ID
rg -n '^\| FE-092 \|' <ledger>

# Open work (any column order); trim wide rows
rg -n '^\| [A-Z]+-[0-9]+ \|.*\| (in_progress|blocked|pending) \|' <ledger> | cut -c1-200

# Next free ID for a prefix (highest existing; add 1)
rg -o '^\| FE-[0-9]+' <ledger> | sort -V | tail -1

# Every log heading for one task
rg -n '^### .*FE-092' <ledger>

# Five newest log headings; then Read one entry with offset/limit
rg -n '^### 20' <ledger> | head -5

# UTC timestamp for a new log entry
date -u +%Y-%m-%dT%H:%M:%SZ

# Ledger size against the archive threshold
wc -c <ledger>

# Docs that cite an ID (no hand-kept index)
rg -l 'FE-092' docs/

# Structural check
python3 scripts/ledger_check.py
```

## Ledger skeleton

```markdown
# <Project> Implementation Ledger

Living task list. Scope: [plan.md](plan.md). Statuses: `pending`, `in_progress`, `blocked`, `done`, `superseded`. Older log entries and trimmed evidence: [archive](implementation_ledger_archive.md).

## Tasks

| ID | Description | Dependencies | Status | Acceptance condition | Evidence |
|---|---|---|---|---|---|

## Environment evidence

Multi-host or multi-OS notes live here as `### <ID>` subsections, not in the table cell.

## Milestones

| Gate | Tasks |
|---|---|
| 1. Approved plan | ART-001 |
| 2. Operational ledger | ART-002 |

## Chronological UTC Log

Newest first. Older entries: [archive](implementation_ledger_archive.md).
```

## Row

```markdown
| FE-097 | <≤120 chars> | FE-096 | pending | <≤200 chars, testable> | <≤300 chars: paths; commands; log 2026-10-01T02:00:00Z> |
```

Escape `|` in cells as `\|`.

## Log entry

Insert directly under the log heading (newest first). At most 6 bullets.

```markdown
### 2026-10-01T02:00:00Z — <what changed> (FE-097)

- <what was done, with repo paths>
- <commands that passed>
- <what remains, with task IDs>
```

One entry may list several IDs: `(FE-097, FE-098)`.

## Archive file

Verbatim moves only. Newest first within each section.

```markdown
# <Project> Implementation Ledger Archive

Moved verbatim from [implementation_ledger.md](implementation_ledger.md). Status lives only in the ledger.

## Evidence detail

### FE-012

<text removed from the row cell, verbatim>

## Chronological UTC Log (archived)

### <oldest entries kept in newest-first order>
```

In the row, replace the trimmed text with `detail: [archive](implementation_ledger_archive.md#fe-012)`.

## Agent profile (AGENTS.md section)

Omit lines that do not apply. Skills read only this section for per-repo facts.

```markdown
## Agent profile

Ledger
- plan `plan.md`; ledger `implementation_ledger.md` (Tasks table, "Chronological UTC Log"); archive `implementation_ledger_archive.md`; docs `docs/README.md`; run guide `README.md`
- ID prefixes: ART FE BE DOC QA; archive threshold 100 KB
- verify: `npm test`; `uv run pytest`; `python3 scripts/ledger_check.py`. Not required: <slow or unavailable commands>
- vocabulary: **in the repo** (exists here) / **not built** (pending)
- docs map: cite IDs inline; coverage is `rg -l '<ID>' docs/`
- pre-checks: <area> -> <skill>
- picking: <project-specific rule for choosing the next task>

Review
- exclude: <generated output, vendored deps, lockfiles, local databases>
- trace: <UI action -> service -> persistence -> returned state>
- extra priorities: <or omit>

Figma
- fileKey `<key>`; page `<id>`; canvas <w>x<h>; inventory `<path>`
- target: <stack and version>; screens `<dir>`; config `<file>`; forbidden: <web/Tailwind, etc.>
- behavior source: `<path>` (wins over Figma on behavior)
- out of scope: hidden frames, <areas>
- visual check: `<command>` when <condition>

Sources
- index `docs/sources/README.md`
- rule: do not invent names or values absent from the sources
```
