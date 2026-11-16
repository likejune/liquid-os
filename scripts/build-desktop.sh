#!/usr/bin/env bash
# Build the desktop components and install them into mkosi.extra/ so they
# end up in the read-only /usr of the image.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
DEST="$ROOT/mkosi.extra"
bash "$ROOT/scripts/fetch-upstream.sh"
for dir in "$ROOT"/third_party/*/; do
  if [ -f "$dir/Cargo.toml" ]; then
    (cd "$dir" && cargo metadata --no-deps --format-version 1 >/dev/null)
  fi
done
components=("$ROOT/third_party/xdg-desktop-portal-cosmic")
for dir in "$ROOT"/third_party/*/; do
  [ "$dir" = "${components[0]}/" ] && continue
  components+=("$dir")
done
for dir in "${components[@]}"; do
  name=$(basename "$dir")
  echo ">> building $name"
  if [ -f "$dir/justfile" ]; then
    if (cd "$dir" && just --show build-release >/dev/null 2>&1); then
      build_recipe=build-release
    else
      build_recipe=build
    fi
    (cd "$dir" && just "$build_recipe" && just rootdir="$DEST" prefix=/usr install)
  else
    (cd "$dir" && cargo build --release)
  fi
done
echo ">> building Liquid Rust workspace"
(cd "$ROOT" && cargo build --workspace --release)
# Liquid design: theme, icons, wallpapers are already in mkosi.extra/usr/share
