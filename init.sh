#!/usr/bin/env bash
# Bootstrap a repo from template/ and optional modules/. Refuses to overwrite.
set -euo pipefail

ROOT="$(cd "$(dirname "$0")" && pwd)"
VALID_MODULES="figma source-docs python-uv formatter database-design rtk"

usage() {
  cat <<'EOF'
Usage: init.sh <target-dir> [options]

  --name NAME           Project name (default: basename of target-dir)
  --prefixes "A B C"    ID prefixes (default: ART DOC QA)
  --verify "cmd"        Profile verify command (default: python3 scripts/ledger_check.py)
  --with a,b            Modules: figma,source-docs,python-uv,formatter,database-design,rtk
  -h, --help

Copies template/, overlays modules, replaces {{PROJECT}} {{PREFIXES}} {{VERIFY}} {{DATE}},
creates .cursor/skills -> ../.agents/skills, and prints remaining <fill me> lines.
Refuses to overwrite existing files.
EOF
}

need_value() {
  if [[ $# -lt 2 ]]; then
    echo "init.sh: $1 needs a value" >&2
    exit 1
  fi
}

TARGET=""
NAME=""
PREFIXES="ART DOC QA"
VERIFY="python3 scripts/ledger_check.py"
WITH=""

while [[ $# -gt 0 ]]; do
  case "$1" in
    --name)
      need_value "$@"
      NAME="$2"
      shift 2
      ;;
    --prefixes)
      need_value "$@"
      PREFIXES="$2"
      shift 2
      ;;
    --verify)
      need_value "$@"
      VERIFY="$2"
      shift 2
      ;;
    --with)
      need_value "$@"
      WITH="$2"
      shift 2
      ;;
    -h|--help)
      usage
      exit 0
      ;;
    -*)
      echo "init.sh: unknown option $1" >&2
      usage >&2
      exit 1
      ;;
    *)
      if [[ -n "$TARGET" ]]; then
        echo "init.sh: unexpected argument $1" >&2
        exit 1
      fi
      TARGET="$1"
      shift
      ;;
  esac
done

if [[ -z "$TARGET" ]]; then
  usage >&2
  exit 1
fi

if [[ -e "$TARGET" && ! -d "$TARGET" ]]; then
  echo "init.sh: $TARGET exists and is not a directory" >&2
  exit 1
fi

mkdir -p "$TARGET"
TARGET="$(cd "$TARGET" && pwd)"
if [[ -z "$NAME" ]]; then
  NAME="$(basename "$TARGET")"
fi

case " $PREFIXES " in
  *" ART "*) ;;
  *) PREFIXES="ART $PREFIXES" ;;
esac

MODULES=()
raw_modules=()
if [[ -n "$WITH" ]]; then
  IFS=',' read -r -a raw_modules <<< "$WITH"
fi
for mod in "${raw_modules[@]+"${raw_modules[@]}"}"; do
  mod="${mod// /}"
  [[ -z "$mod" ]] && continue
  case " $VALID_MODULES " in
    *" $mod "*) MODULES+=("$mod") ;;
    *)
      echo "init.sh: unknown module $mod (valid: $VALID_MODULES)" >&2
      exit 1
      ;;
  esac
done

list_files() {
  local src="${1%/}"
  find "$src" -type f ! -name 'AGENTS.md.fragment' ! -name '.DS_Store'
}

existing=()
check_tree() {
  local src="${1%/}"
  local f rel dest
  while IFS= read -r f; do
    [[ -z "$f" ]] && continue
    rel="${f#$src/}"
    dest="$TARGET/$rel"
    if [[ -e "$dest" || -L "$dest" ]]; then
      existing+=("$dest")
    fi
  done < <(list_files "$src")
}

check_tree "$ROOT/template"
for mod in "${MODULES[@]+"${MODULES[@]}"}"; do
  check_tree "$ROOT/modules/$mod"
done
if [[ -e "$TARGET/.cursor/skills" || -L "$TARGET/.cursor/skills" ]]; then
  existing+=("$TARGET/.cursor/skills")
fi

if [[ ${#existing[@]} -gt 0 ]]; then
  echo "init.sh: refusing to overwrite:" >&2
  printf '  %s\n' "${existing[@]}" >&2
  exit 1
fi

copied=()
copy_tree() {
  local src="${1%/}"
  local f rel dest
  while IFS= read -r f; do
    [[ -z "$f" ]] && continue
    rel="${f#$src/}"
    dest="$TARGET/$rel"
    mkdir -p "$(dirname "$dest")"
    cp "$f" "$dest"
    copied+=("$dest")
  done < <(list_files "$src")
}

copy_tree "$ROOT/template"
for mod in "${MODULES[@]+"${MODULES[@]}"}"; do
  copy_tree "$ROOT/modules/$mod"
done

fragments=()
for mod in "${MODULES[@]+"${MODULES[@]}"}"; do
  frag="$ROOT/modules/$mod/AGENTS.md.fragment"
  if [[ -f "$frag" ]]; then
    fragments+=("$frag")
  fi
done

DATE_UTC="$(date -u +%Y-%m-%dT%H:%M:%SZ)"
export BP_PROJECT="$NAME" BP_PREFIXES="$PREFIXES" BP_VERIFY="$VERIFY" BP_DATE="$DATE_UTC"

python3 - "$TARGET" "${#fragments[@]}" ${fragments[@]+"${fragments[@]}"} -- ${copied[@]+"${copied[@]}"} <<'PY'
import os
import sys
from pathlib import Path

args = sys.argv[1:]
target = Path(args[0])
n_frag = int(args[1])
fragments = [Path(p) for p in args[2 : 2 + n_frag]]
rest = args[2 + n_frag :]
if rest[:1] == ["--"]:
    rest = rest[1:]
copied = [Path(p) for p in rest]
agents = target / "AGENTS.md"
if fragments:
    marker = "<!-- end:agent-profile -->"
    block = "\n".join(p.read_text(encoding="utf-8").rstrip() + "\n" for p in fragments)
    text = agents.read_text(encoding="utf-8")
    if marker in text:
        text = text.replace(marker, block + "\n" + marker, 1)
    else:
        text = text.rstrip() + "\n\n" + block
    agents.write_text(text, encoding="utf-8")

repl = {
    "{{PROJECT}}": os.environ["BP_PROJECT"],
    "{{PREFIXES}}": os.environ["BP_PREFIXES"],
    "{{VERIFY}}": os.environ["BP_VERIFY"],
    "{{DATE}}": os.environ["BP_DATE"],
}
for path in copied:
    if not path.is_file() or path.is_symlink():
        continue
    try:
        text = path.read_text(encoding="utf-8")
    except (OSError, UnicodeDecodeError):
        continue
    if "{{" not in text:
        continue
    for key, value in repl.items():
        text = text.replace(key, value)
    path.write_text(text, encoding="utf-8")
PY

docs_readme="$TARGET/docs/README.md"
if [[ -f "$docs_readme" && -d "$TARGET/docs/ui" ]] && ! grep -q 'ui/README' "$docs_readme"; then
  printf '\nUI inventory: [docs/ui](ui/README.md).\n' >> "$docs_readme"
fi
if [[ -f "$docs_readme" && -d "$TARGET/docs/sources" ]] && ! grep -q 'sources/README' "$docs_readme"; then
  printf '\nCanonical sources: [docs/sources](sources/README.md).\n' >> "$docs_readme"
fi

chmod +x "$TARGET/scripts/ledger_check.py"
ln -s ../.agents/skills "$TARGET/.cursor/skills"

echo "init.sh: created $TARGET ($NAME)"
echo "init.sh: remaining <fill me> lines:"
find "$TARGET" \( -name '*.md' -o -name '*.mdc' \) -exec grep -n '<fill me>' {} + || true
