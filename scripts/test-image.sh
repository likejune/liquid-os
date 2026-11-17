#!/usr/bin/env bash
# Boot the built image and require userspace to reach a usable target.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"
rm -f boot.log

set +e
timeout 180 mkosi qemu -- -nographic < /dev/null 2>&1 | tee boot.log
qemu_status=${PIPESTATUS[0]}
set -e

if grep -Eiq 'Kernel panic|VFS: Unable to mount root|Failed to start systemd-repart.service' boot.log; then
  echo "Liquid OS failed during kernel boot" >&2
  exit 1
fi

if grep -Eiq 'Welcome to Liquid OS|liquid login:|Reached target (Multi-User System|Graphical Interface)' boot.log; then
  echo "Liquid OS reached a usable userspace"
  exit 0
fi

echo "Liquid OS did not reach a usable userspace (qemu status: $qemu_status)" >&2
exit 1
