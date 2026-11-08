# 02. Harnesses

[Index](README.md) · [ledger](../implementation_ledger.md)

Ledgibly’s core is harness-agnostic: one plan, one ledger, docs, an `AGENTS.md` Agent profile, and a copied stdlib `scripts/ledger_check.py`. Canonical skills stay a single tree under `skills/` (DOC-017). Cursor is the **default adapter** (BP-020); other coding agents plug in without forking that tree.

## Core vs adapters

| Layer | What it is | Portable? |
|---|---|---|
| Core | `plan.md`, `implementation_ledger.md`, `docs/`, `AGENTS.md`, `scripts/ledger_check.py`, `skills/*/SKILL.md` | Yes |
| Skill hub (user) | `~/.agents/skills/<name>/` | Yes — several tools already read it |
| Skill hub (project) | `.agents/skills/` | Yes |
| Adapter | Extra paths or files a specific product needs (Cursor rules, Claude `CLAUDE.md`, Copilot `.github/…`) | Per tool |

Do **not** maintain separate skill copies per product. Install once into the hub; add a second home only when a tool cannot see the hub.

```text
skills/  (canonical in this repo)
   │
   ▼  ledgibly install-skills
~/.agents/skills/   ← hub (Codex, OpenCode, Copilot, Cursor)
   │
   ├── also ~/.cursor/skills/     (Cursor default)
   └── also ~/.claude/skills/     (Claude Code; BP-019)
```

`ledgibly install-skills` copies the three canonical skills into all three user homes (BP-009, BP-019).

## Path table

| Tool | Always-on profile | User skills | Project skills | Adapter |
|---|---|---|---|---|
| **Cursor** (default) | `AGENTS.md` `## Agent profile` | `~/.cursor/skills` and `~/.agents/skills` | `.cursor/skills` → `.agents/skills`; `.cursor/rules/*.mdc` | Bare `init` (BP-020) |
| **Codex** | `AGENTS.md` (native) | `~/.agents/skills` | `.agents/skills` | Hub is enough; `~/.codex` not required (BP-023) |
| **OpenCode** | `AGENTS.md` | `~/.agents/skills` (also `~/.config/opencode/skills`) | `.agents/skills` (also `.opencode/skills`, `.claude/skills`) | Hub is enough; no `.opencode/skills` overlay (BP-022) |
| **Claude Code** | `CLAUDE.md` pointer; `AGENTS.md` stays the profile | `~/.claude/skills` (does not document the hub) | `.claude/skills` → `.agents/skills` | `--with claude` (BP-021) |
| **GitHub Copilot** | `.github/copilot-instructions.md` pointer; `AGENTS.md` still the profile | `~/.agents/skills` (also `~/.copilot/skills`) | `.agents/skills` (also `.github/skills`) | `--with copilot` instructions only (BP-024) |

## Design rules

1. **One SKILL.md source.** Adapters may symlink or copy into a second directory; they must not diverge content.
2. **AGENTS.md stays the profile.** Per-repo facts stay in `## Agent profile`. Skills stay generic.
3. **Claude is the odd one out** for skills discovery: it needs `~/.claude/skills` and `.claude/skills`. That is why install-skills copies there and `--with claude` links the project dir (BP-019, BP-021).
4. **OpenCode and Codex already meet the hub.** Document that; do not ship duplicate trees (BP-022, BP-023).
5. **Copilot** already reads the hub for skills. `--with copilot` only adds `.github/copilot-instructions.md` pointing at the same document roles (BP-024).

## Smoke check

Selftest asserts files and paths. It does not start Cursor, Claude Code, Codex, OpenCode, or Copilot. After `uv tool install ledgibly==0.1.2` (BP-025) or install from this repo, run `ledgibly install-skills` and `ledgibly init /path/to/repo --with claude,copilot`, then:

- **Cursor** (BP-020): skills show under user skills; `AGENTS.md` Agent profile loads; `.cursor/rules/ponytail.mdc` is present.
- **Claude Code** (BP-019, BP-021): `~/.claude/skills/keep-implementation-ledger/SKILL.md` exists; `CLAUDE.md` points at `AGENTS.md`; project `.claude/skills` follows `.agents/skills`.
- **Codex / OpenCode** (BP-022, BP-023): `AGENTS.md` plus the hub; no extra files required.
- **Copilot** (BP-024): `.github/copilot-instructions.md` is present; skills load from `.agents/skills`.
