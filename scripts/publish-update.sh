#!/usr/bin/env bash
# Publish a built image as an update for systemd-sysupdate.
# Usage: scripts/publish-update.sh <mkosi.output dir> <destination dir or rsync target>
set -euo pipefail
SRC="${1:?mkosi.output directory}"
DST="${2:?destination}"
GNUPGHOME="${GNUPGHOME:-keys/gnupg}"
export GNUPGHOME
STAGE=$(mktemp -d)
trap 'rm -rf "$STAGE"' EXIT
# Files produced by mkosi with SplitArtifacts=yes; names match sysupdate.d/*.transfer
cp "$SRC"/liquid-os_*.usr-x86-64.*.raw "$STAGE"/
cp "$SRC"/liquid-os_*.usr-x86-64-verity.*.raw "$STAGE"/
cp "$SRC"/liquid-os_*.efi "$STAGE"/
( cd "$STAGE" && sha256sum -- * > SHA256SUMS )
gpg --batch --yes --detach-sign --output "$STAGE/SHA256SUMS.gpg" "$STAGE/SHA256SUMS"
# Merge with already published versions: keep old files, append new sums
rsync -av "$STAGE"/ "$DST"/
echo "Published: $(ls "$STAGE")"
