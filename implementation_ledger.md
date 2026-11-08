# Ledgibly Implementation Ledger

Living task list. Scope: [plan.md](plan.md). Statuses: `pending`, `in_progress`, `blocked`, `done`, `superseded`. Older log entries and trimmed evidence: [archive](implementation_ledger_archive.md).

## Tasks

| ID | Description | Dependencies | Status | Acceptance condition | Evidence |
|---|---|---|---|---|---|
| BP-035 | Cache-bust the PyPI shields badge so GitHub Camo shows the current version | BP-034 | done | GitHub README PyPI badge URL ends in .svg and Shields reports the published version | README.md img.shields.io/pypi/v/ledgibly.svg; curl shields SVG v0.1.3; log 2026-11-08T10:01:00Z |
| BP-034 | Patch release v0.1.3 so PyPI ships hub-only adapters and review fixes | BP-030, BP-033, QA-007 | done | pypi.org/project/ledgibly/0.1.3 exists | tag v0.1.3; Release 42469027727; .github/workflows/release.yml; log 2026-11-08T09:55:57Z |
| DOC-019 | Harness docs cite vendor URLs; plan.md records hub-only skills and no .cursor/skills link | | done | docs/02-harnesses.md path table cites Cursor, Claude, Codex, OpenCode, Copilot URLs with checked date; plan.md Architecture matches BP-028 and BP-030 | plan.md; docs/02-harnesses.md; docs/01-framework.md; `python3 template/scripts/ledger_check.py`; log 2026-11-08T09:14:37Z |
| BP-026 | CLAUDE.md imports @AGENTS.md instead of linking to it | | done | First line of modules/claude/CLAUDE.md is @AGENTS.md; selftest asserts it | modules/claude/CLAUDE.md; tests/selftest.sh; `bash tests/selftest.sh`; log 2026-11-08T09:14:56Z |
| BP-027 | Always-on .cursor/rules reach Codex, OpenCode, Copilot, and Claude | BP-026 | done | template/AGENTS.md tells agents to read .cursor/rules/*.mdc; CLAUDE.md imports @.cursor/rules/ponytail.mdc; selftest checks both | template/AGENTS.md; modules/claude/CLAUDE.md; tests/selftest.sh; `bash tests/selftest.sh`; log 2026-11-08T09:15:18Z |
| BP-028 | install-skills writes only ~/.agents/skills; --claude adds ~/.claude/skills | DOC-019 | done | Fake-HOME selftest: no ~/.cursor/skills; --claude writes ~/.claude/skills; leftover ~/.cursor kit skill backed up; README Start here and main.rs help match | src/skills.rs; src/main.rs; README.md; tests/selftest.sh; `cargo test`; `bash tests/selftest.sh`; log 2026-11-08T09:16:16Z |
| BP-029 | install-skills writes backups outside folders tools scan | BP-028 | done | Backups land in ~/.ledgibly/backups/<ts>/; selftest finds no .backup-* under any skill folder | src/skills.rs; tests/selftest.sh; AGENTS.md; `cargo test`; `bash tests/selftest.sh`; log 2026-11-08T09:16:50Z |
| BP-030 | Bare init stops creating the .cursor/skills symlink | DOC-019 | done | Bare selftest has no .cursor/skills; --with claude still links .claude/skills; conflict check, template/AGENTS.md, README What a new repo gets match | src/init.rs; template/AGENTS.md; README.md; tests/selftest.sh; `cargo test`; `bash tests/selftest.sh`; log 2026-11-08T09:17:29Z |
| BP-031 | Skills and template/AGENTS.md use wording that fits any harness | BP-030 | done | keep-implementation-ledger says harness plan todos; figma-ui loads figma-design-to-code only if the harness provides it | skills/keep-implementation-ledger/SKILL.md; skills/figma-ui/SKILL.md; README.md; `bash tests/selftest.sh`; log 2026-11-08T09:23:17Z |
| BP-032 | ledger_check warns when a done row depends on a row that is not done | | done | Selftest fixture prints the warning; this ledger still has 0 errors | template/scripts/ledger_check.py; tests/selftest.sh; docs/01-framework.md; `bash tests/selftest.sh`; `python3 template/scripts/ledger_check.py`; log 2026-11-08T09:23:56Z |
| BP-033 | ledger_check accepts (A-1, A-2) in a log heading | | done | Each ID in the heading is checked against the table; selftest covers it; reference.md shows the form | template/scripts/ledger_check.py; skills/keep-implementation-ledger/reference.md; tests/selftest.sh; `bash tests/selftest.sh`; log 2026-11-08T09:24:32Z |
| QA-006 | CI runs cargo test --locked on Linux, macOS, Windows; selftest on Linux and macOS | BP-030 | done | Windows job runs init --with claude and confirms CLAUDE.md and .claude/skills exist | .github/workflows/ci.yml linux/macos/windows; `cargo test --locked`; docs/01-framework.md; log 2026-11-08T09:24:51Z |
| QA-007 | Release fails early when the tag does not match the Cargo.toml version | | done | release.yml step compares GITHUB_REF_NAME with cargo metadata version before any wheel job runs | .github/workflows/release.yml gate job; `cargo metadata` version 0.1.2; docs/01-framework.md; log 2026-11-08T09:25:11Z |
| QA-008 | Manual smoke: each harness loads the profile and shared skills after a fresh init | BP-026, BP-027, BP-028, BP-030 | done | Environment evidence has a ### QA-008 entry per harness with observed load (or blocked if not installed) | Environment evidence ### QA-008; /tmp/ledgibly-qa008 init; Cursor observed; Claude Codex OpenCode Copilot blocked not installed; log 2026-11-08T09:25:27Z |
| BP-025 | Patch release v0.1.2 so PyPI ships adapters and README | BP-024, DOC-018 | done | pypi.org/project/ledgibly/0.1.2 exists | tag v0.1.2; Release 42464038529; pypi 0.1.2 description has coding-agents tagline; log 2026-11-08T08:45:59Z |
| DOC-018 | README, CLI about, and package description: Cursor is default adapter not Cursor-only | DOC-017 | done | README and Cargo/PyPI description admit other harnesses; Cursor remains default | README.md; Cargo.toml; pyproject.toml; src/main.rs; docs/01-framework.md; `python3 template/scripts/ledger_check.py`; log 2026-11-08T08:37:29Z |
| DOC-016 | README logo uses absolute GitHub raw URL so PyPI renders it | DOC-015 | done | PyPI project page shows brand/ledgibly.png after next release | README.md absolute raw URL; docs/01-framework.md; `python3 template/scripts/ledger_check.py`; log 2026-11-08T08:24:53Z |
| BP-018 | Patch release v0.1.1 so PyPI picks up the logo README | DOC-016 | done | pypi.org/project/ledgibly/0.1.1 shows the logo | tag v0.1.1; Release 42462566413; pypi 0.1.1 description has raw logo URL; log 2026-11-08T08:24:53Z |
| DOC-017 | Docs: harness-agnostic core vs adapters for Cursor OpenCode Codex Claude Copilot | | done | docs/ names hub paths and what each tool still needs | docs/02-harnesses.md; plan.md; docs/README.md; log 2026-11-08T08:17:44Z |
| BP-019 | install-skills also installs to ~/.claude/skills from hub | DOC-017 | done | Fake-HOME selftest has the three skills under .claude/skills | src/skills.rs dest_roots; tests/selftest.sh; `bash tests/selftest.sh`; log 2026-11-08T08:37:23Z |
| BP-020 | Cursor adapter stays default init (.cursor/rules, skills symlink) | DOC-017 | done | Bare init still Cursor-shaped; documented as adapter | template/.cursor/rules; docs/02-harnesses.md; `bash tests/selftest.sh` bare; log 2026-11-08T08:37:24Z |
| BP-021 | Claude Code adapter: CLAUDE.md pointer and .claude/skills | DOC-017, BP-019 | done | --with claude lands those files; no overwrite | modules/claude/CLAUDE.md; src/init.rs link_hub_skills; `bash tests/selftest.sh`; log 2026-11-08T08:37:25Z |
| BP-022 | OpenCode adapter: hub enough or optional .opencode/skills overlay | DOC-017 | done | Docs + optional overlay; selftest if we ship files | docs/02-harnesses.md hub-only; no modules/opencode; log 2026-11-08T08:37:26Z |
| BP-023 | Codex adapter: document AGENTS.md hub; ~/.codex not required | DOC-017 | done | Docs only unless a gap appears | docs/02-harnesses.md; log 2026-11-08T08:37:27Z |
| BP-024 | Copilot adapter: .github/copilot-instructions.md and/or .github/skills | DOC-017 | done | --with copilot overlay; AGENTS.md still the profile | modules/copilot/.github/copilot-instructions.md; `bash tests/selftest.sh`; log 2026-11-08T08:37:28Z |
| DOC-015 | Restore short README opener and refresh PyPI badge URL | DOC-014 | done | Tagline and ledger+legible opener restored; badge uses logo=pypi; Start here still uv tool install | README.md; docs/01-framework.md; `python3 template/scripts/ledger_check.py`; log 2026-11-08T08:08:25Z |
| BP-011 | First PyPI release: Trusted Publishing + tag v0.1.0 | QA-003 | done | `uv tool install ledgibly` installs 0.1.0; pypi.org/project/ledgibly/0.1.0 exists | tag v0.1.0; Release run 42461088507; `uv tool install ledgibly`; `ledgibly --version`; log 2026-11-08T08:03:03Z |
| DOC-014 | README PyPI version badge after the package is live | BP-011 | done | README shows a PyPI badge; Start here still `uv tool install ledgibly` | README.md; docs/01-framework.md; `python3 template/scripts/ledger_check.py`; log 2026-11-08T08:03:03Z |
| QA-005 | CI asserts sdist payload includes hidden template files | QA-003 | done | CI fails if `template/.cursor/rules/ponytail.mdc` is missing from `cargo package --list` | .github/workflows/ci.yml sdist payload; `cargo package --list \| grep ponytail`; log 2026-11-08T08:08:25Z |
| BP-007 | Scaffold Rust CLI: Cargo.toml, maturin bin wheel, clap, rust-embed | | done | cargo build; Template embeds .cursor/rules/ponytail.mdc and .gitignore; ledgibly --help works | Cargo.toml; pyproject.toml; src/; `cargo test`; log 2026-11-08T07:20:09Z |
| BP-008 | Port init.sh to ledgibly init with same flags and no-overwrite | BP-007 | done | bash tests/selftest.sh passes against target/debug/ledgibly | src/init.rs; tests/selftest.sh; `bash tests/selftest.sh`; log 2026-11-08T07:20:09Z |
| BP-009 | Port install-skills as copy-from-embed into ~/.cursor and ~/.agents | BP-007 | done | Fake-HOME selftest installs three skills with backup | src/skills.rs; tests/selftest.sh; `bash tests/selftest.sh`; log 2026-11-08T07:20:09Z |
| BP-010 | Retire shell installers; update plan, README, AGENTS verify, skill mention | BP-008, BP-009 | done | init.sh and install-user-skills.sh gone; README leads with uv tool install | plan.md; README.md; AGENTS.md; `bash tests/selftest.sh`; log 2026-11-08T07:20:09Z |
| DOC-013 | Document Rust CLI install and kit vs target dependency rules | BP-010 | done | docs/01-framework.md describes ledgibly CLI and copied stdlib checker | docs/01-framework.md; README.md; `python3 template/scripts/ledger_check.py`; log 2026-11-08T07:20:09Z |
| QA-003 | CI: cargo test, selftest, maturin wheels on tags; no publish yet | BP-008, BP-009 | done | .github/workflows build on PR and wheels on v* tags | .github/workflows/ci.yml; release.yml; `cargo clippy -- -D warnings`; log 2026-11-08T07:20:09Z |
| DOC-012 | Fix grammar and typos in the README opener | DOC-011 | done | Opener uses correct grammar; meaning unchanged | README.md; docs/01-framework.md; `python3 template/scripts/ledger_check.py`; log 2026-11-05T03:12:42Z |
| DOC-011 | README header: pitch, Docs/Template/Skills/Modules nav, MIT badge | DOC-010 | done | README has centered nav and MIT shield under the logo | README.md; docs/01-framework.md; `python3 template/scripts/ledger_check.py`; log 2026-11-05T02:56:48Z |
| DOC-010 | Add brand/ledgibly.png and show it at the top of the README | DOC-009 | done | README shows brand/ledgibly.png above the title; brand/ is tracked | brand/ledgibly.png; README.md; docs/01-framework.md; `python3 template/scripts/ledger_check.py`; log 2026-11-05T02:52:25Z |
| DOC-009 | README motivations: agents track work, tokens, bugs, product-owner role | DOC-008 | done | Motivations say the ledger is how the agent tracks the use case | README.md; docs/01-framework.md; `python3 template/scripts/ledger_check.py`; log 2026-11-04T14:07:57Z |
| DOC-008 | Rewrite kit README: claim, what you can do, start here | DOC-007 | done | README leads with claim, capabilities, motivations, then clone and init | README.md; docs/01-framework.md; `python3 template/scripts/ledger_check.py`; log 2026-11-04T14:04:11Z |
| DOC-007 | README opener: name meaning and how the kit works | DOC-006 | done | README opener explains Ledgibly (ledger + legible) and one-status-source workflow | README.md; docs/01-framework.md; `python3 template/scripts/ledger_check.py`; log 2026-11-04T13:57:54Z |
| DOC-006 | Rename kit to Ledgibly in current docs, LICENSE, and GitHub repo | DOC-005 | done | Titles and LICENSE say Ledgibly; GitHub repo is ledgibly; selftest temp dir uses ledgibly | README.md; LICENSE; docs/01-framework.md; `gh repo view` FedeMatt/ledgibly; `bash tests/selftest.sh`; log 2026-11-04T13:56:12Z |
| DOC-005 | Rewrite kit README opener; kit vs template vs this repo's ledger | DOC-004 | done | README opener names kit vs template vs this repo's ledger; README and plan.md do not call this repo a second product | README.md; plan.md; docs/01-framework.md; `python3 template/scripts/ledger_check.py`; log 2026-11-04T13:47:11Z |
| BP-004 | init.sh: placeholders only in copied files; option values; prepend ART | BP-001 | done | init does not rewrite pre-existing files; missing option values error; PREFIXES always includes ART | init.sh; template/implementation_ledger.md; `bash tests/selftest.sh`; log 2026-11-04T09:17:23Z |
| BP-005 | ledger_check: escaped pipes, superseded replacement, docs word-boundary | BP-001 | done | escaped pipes in cells parse; superseded ignores Dependencies; docs citations use word boundaries | template/scripts/ledger_check.py; skills/keep-implementation-ledger/reference.md; `bash tests/selftest.sh`; log 2026-11-04T09:17:23Z |
| BP-006 | Move rtk.mdc to modules/rtk; add Use-when to python-uv and database-design | BP-001 | done | Bare init has no rtk.mdc; --with rtk adds it; those two skills have Use when in description | modules/rtk/; python-uv and database-design SKILL.md; `bash tests/selftest.sh`; log 2026-11-04T09:17:23Z |
| QA-002 | selftest covers existing-dir init, escaped pipes, failure fixtures, rtk | BP-004, BP-005, BP-006 | done | selftest asserts byte-identical user file, escaped pipe pass, three checker failures, rtk opt-in | tests/selftest.sh; `bash tests/selftest.sh`; log 2026-11-04T09:17:23Z |
| DOC-004 | Move checker error list into docs; README links it; cite new IDs | BP-004, BP-005, BP-006 | done | docs/01-framework.md lists checker errors/warnings and rtk module; README has no error list | docs/01-framework.md; README.md; plan.md; `bash tests/selftest.sh`; log 2026-11-04T09:17:23Z |
| BP-003 | Add MIT LICENSE with generic contributors copyright | BP-001 | done | LICENSE is MIT; README links it; init does not copy it into targets | LICENSE; README.md; `bash tests/selftest.sh`; log 2026-11-04T08:54:04Z |
| BP-002 | Install canonical skills into ~/.cursor/skills and ~/.agents/skills | BP-001 | done | Three skills readable via symlink from both homes | `./install-user-skills.sh`; SKILL.md via symlink; backup-20261104T082343Z; log 2026-11-04T08:24:03Z |
| BP-001 | Create reusable ledger kit: skills, template, modules, init, checker | | done | Kit boots a target repo with profile, ledger, docs, and no overwrite | skills/; template/; modules/; init.sh; install-user-skills.sh; log 2026-11-04T08:23:09Z |
| DOC-003 | Remove comparative-history section from the kit README | DOC-002 | done | README has no Design decisions / earlier-drift table; docs cite DOC-003 | README.md; docs/01-framework.md; `bash tests/selftest.sh`; log 2026-11-04T09:01:30Z |
| DOC-002 | Strip private-project names from public-facing docs | DOC-001 | done | README, plan, and DOC-001 log name no private repos or product stacks | README.md; plan.md; docs/01-framework.md; `rtk rg` empty; log 2026-11-04T08:54:04Z |
| DOC-001 | Framework README, plan, and docs/01-framework.md | BP-001 | done | README explains roles, bootstrap, review findings; docs cite IDs inline | README.md; plan.md; docs/01-framework.md; log 2026-11-04T08:23:09Z |
| QA-001 | selftest for init and ledger_check | BP-001 | done | selftest inits with all modules, checker passes, re-init refuses, bad row fails | tests/selftest.sh; `bash tests/selftest.sh`; log 2026-11-04T08:23:09Z |

## Environment evidence

Multi-host or multi-OS notes live here as `### <ID>` subsections, not in the table cell.

### QA-008

- **Cursor** (this session, 2026-11-08): Agent profile from `AGENTS.md`; user skills from `~/.agents/skills/keep-implementation-ledger/SKILL.md`; after `ledgibly init /tmp/ledgibly-qa008 --with claude,copilot`, `.cursor/rules/ponytail.mdc` is present.
- **Claude Code**: blocked: `claude` not installed on this machine. Init did write `CLAUDE.md` (`@AGENTS.md`) and `.claude/skills`.
- **Codex**: blocked: `codex` not installed on this machine.
- **OpenCode**: blocked: `opencode` not installed on this machine.
- **GitHub Copilot**: blocked: `copilot` not installed on this machine. Init did write `.github/copilot-instructions.md`.

## Milestones

| Gate | Tasks |
|---|---|
| 1. Kit in the repo | BP-001, DOC-001, QA-001, BP-002 |
| 2. Rust CLI on PyPI path | BP-007, BP-008, BP-009, BP-010, DOC-013, QA-003 |
| 3. First PyPI release | BP-011, DOC-014 |
| 4. Multi-harness adapters | DOC-017, DOC-018, BP-019, BP-020, BP-021, BP-022, BP-023, BP-024, BP-025 |
| 5. Hub-only + review harden | DOC-019, BP-026, BP-027, BP-028, BP-029, BP-030, BP-031, BP-032, BP-033, QA-006, QA-007, QA-008, BP-034 |

## Chronological UTC Log

### 2026-11-08T10:01:00Z — Cache-bust PyPI badge for GitHub README (BP-035)

- GitHub Camo was serving a cached Shields SVG for 0.1.2. Badge src is now img.shields.io/pypi/v/ledgibly.svg so Camo fetches a new asset. PyPI and Shields already report 0.1.3.

### 2026-11-08T09:55:57Z — Published patch release v0.1.3 (BP-034)

- PyPI has 0.1.3 wheels (linux x86_64/aarch64, macos arm64/x86_64, win_amd64) plus sdist. Release Actions run https://github.com/FedeMatt/ledgibly/actions/runs/42469027727 succeeded after installing maturin from PyPI (GitHub Releases 504 broke maturin-action). Tag `v0.1.3` at c1e0309.

Newest first. Older entries: [archive](implementation_ledger_archive.md).

### 2026-11-08T09:29:33Z — Started patch release v0.1.3 (BP-034)

- Bumped Cargo.toml / Cargo.lock to 0.1.3. Docs cite BP-034. Tag and Trusted Publishing next.

### 2026-11-08T09:25:27Z — Harness smoke recorded (QA-008)

- Init `/tmp/ledgibly-qa008 --with claude,copilot`: ponytail, `@AGENTS.md`, `.claude/skills`, copilot-instructions present.
- Cursor observed in this session (profile + `~/.agents/skills`). Claude Code, Codex, OpenCode, Copilot CLIs not installed: blocked in Environment evidence.
- `python3 template/scripts/ledger_check.py`.

### 2026-11-08T09:25:11Z — Release gate matches tag to Cargo.toml (QA-007)

- `release.yml` `gate` job runs before linux/macos/windows/sdist. On `v*` tags, `GITHUB_REF_NAME` (minus `v`) must equal `cargo metadata` version.
- Local metadata version is 0.1.2, matching Cargo.toml. Dispatch without a tag skips the compare step.
- `python3 template/scripts/ledger_check.py`.

### 2026-11-08T09:24:51Z — CI matrix: locked tests, Windows claude init (QA-006)

- `.github/workflows/ci.yml`: linux (fmt, clippy, `cargo test --locked`, selftest, ledger_check, sdist), macos (`cargo test --locked`, selftest), windows (`cargo test --locked`, `init --with claude`).
- Local `cargo test --locked` passed. Windows job not run here.
- `python3 template/scripts/ledger_check.py`.

### 2026-11-08T09:24:32Z — Log headings may list several IDs (BP-033)

- `LOG_HEADING_RE` accepts `(ART-001, ART-002)`. Each ID is checked against the Tasks table.
- `reference.md` documents the form. Selftest: valid pair passes; `NOPE-001` warns; 0 errors.
- `bash tests/selftest.sh`; `python3 template/scripts/ledger_check.py`.

### 2026-11-08T09:23:56Z — Checker warns when done depends on unfinished (BP-032)

- `ledger_check.py` warns `{id}: done depends on {dep} ({status})` unless the dep is `done` or `superseded`.
- Selftest fixture ART-002 done on ART-003 pending prints the warning and 0 errors. Kit ledger still 0 warnings.
- `bash tests/selftest.sh`; `python3 template/scripts/ledger_check.py`.

### 2026-11-08T09:23:17Z — Skills use harness-neutral wording (BP-031)

- `keep-implementation-ledger` says harness plan todos. `figma-ui` loads `figma-design-to-code` only if the harness has it.
- README: plan todos include harness plan files. Fake-HOME selftest greps the copied skill.
- `bash tests/selftest.sh`; `python3 template/scripts/ledger_check.py`.

### 2026-11-08T09:17:29Z — Bare init no longer links .cursor/skills (BP-030)

- `src/init.rs` creates `.agents/skills` and, with `--with claude`, `.claude/skills` only. Conflict check no longer treats `.cursor/skills` as an init file.
- `template/AGENTS.md` and README tree drop the symlink. Selftest asserts it is absent on bare and all-modules inits.
- `cargo test`; `bash tests/selftest.sh`; `python3 template/scripts/ledger_check.py`.

### 2026-11-08T09:16:50Z — Skill backups live under ~/.ledgibly/backups (BP-029)

- `src/skills.rs` writes replacements to `~/.ledgibly/backups/<ts>/<home>/`, including leftover `~/.cursor/skills` copies.
- Selftest asserts no `.backup-*` under skill folders. Kit Review exclude updated.
- `cargo test`; `bash tests/selftest.sh`; `python3 template/scripts/ledger_check.py`.

### 2026-11-08T09:16:16Z — install-skills writes the hub only (BP-028)

- Default dest is `~/.agents/skills`. `--claude` adds `~/.claude/skills`. Leftover `~/.cursor/skills/<name>` is moved aside.
- README Start here and `src/main.rs` help match. Fake-HOME selftest covers skip, `--force`, leftover cursor, and no default `.cursor`/`.claude`.
- `cargo test`; `bash tests/selftest.sh`; `python3 template/scripts/ledger_check.py`.

### 2026-11-08T09:15:18Z — Other harnesses pointed at .cursor/rules (BP-027)

- `template/AGENTS.md` tells agents to read `.cursor/rules/*.mdc`.
- `modules/claude/CLAUDE.md` imports `@.cursor/rules/ponytail.mdc` after `@AGENTS.md`.
- `bash tests/selftest.sh`; `python3 template/scripts/ledger_check.py`.

### 2026-11-08T09:14:56Z — CLAUDE.md imports AGENTS.md (BP-026)

- `modules/claude/CLAUDE.md` first line is `@AGENTS.md` so Claude Code loads the profile.
- `tests/selftest.sh` asserts that first line on all-modules and `--with claude` inits.
- `bash tests/selftest.sh`; `python3 template/scripts/ledger_check.py`.

### 2026-11-08T09:14:37Z — Hub-only architecture and vendor-cited harness table (DOC-019)

- `plan.md`: `install-skills` writes `~/.agents/skills`; `--claude` is the only extra user home; Cursor adapter copies `.cursor/rules` and does not write or symlink `.cursor/skills`.
- `docs/02-harnesses.md`: path table cites Cursor, Claude, Codex, OpenCode, Copilot URLs (checked 2026-11-08); kit writes vs vendor reads split; OpenCode `OPENCODE_DISABLE_CLAUDE_CODE_SKILLS=1`.
- `docs/01-framework.md` cites DOC-019. `python3 template/scripts/ledger_check.py`.

### 2026-11-08T08:54:39Z — Multi-harness review filed as pending rows (DOC-019)

- Cursor skills dirs: https://cursor.com/docs/skills. Copilot: https://docs.github.com/en/copilot/concepts/agents/about-agent-skills. OpenCode: https://opencode.ai/docs/skills/ and issue 29950.
- Claude: https://code.claude.com/docs/en/claude-md (`@AGENTS.md` import) and https://code.claude.com/docs/en/skills (`.claude/skills` only).
- High: `modules/claude/CLAUDE.md` markdown-links AGENTS.md, so Claude skips the profile once CLAUDE.md exists.
- High: `src/skills.rs` dest_roots (lines 31–35) copies each skill into `~/.agents`, `~/.cursor`, and `~/.claude`; Cursor and OpenCode then discover duplicates.
- High: always-on policy lives only in `template/.cursor/rules/*.mdc`; `template/AGENTS.md` never points other harnesses at it.
- Pending: DOC-019, BP-026–BP-033, QA-006–QA-008. Decisions: hub-only install-skills (`--claude` extra); drop `.cursor/skills` symlink.

### 2026-11-08T08:45:59Z — Patch release v0.1.2 on PyPI (BP-025)

- Tag `v0.1.2`; Release 42464038529. PyPI 0.1.2 description has the coding-agents tagline and harness README. Smoke check in docs/02-harnesses.md.

### 2026-11-08T08:37:29Z — README no longer Cursor-only (DOC-018)

- Tagline “A ledger kit for coding agents.” Cargo/PyPI description and CLI about match. Install-skills destinations and `--with claude,copilot` documented. Cursor remains default adapter.

### 2026-11-08T08:37:28Z — Copilot instructions overlay (BP-024)

- `modules/copilot/.github/copilot-instructions.md` points at plan, ledger, docs, AGENTS.md. No `.github/skills`. Isolated `--with copilot` selftest; bare init has no file.

### 2026-11-08T08:37:27Z — Codex uses AGENTS.md hub (BP-023)

- `docs/02-harnesses.md`: Codex reads `AGENTS.md` and `~/.agents/skills` / `.agents/skills`. No `~/.codex` overlay.

### 2026-11-08T08:37:26Z — OpenCode hub is enough (BP-022)

- `docs/02-harnesses.md`: OpenCode already scans `~/.agents/skills` and `.agents/skills`. No `modules/opencode`.

### 2026-11-08T08:37:25Z — Claude adapter via --with claude (BP-021)

- `modules/claude/CLAUDE.md` pointer. `--with claude` also links `.claude/skills` → `.agents/skills`. Bare init has neither. Selftest isolated + all-modules.

### 2026-11-08T08:37:24Z — Cursor remains default adapter (BP-020)

- Bare init still writes ponytail.mdc and `.cursor/skills` symlink. Documented as adapter in `docs/02-harnesses.md`.

### 2026-11-08T08:37:23Z — install-skills copies to ~/.claude/skills (BP-019)

- `src/skills.rs` dest_roots adds `.claude/skills`. Fake-HOME selftest asserts the three skills there.

### 2026-11-08T08:24:53Z — Patch release v0.1.1 on PyPI (BP-018)

- Tag `v0.1.1`; Release 42462566413 published wheels+sdist. PyPI 0.1.1 description has the absolute logo URL. Dirty-lock CI failure (42462411114) fixed in 6ca4831.

### 2026-11-08T08:24:52Z — README logo uses absolute GitHub raw URL (DOC-016)

- Image src → `https://raw.githubusercontent.com/FedeMatt/ledgibly/main/brand/ledgibly.png`. Cited in docs/01-framework.md.

### 2026-11-08T08:17:44Z — Harness-agnostic core vs adapters documented (DOC-017)

- `docs/02-harnesses.md`: hub paths, Cursor/OpenCode/Codex/Claude/Copilot table, Claude as extra skill home. `plan.md` admits other harnesses; Cursor remains default adapter. BP-019–BP-024 left pending.

### 2026-11-08T08:08:25Z — CI asserts sdist includes ponytail.mdc (QA-005)

- `.github/workflows/ci.yml`: `cargo package --list | grep -F 'template/.cursor/rules/ponytail.mdc'` after ledger_check.

### 2026-11-08T08:08:25Z — Restored short README opener and refreshed badge (DOC-015)

- README.md: tagline and `23ef4d1` ledger+legible opener; PyPI badge URL uses `?logo=pypi` to bust Camo cache. docs/01-framework.md cites DOC-015.

### 2026-11-08T08:03:03Z — README shows PyPI version badge (DOC-014)

- README.md: shields.io `pypi/v/ledgibly` next to the MIT badge. Start here still `uv tool install ledgibly`. docs/01-framework.md cites DOC-014.

### 2026-11-08T08:03:03Z — ledgibly 0.1.0 on PyPI (BP-011)

- Pushed `main` (`f3b0bb2`) and tag `v0.1.0`. Release workflow published wheels; `uv tool install ledgibly` → `ledgibly 0.1.0`. https://pypi.org/project/ledgibly/0.1.0/
- plan.md Architecture notes PyPI platform wheels; docs/01-framework.md cites BP-011.

### 2026-11-08T07:48:00Z — Started first PyPI release wiring (BP-011)

- Uncommented Trusted Publishing job in `.github/workflows/release.yml` (`environment: pypi`, `id-token: write`).
- `https://pypi.org/pypi/ledgibly/json` is 404 (name free).
- Dry-run: `cargo package --list` includes `template/.cursor/rules/ponytail.mdc`; `maturin build --release --out dist` wrote `ledgibly-0.1.0-py3-none-macosx_10_12_x86_64.whl` with `ledgibly-0.1.0.data/scripts/ledgibly`. Upload waits on pending publisher.

### 2026-11-08T07:20:09Z — CI for Rust CLI and maturin wheels (QA-003)

- `.github/workflows/ci.yml`: fmt, clippy, cargo test, selftest, ledger_check.
- `.github/workflows/release.yml`: linux/macOS/Windows wheels + sdist on `v*` tags; publish left commented.

### 2026-11-08T07:20:09Z — Docs describe Rust CLI and stdlib target checker (DOC-013)

- `docs/01-framework.md` and README: `uv tool install`, `ledgibly init` / `install-skills`, copied stdlib checker, no target runtime dep on the package.

### 2026-11-08T07:20:09Z — Shell installers retired; kit docs updated (BP-010)

- Removed `init.sh` and `install-user-skills.sh`. Updated `plan.md`, `README.md`, `AGENTS.md`, skill mention of `ledgibly init`.

### 2026-11-08T07:20:09Z — install-skills copies from embed (BP-009)

- `src/skills.rs` copies three skills into `~/.cursor/skills` and `~/.agents/skills` with backup. Selftest uses fake HOME.

### 2026-11-08T07:20:09Z — ledgibly init matches former init.sh (BP-008)

- `src/init.rs`: template + modules, no-overwrite, placeholders, fragments, ART prepend, `.cursor/skills` symlink. `bash tests/selftest.sh` ok.

### 2026-11-08T07:20:09Z — Rust CLI scaffold with maturin bin + rust-embed (BP-007)

- `Cargo.toml`, `pyproject.toml` (`bindings = "bin"`), `src/{main,embed,init,skills}.rs`. Embed includes `.cursor/rules/ponytail.mdc` and `.gitignore`. `cargo test` 9 passed.

### 2026-11-05T03:12:42Z — README opener grammar and typos fixed (DOC-012)

- README.md opener: widespread, arisen, lost, these problems, a harness, the name comes from. Meaning unchanged.

### 2026-11-05T02:56:48Z — README header has nav and MIT badge (DOC-011)

- README.md: centered pitch, Docs/Template/Skills/Modules links, MIT shield under the logo. Opener no longer repeats the license line.
- docs/01-framework.md cites DOC-011.

### 2026-11-05T02:52:25Z — README shows Ledgibly brand mark (DOC-010)

- Copied primary logo to brand/ledgibly.png. README.md centers it above the title. docs/01-framework.md cites DOC-010.

### 2026-11-04T14:07:57Z — Motivations: agent as product owner of the use case (DOC-009)

- README.md Motivations: one row and the recent log, fewer tokens, fewer invented stories, the assistant owns the use case.
- docs/01-framework.md cites DOC-009.

### 2026-11-04T14:04:11Z — README rewritten in ds4 voice (DOC-008)

- README.md leads with a claim, what you can do, motivations, how to use the kit, then clone and init. Spec tables come after. ASCII diagram removed.
- docs/01-framework.md cites DOC-008.

### 2026-11-04T13:57:54Z — README opener names the idea and the workflow (DOC-007)

- README.md opener: Ledgibly is ledger + legible; one ledger table is the only status source; `init.sh` copies `template/`.
- docs/01-framework.md cites DOC-007.

### 2026-11-04T13:56:12Z — Kit renamed to Ledgibly (DOC-006)

- Current titles, LICENSE, template, and keep-implementation-ledger skill say Ledgibly. selftest temp dirs use `ledgibly`.
- GitHub repo is `FedeMatt/ledgibly` (`gh repo rename ledgibly --yes`). Older log entries still say project-blueprint.

### 2026-11-04T13:47:11Z — README opener names kit vs template (DOC-005)

- README.md opener: `init.sh` copies `template/`, not this repository's ledger. Same document roles track kit work.
- plan.md Architecture uses the same wording. docs/01-framework.md cites DOC-005.

### 2026-11-04T09:17:23Z — Checker error list lives in docs (DOC-004)

- docs/01-framework.md lists checker errors/warnings, escaped pipes, rtk module, and init placeholder scope.
- README.md Bootstrap lists rtk; Ledger mechanics links the docs chapter. plan.md Architecture lists rtk as a module.

### 2026-11-04T09:17:23Z — selftest covers init and checker edge cases (QA-002)

- tests/selftest.sh: existing-dir `src/t.hbs` stays byte-identical; escaped pipe evidence passes; missing `--name` value fails; ART prepended to prefixes.
- Failure fixtures: 7-column row, `done` without evidence, superseded with only a dependency, log not newest-first.
- Bare init has no rtk.mdc; `--with rtk` adds it. `bash tests/selftest.sh` printed `selftest: ok`.

### 2026-11-04T09:17:23Z — rtk is an opt-in module (BP-006)

- Moved template/.cursor/rules/rtk.mdc to modules/rtk/.cursor/rules/rtk.mdc.
- python-uv-workflow and database-design skill descriptions include "Use when".

### 2026-11-04T09:17:23Z — ledger_check handles escaped pipes and docs IDs (BP-005)

- split_row splits on unescaped `|`; superseded replacement IDs ignore Dependencies; docs citation uses word boundaries.
- reference.md: escape `|` in cells as `\|`.

### 2026-11-04T09:17:23Z — init.sh only rewrites copied files (BP-004)

- Placeholders run only on paths copy_tree wrote. Missing option values print `needs a value`. PREFIXES without ART gets ART prepended.
- Seeded ART-001 is pending until plan.md and AGENTS.md have no `<fill me>`.

### 2026-11-04T09:01:30Z — Kit README has no comparative-history section (DOC-003)

- Deleted the README Design decisions / earlier-drift table. docs/01-framework.md cites DOC-001, DOC-002, DOC-003 on install/run-only README.

### 2026-11-04T08:54:04Z — MIT LICENSE for the kit (BP-003)

- LICENSE is MIT, copyright 2026 project-blueprint contributors. README links it. init.sh copies template/ only, so targets pick their own license.

### 2026-11-04T08:54:04Z — Public-facing docs name no private projects (DOC-002)

- README design-decision section, plan.md Product/non-goals, and the DOC-001 log bullet no longer name private repos or product stacks. docs/01-framework.md cites DOC-002.

### 2026-11-04T08:24:03Z — Canonical skills installed as user-level symlinks (BP-002)

- `./install-user-skills.sh` backed up existing keep-implementation-ledger and figma-ui, then linked keep-implementation-ledger, code-review, and figma-ui from both ~/.cursor/skills and ~/.agents/skills into skills/.
- SKILL.md is readable through the symlink. If a future Cursor chat ignores the links, rerun with `--copy`.

### 2026-11-04T08:23:09Z — selftest covers init and checker (QA-001)

- `tests/selftest.sh` inits with every module, inits a bare repo, asserts no leftover `{{PLACEHOLDER}}` tokens, symlink `.cursor/skills -> ../.agents/skills`.
- `python3 scripts/ledger_check.py` passes on a fresh init; re-init refuses; a 7-column row is an error.
- `bash tests/selftest.sh` printed `selftest: ok`.

### 2026-11-04T08:23:09Z — Framework docs and README (DOC-001)

- README.md records document roles, bootstrap, canonical skills, design-decision notes.
- docs/01-framework.md cites BP-001, DOC-001, QA-001. plan.md is scope only.

### 2026-11-04T08:23:09Z — project-blueprint kit created (BP-001)

- Canonical skills in skills/: keep-implementation-ledger, code-review, figma-ui.
- template/ plus modules/ (figma, source-docs, python-uv, formatter, database-design).
- init.sh and install-user-skills.sh. Checker in template/scripts/ledger_check.py.
