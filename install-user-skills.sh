#!/usr/bin/env bash
# Install canonical skills into ~/.cursor/skills and ~/.agents/skills.
# Existing copies are moved to .backup-<UTC>/. Default is symlink; pass --copy if Cursor ignores links.
set -euo pipefail

ROOT="$(cd "$(dirname "$0")" && pwd)"
MODE="symlink"
if [[ "${1:-}" == "--copy" ]]; then
  MODE="copy"
elif [[ "${1:-}" == "-h" || "${1:-}" == "--help" ]]; then
  cat <<'EOF'
Usage: install-user-skills.sh [--copy]

Install keep-implementation-ledger, code-review, and figma-ui into
~/.cursor/skills and ~/.agents/skills. Existing files or links are moved to
.backup-<UTC>/ first. --copy copies instead of symlinking.
EOF
  exit 0
elif [[ $# -gt 0 ]]; then
  echo "install-user-skills.sh: unknown option $1" >&2
  exit 1
fi

TS="$(date -u +%Y%m%dT%H%M%SZ)"
SKILLS=(keep-implementation-ledger code-review figma-ui)

install_one() {
  local name="$1" dest_root="$2"
  local src="$ROOT/skills/$name"
  local dest="$dest_root/$name"
  mkdir -p "$dest_root"
  if [[ -L "$dest" ]]; then
    local cur
    cur="$(readlink "$dest")"
    if [[ "$cur" == "$src" && "$MODE" == "symlink" ]]; then
      echo "already linked: $dest"
      return
    fi
  fi
  if [[ -e "$dest" || -L "$dest" ]]; then
    local bak="$dest_root/.backup-$TS"
    mkdir -p "$bak"
    mv "$dest" "$bak/"
    echo "backed up $dest -> $bak/$name"
  fi
  if [[ "$MODE" == "copy" ]]; then
    cp -R "$src" "$dest"
    echo "copied $name -> $dest"
  else
    ln -s "$src" "$dest"
    echo "linked $dest -> $src"
  fi
}

for name in "${SKILLS[@]}"; do
  if [[ ! -d "$ROOT/skills/$name" ]]; then
    echo "install-user-skills.sh: missing $ROOT/skills/$name" >&2
    exit 1
  fi
  install_one "$name" "$HOME/.cursor/skills"
  install_one "$name" "$HOME/.agents/skills"
done

echo "install-user-skills.sh: done ($MODE)"
