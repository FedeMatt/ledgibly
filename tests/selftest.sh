#!/usr/bin/env bash
# One runnable check: init.sh + ledger_check.py. No frameworks.
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
TMP="$(mktemp -d "${TMPDIR:-/tmp}/project-blueprint.XXXXXX")"
trap 'rm -rf "$TMP"' EXIT

APP="$TMP/app"
"$ROOT/init.sh" "$APP" \
  --name DemoApp \
  --prefixes "ART FE BE DOC QA" \
  --verify "python3 scripts/ledger_check.py" \
  --with figma,source-docs,python-uv,formatter,database-design

if leftover="$(grep -R -n -E '\{\{[A-Z][A-Z0-9_]*\}\}' "$APP" || true)" && [[ -n "$leftover" ]]; then
  echo "selftest: unreplaced placeholders:" >&2
  echo "$leftover" >&2
  exit 1
fi

test -L "$APP/.cursor/skills"
test "$(readlink "$APP/.cursor/skills")" = "../.agents/skills"
test -f "$APP/docs/ui/screens.md"
test -f "$APP/docs/sources/README.md"
test -f "$APP/.agents/skills/source-docs/SKILL.md"
test -f "$APP/.agents/skills/python-uv-workflow/SKILL.md"
test -f "$APP/.agents/skills/database-design/SKILL.md"
test -f "$APP/.cursor/rules/formatting.mdc"
grep -q 'DemoApp' "$APP/AGENTS.md"
grep -q 'ART FE BE DOC QA' "$APP/AGENTS.md"
grep -q '^fileKey' "$APP/AGENTS.md" || grep -q 'fileKey' "$APP/AGENTS.md"
grep -q 'Sources' "$APP/AGENTS.md"
grep -q 'docs/ui' "$APP/docs/README.md"
grep -q 'docs/sources' "$APP/docs/README.md"

python3 "$APP/scripts/ledger_check.py" --root "$APP"

BARE="$TMP/bare"
"$ROOT/init.sh" "$BARE" --name BareApp
test ! -e "$BARE/docs/ui"
test ! -f "$BARE/.agents/skills/source-docs/SKILL.md"
python3 "$BARE/scripts/ledger_check.py" --root "$BARE"

if "$ROOT/init.sh" "$APP" --name DemoApp --prefixes "ART DOC QA" --verify "true"; then
  echo "selftest: re-init should have refused to overwrite" >&2
  exit 1
fi

python3 - "$APP/implementation_ledger.md" <<'PY'
from pathlib import Path
import sys
path = Path(sys.argv[1])
lines = path.read_text(encoding="utf-8").splitlines(True)
out = []
inserted = False
for line in lines:
    out.append(line)
    if not inserted and line.startswith("| ART-002 |"):
        out.append("| BAD | x | | done | y | z | extra |\n")
        inserted = True
path.write_text("".join(out), encoding="utf-8")
if not inserted:
    raise SystemExit("selftest: ART-002 row not found")
PY

if python3 "$APP/scripts/ledger_check.py" --root "$APP"; then
  echo "selftest: checker should have failed on a malformed row" >&2
  exit 1
fi

echo "selftest: ok"
