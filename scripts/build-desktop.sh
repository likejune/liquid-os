#!/usr/bin/env bash
# Build the desktop components and install them into mkosi.extra/ so they
# end up in the read-only /usr of the image.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
DEST="$ROOT/mkosi.extra"
bash "$ROOT/scripts/fetch-upstream.sh"
for dir in "$ROOT"/third_party/*/; do
  name=$(basename "$dir")
  echo ">> building $name"
  if [ -f "$dir/justfile" ]; then
    (cd "$dir" && just build-release && just rootdir="$DEST" prefix=/usr install)
  else
    (cd "$dir" && cargo build --release)
  fi
done
echo ">> building Liquid Rust workspace"
(cd "$ROOT" && cargo build --workspace --release)
# Liquid design: theme, icons, wallpapers are already in mkosi.extra/usr/share
