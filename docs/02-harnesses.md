# 02. Harnesses

[Index](README.md) · [ledger](../implementation_ledger.md)

Ledgibly’s core is harness-agnostic: one plan, one ledger, docs, an `AGENTS.md` Agent profile, and a copied stdlib `scripts/ledger_check.py`. Canonical skills stay a single tree under `skills/` (DOC-017). Cursor is the **default adapter** today; other coding agents plug in without forking that tree.

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
   ├── also ~/.cursor/skills/     (Cursor today)
   └── also ~/.claude/skills/     (Claude Code; not built — BP-019)
```

## Path table

| Tool | Always-on profile | User skills | Project skills | Extra adapter (pending unless noted) |
|---|---|---|---|---|
| **Cursor** (default) | `AGENTS.md` `## Agent profile` | `~/.cursor/skills` and `~/.agents/skills` | `.cursor/skills` → `.agents/skills`; `.cursor/rules/*.mdc` | Shipped (BP-020 documents it as adapter) |
| **Codex** | `AGENTS.md` (native) | `~/.agents/skills` | `.agents/skills` | Docs only; `~/.codex` not required (BP-023) |
| **OpenCode** | project/`AGENTS.md` as used by the agent | `~/.agents/skills` (also scans `~/.config/opencode/skills`) | `.agents/skills` / `.opencode/skills` | Prefer hub; optional overlay only if needed (BP-022) |
| **Claude Code** | `CLAUDE.md` (or `AGENTS.md` in place of it) | `~/.claude/skills` — **not** documented as reading `~/.agents/skills` | `.claude/skills` | Extra skill home + profile pointer (BP-019, BP-021) |
| **GitHub Copilot** | `.github/copilot-instructions.md` and/or `AGENTS.md` | `~/.agents/skills` / `~/.copilot/skills` | `.agents/skills` / `.github/skills` | Opt-in overlay (BP-024) |

## Design rules

1. **One SKILL.md source.** Adapters may symlink or copy into a second directory; they must not diverge content.
2. **AGENTS.md stays the profile.** Per-repo facts stay in `## Agent profile`. Skills stay generic.
3. **Claude is the odd one out** for skills discovery today: it needs `~/.claude/skills` (and often `.claude/skills` in-repo). That is why BP-019 comes first after this chapter.
4. **OpenCode and Codex already meet the hub.** Prefer documenting that over shipping duplicate trees.
5. **Copilot** can use the hub for skills; repository-wide always-on text may still want `.github/copilot-instructions.md` that points at the same document roles.

Adapter implementation rows: BP-019 (Claude skill home), BP-020 (Cursor as default adapter), BP-021 (Claude init), BP-022 (OpenCode), BP-023 (Codex), BP-024 (Copilot).
