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
   ▼  ledgibly install-skills          → ~/.agents/skills/   hub
   └──  ledgibly install-skills --claude  → ~/.claude/skills/  Claude only
```

Kit writes (DOC-019): hub only by default (BP-028). No `~/.cursor/skills` and no project `.cursor/skills` link (BP-030). `--claude` / `--with claude` are the Claude extra home. Replaced skills go to `~/.ledgibly/backups/<ts>/`, not inside a scan root (BP-029).

## Path table

What each tool **reads** (vendor docs, checked 2026-11-08). Kit writes are the architecture bullets above, not this table.

| Tool | Always-on profile | User skills | Project skills | Adapter |
|---|---|---|---|---|
| **Cursor** (default) | `AGENTS.md` `## Agent profile` | `~/.agents/skills` (also `~/.cursor/skills`, `~/.claude/skills`) | `.agents/skills` (also `.cursor/skills`, `.claude/skills` if present); `.cursor/rules/*.mdc` (BP-027) | Bare `init` writes rules, not a `.cursor/skills` link (BP-020, BP-030) |
| **Codex** | `AGENTS.md` (native) | `~/.agents/skills` | `.agents/skills` | Hub is enough; `~/.codex` not required (BP-023) |
| **OpenCode** | `AGENTS.md` | `~/.agents/skills` (also `~/.config/opencode/skills`, `~/.claude/skills`) | `.agents/skills` (also `.opencode/skills`, `.claude/skills`) | Hub is enough; no `.opencode/skills` overlay (BP-022) |
| **Claude Code** | `CLAUDE.md` first line `@AGENTS.md` (BP-026); else Claude skips `AGENTS.md` | `~/.claude/skills` only | `.claude/skills` → `.agents/skills` | `--with claude`; `install-skills --claude` (BP-021) |
| **GitHub Copilot** | `.github/copilot-instructions.md`; `AGENTS.md` still the profile | `~/.agents/skills` (also `~/.copilot/skills`, `~/.claude/skills`) | `.agents/skills` (also `.github/skills`, `.claude/skills`) | `--with copilot` instructions only (BP-024) |

Sources (2026-11-08): [Cursor skills](https://cursor.com/docs/skills); [Claude CLAUDE.md](https://code.claude.com/docs/en/claude-md); [Claude skills](https://code.claude.com/docs/en/skills); [Codex skills](https://learn.chatgpt.com/docs/build-skills.md), [Codex customization](https://learn.chatgpt.com/docs/customization/overview); [OpenCode skills](https://opencode.ai/docs/skills/); [OpenCode duplicate names](https://github.com/anomalyco/opencode/issues/29950); [Copilot agent skills](https://docs.github.com/en/copilot/concepts/agents/about-agent-skills).

## Design rules

1. **One SKILL.md source.** Adapters may symlink or copy into a second directory; they must not diverge content.
2. **AGENTS.md stays the profile.** Per-repo facts stay in `## Agent profile`. Skills stay generic (BP-031: harness plan todos; figma-design-to-code only if the harness has it).
3. **Claude is the only second home, and it is opt-in.** Claude does not read the hub, so `install-skills --claude` copies `~/.claude/skills` and `--with claude` links `.claude/skills` (BP-019, BP-021, DOC-019). Cursor, OpenCode, and Copilot also scan `.claude` / `~/.claude`, so a Claude extra home can surface each skill twice. For OpenCode, set `OPENCODE_DISABLE_CLAUDE_CODE_SKILLS=1` when using `--with claude`.
4. **OpenCode and Codex already meet the hub.** Document that; do not ship duplicate trees (BP-022, BP-023).
5. **Copilot** already reads the hub for skills. `--with copilot` only adds `.github/copilot-instructions.md` pointing at the same document roles (BP-024).
6. **Always-on rules** stay in `.cursor/rules/*.mdc`. `AGENTS.md` points other harnesses at that glob; Claude imports `@.cursor/rules/ponytail.mdc` (BP-027).

## Smoke check

Selftest asserts files and paths. It does not start Cursor, Claude Code, Codex, OpenCode, or Copilot. Live results live in the ledger Environment evidence `### QA-008` (QA-008). After `uv tool install ledgibly==0.1.3` (BP-034; earlier adapter ship BP-025) or install from this repo, run `ledgibly install-skills` (add `--claude` for Claude Code) and `ledgibly init /path/to/repo --with claude,copilot`, then:

- **Cursor** (BP-020): skills show under user skills; `AGENTS.md` Agent profile loads; `.cursor/rules/ponytail.mdc` is present.
- **Claude Code** (BP-019, BP-021, BP-026): `~/.claude/skills/keep-implementation-ledger/SKILL.md` exists after `install-skills --claude`; `CLAUDE.md` starts with `@AGENTS.md`; project `.claude/skills` follows `.agents/skills`.
- **Codex / OpenCode** (BP-022, BP-023): `AGENTS.md` plus the hub; no extra files required.
- **Copilot** (BP-024): `.github/copilot-instructions.md` is present; skills load from `.agents/skills`.
