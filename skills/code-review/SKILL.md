---
name: code-review
description: Review repository changes against the project's Agent profile, plan, and implementation ledger. Use when auditing the repository, reviewing a change, or deciding whether work is ready to continue.
---

# Code review

Per-repo facts live in the `## Agent profile` section of AGENTS.md. Read the `Ledger` and `Review` blocks first. If `Review` is missing or still contains `<fill me>`, use the default priorities below and skip stack-specific extras.

## Establish scope

1. Read `AGENTS.md`, `plan.md`, and the relevant rows in the ledger (one-row `rg`, not the whole file).
2. Read the matching chapter under `docs/` (cite IDs; do not treat docs as status).
3. Run profile pre-checks when the change touches those areas (Figma, source-docs, and so on).
4. Review the hand-written source and tests. Exclude whatever the `Review` block lists (generated output, vendored deps, virtualenvs, caches, lockfiles, binaries, local databases) unless the change specifically concerns them.

## Review priorities

Default order. Profile `extra priorities` append after these; they do not replace them.

- **Correctness:** trace the path in the `Review` `trace` line (for example UI action → service → persistence → returned state). Flag broken transitions and untested branches on that path.
- **Safety:** exposed credentials, unsafe default secrets, unrestricted network surfaces, path handling problems, raw internal exceptions returned to clients.
- **Data integrity:** validation, uniqueness, transaction rollback, cascade behavior, concurrent allocation.
- **Portability:** honor the profile (desktop forks, OS-specific modules, no host-only imports in shared code).
- **Fidelity to sources:** if a `Sources` or device/spec block exists, do not invent names, opcodes, or values absent from those files.
- **Maintainability:** flag large units only when size causes a concrete defect, duplicated state transition, or untestable behavior.
- **Hygiene:** tracked build artifacts, logs, local machine configuration, databases, secrets, stale ledger evidence, or status restated outside the ledger.

## Verification

Use the environment's installed tools without upgrading dependencies. Run the profile `verify` commands. Skip commands the profile marks as not required. If `scripts/ledger_check.py` exists, run it.

Report only reproducible findings. For each finding, give severity, an exact file and line, the failure mode, and the smallest credible fix. Separate verified defects from test gaps and future-scope items.

See [checklist.md](checklist.md).
