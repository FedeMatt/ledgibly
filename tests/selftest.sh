#!/usr/bin/env bash
# One runnable check: ledgibly init + install-skills + ledger_check.py. No frameworks.
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
TMP="$(mktemp -d "${TMPDIR:-/tmp}/ledgibly.XXXXXX")"
trap 'rm -rf "$TMP"' EXIT

cargo build --quiet --manifest-path "$ROOT/Cargo.toml"
LEDGIBLY="$ROOT/target/debug/ledgibly"

if "$LEDGIBLY" init "$TMP/noval" --name 2>"$TMP/noval.err"; then
  echo "selftest: --name without a value should fail" >&2
  exit 1
fi
grep -q 'needs a value' "$TMP/noval.err"

APP="$TMP/app"
"$LEDGIBLY" init "$APP" \
  --name DemoApp \
  --prefixes "ART FE BE DOC QA" \
  --verify "python3 scripts/ledger_check.py" \
  --with figma,source-docs,python-uv,formatter,database-design,rtk,claude,copilot

if leftover="$(grep -R -n -E '\{\{[A-Z][A-Z0-9_]*\}\}' "$APP" || true)" && [[ -n "$leftover" ]]; then
  echo "selftest: unreplaced placeholders:" >&2
  echo "$leftover" >&2
  exit 1
fi

test ! -e "$APP/.cursor/skills"
test -d "$APP/.agents/skills"
test -f "$APP/.cursor/rules/ponytail.mdc"
test -f "$APP/.gitignore"
test -f "$APP/docs/ui/screens.md"
test -f "$APP/docs/sources/README.md"
test -f "$APP/.agents/skills/source-docs/SKILL.md"
test -f "$APP/.agents/skills/python-uv-workflow/SKILL.md"
test -f "$APP/.agents/skills/database-design/SKILL.md"
test -f "$APP/.cursor/rules/formatting.mdc"
test -f "$APP/.cursor/rules/rtk.mdc"
test -f "$APP/CLAUDE.md"
test "$(head -n1 "$APP/CLAUDE.md")" = "@AGENTS.md"
grep -q '.cursor/rules' "$APP/AGENTS.md"
grep -q '@.cursor/rules/ponytail.mdc' "$APP/CLAUDE.md"
test -L "$APP/.claude/skills"
test "$(readlink "$APP/.claude/skills")" = "../.agents/skills"
test -f "$APP/.github/copilot-instructions.md"
grep -q 'DemoApp' "$APP/AGENTS.md"
grep -q 'ART FE BE DOC QA' "$APP/AGENTS.md"
grep -q '^fileKey' "$APP/AGENTS.md" || grep -q 'fileKey' "$APP/AGENTS.md"
grep -q 'Sources' "$APP/AGENTS.md"
grep -q 'docs/ui' "$APP/docs/README.md"
grep -q 'docs/sources' "$APP/docs/README.md"
grep -q 'Use when' "$APP/.agents/skills/python-uv-workflow/SKILL.md"
grep -q 'Use when' "$APP/.agents/skills/database-design/SKILL.md"

python3 "$APP/scripts/ledger_check.py" --root "$APP"

BARE="$TMP/bare"
"$LEDGIBLY" init "$BARE" --name BareApp
test ! -e "$BARE/.cursor/skills"
test ! -e "$BARE/docs/ui"
test ! -f "$BARE/.agents/skills/source-docs/SKILL.md"
test ! -f "$BARE/.cursor/rules/rtk.mdc"
test ! -f "$BARE/CLAUDE.md"
test ! -e "$BARE/.claude/skills"
test ! -f "$BARE/.github/copilot-instructions.md"
python3 "$BARE/scripts/ledger_check.py" --root "$BARE"

EXIST="$TMP/existing"
mkdir -p "$EXIST/src"
printf '{{PROJECT}}\r\n' > "$EXIST/src/t.hbs"
cp "$EXIST/src/t.hbs" "$TMP/t.hbs.orig"
"$LEDGIBLY" init "$EXIST" --name ExistingApp
cmp "$EXIST/src/t.hbs" "$TMP/t.hbs.orig"

PFX="$TMP/pfx"
"$LEDGIBLY" init "$PFX" --prefixes "FE BE"
grep -q 'ID prefixes: ART FE BE' "$PFX/AGENTS.md"

RTK="$TMP/rtkonly"
"$LEDGIBLY" init "$RTK" --name RtkApp --with rtk
test -f "$RTK/.cursor/rules/rtk.mdc"

CLAUDE="$TMP/claudeonly"
"$LEDGIBLY" init "$CLAUDE" --name ClaudeApp --with claude
test -f "$CLAUDE/CLAUDE.md"
test "$(head -n1 "$CLAUDE/CLAUDE.md")" = "@AGENTS.md"
grep -q '.cursor/rules' "$CLAUDE/AGENTS.md"
grep -q '@.cursor/rules/ponytail.mdc' "$CLAUDE/CLAUDE.md"
test -L "$CLAUDE/.claude/skills"
test "$(readlink "$CLAUDE/.claude/skills")" = "../.agents/skills"
test ! -f "$CLAUDE/.github/copilot-instructions.md"

COPILOT="$TMP/copilotonly"
"$LEDGIBLY" init "$COPILOT" --name CopilotApp --with copilot
test -f "$COPILOT/.github/copilot-instructions.md"
test ! -f "$COPILOT/CLAUDE.md"
test ! -e "$COPILOT/.claude/skills"

if "$LEDGIBLY" init "$APP" --name DemoApp --prefixes "ART DOC QA" --verify "true"; then
  echo "selftest: re-init should have refused to overwrite" >&2
  exit 1
fi

FAKE_HOME="$TMP/home"
mkdir -p "$FAKE_HOME"
HOME="$FAKE_HOME" "$LEDGIBLY" install-skills
test -f "$FAKE_HOME/.agents/skills/keep-implementation-ledger/SKILL.md"
test -f "$FAKE_HOME/.agents/skills/code-review/SKILL.md"
test -f "$FAKE_HOME/.agents/skills/figma-ui/SKILL.md"
test -f "$FAKE_HOME/.agents/skills/keep-implementation-ledger/reference.md"
test ! -e "$FAKE_HOME/.cursor/skills"
test ! -e "$FAKE_HOME/.claude/skills"
HOME="$FAKE_HOME" "$LEDGIBLY" install-skills --claude
test -f "$FAKE_HOME/.claude/skills/keep-implementation-ledger/SKILL.md"
test -f "$FAKE_HOME/.claude/skills/code-review/SKILL.md"
test -f "$FAKE_HOME/.claude/skills/figma-ui/SKILL.md"
# Second run with matching contents should skip
HOME="$FAKE_HOME" "$LEDGIBLY" install-skills >"$TMP/skills2.out"
grep -q 'already installed' "$TMP/skills2.out"
# Force replaces with backup
echo dirty >"$FAKE_HOME/.agents/skills/code-review/SKILL.md"
HOME="$FAKE_HOME" "$LEDGIBLY" install-skills --force >"$TMP/skills3.out"
grep -q 'backed up' "$TMP/skills3.out"
grep -q 'name: code-review' "$FAKE_HOME/.agents/skills/code-review/SKILL.md"
test -d "$(echo "$FAKE_HOME"/.ledgibly/backups/*)"
test ! -d "$(echo "$FAKE_HOME"/.agents/skills/.backup-*)"
# Leftover ~/.cursor kit skill is moved aside, not rewritten
mkdir -p "$FAKE_HOME/.cursor/skills/code-review"
echo leftover >"$FAKE_HOME/.cursor/skills/code-review/SKILL.md"
HOME="$FAKE_HOME" "$LEDGIBLY" install-skills >"$TMP/skills4.out"
grep -q 'backed up' "$TMP/skills4.out"
test ! -e "$FAKE_HOME/.cursor/skills/code-review"
test ! -d "$(echo "$FAKE_HOME"/.cursor/skills/.backup-*)"
test ! -d "$(echo "$FAKE_HOME"/.claude/skills/.backup-*)"
test -d "$(echo "$FAKE_HOME"/.ledgibly/backups/*)"

replace_art002() {
  local ledger="$1"
  local new_row="$2"
  python3 - "$ledger" "$new_row" <<'PY'
from pathlib import Path
import sys
path = Path(sys.argv[1])
new_row = sys.argv[2]
if not new_row.endswith("\n"):
    new_row += "\n"
lines = path.read_text(encoding="utf-8").splitlines(True)
out = []
found = False
for line in lines:
    if line.startswith("| ART-002 |"):
        out.append(new_row)
        found = True
    else:
        out.append(line)
if not found:
    raise SystemExit("selftest: ART-002 row not found")
path.write_text("".join(out), encoding="utf-8")
PY
}

PIPE="$TMP/pipe"
cp -R "$BARE" "$PIPE"
replace_art002 "$PIPE/implementation_ledger.md" \
  "| ART-002 | Operational implementation ledger | ART-001 | done | Ledger has Tasks table, UTC log, and checker passes | \`rg 'a\\|b' docs/\` passed |"
python3 "$PIPE/scripts/ledger_check.py" --root "$PIPE"

fail_case() {
  local name="$1"
  local dir="$TMP/$name"
  cp -R "$BARE" "$dir"
  case "$name" in
    columns)
      python3 - "$dir/implementation_ledger.md" <<'PY'
from pathlib import Path
import sys
path = Path(sys.argv[1])
lines = path.read_text(encoding="utf-8").splitlines(True)
out = []
found = False
for line in lines:
    out.append(line)
    if not found and line.startswith("| ART-002 |"):
        out.append("| BAD | x | | done | y | z | extra |\n")
        found = True
path.write_text("".join(out), encoding="utf-8")
if not found:
    raise SystemExit("selftest: ART-002 row not found")
PY
      ;;
    no-evidence)
      replace_art002 "$dir/implementation_ledger.md" \
        "| ART-002 | Operational implementation ledger | ART-001 | done | Ledger has Tasks table, UTC log, and checker passes | |"
      ;;
    superseded-dep)
      python3 - "$dir/implementation_ledger.md" <<'PY'
from pathlib import Path
import sys
path = Path(sys.argv[1])
text = path.read_text(encoding="utf-8")
needle = "| ART-002 |"
if needle not in text:
    raise SystemExit("selftest: ART-002 row not found")
text = text.replace(
    needle,
    "| ART-003 | replaced | ART-002 | superseded | leftover | |\n" + needle,
    1,
)
path.write_text(text, encoding="utf-8")
PY
      ;;
    log-order)
      python3 - "$dir/implementation_ledger.md" <<'PY'
from pathlib import Path
import sys
path = Path(sys.argv[1])
text = path.read_text(encoding="utf-8")
marker = "## Chronological UTC Log"
idx = text.find(marker)
if idx < 0:
    raise SystemExit("selftest: log heading not found")
nl = text.find("\n", idx)
extra = (
    "\n\n### 2020-01-01T00:00:00Z — old (ART-002)\n\n"
    "### 2021-01-01T00:00:00Z — newer (ART-002)\n"
)
path.write_text(text[: nl + 1] + extra + text[nl + 1 :], encoding="utf-8")
PY
      ;;
    *)
      echo "selftest: unknown fail case $name" >&2
      exit 1
      ;;
  esac
  if python3 "$dir/scripts/ledger_check.py" --root "$dir"; then
    echo "selftest: $name should have failed" >&2
    exit 1
  fi
}

fail_case columns
fail_case no-evidence
fail_case superseded-dep
fail_case log-order

echo "selftest: ok"
