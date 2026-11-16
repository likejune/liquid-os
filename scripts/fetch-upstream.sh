#!/usr/bin/env bash
# Fetch pinned upstream desktop components into third_party/ and apply
# Liquid OS patches from patches/<repo>/*.patch.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
mkdir -p "$ROOT/third_party"
grep -v '^#' "$ROOT/upstream.lock" | while read -r repo tag sha; do
  [ -n "${repo:-}" ] || continue
  name="${repo#*/}"
  dir="$ROOT/third_party/$name"
  if [ ! -d "$dir" ]; then
    git clone --depth 1 --branch "$tag" "https://github.com/$repo.git" "$dir"
  fi
  if [ "$sha" != "-" ]; then
    actual=$(git -C "$dir" rev-parse HEAD)
    [ "$actual" = "$sha" ] || { echo "$repo: expected $sha, got $actual" >&2; exit 1; }
  fi
  for p in "$ROOT"/patches/"$name"/*.patch; do
    [ -e "$p" ] || continue
    git -C "$dir" apply --check "$p" && git -C "$dir" apply "$p"
    echo ">> $name: applied $(basename "$p")"
  done
  if [ -f "$dir/Cargo.toml" ] && ! grep -q '^\[workspace\]' "$dir/Cargo.toml"; then
    printf '\n[workspace]\n' >> "$dir/Cargo.toml"
  fi
done
