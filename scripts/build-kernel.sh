#!/usr/bin/env bash
# Build the Fedora kernel with Liquid configuration and patches into a local
# package directory that mkosi automatically exposes as an RPM repository.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
WORK="${LIQUID_KERNEL_WORK:-$ROOT/.kernel-build}"
OUT="${LIQUID_KERNEL_OUT:-$ROOT/mkosi.packages}"
DIST="$WORK/fedora-kernel"

rm -rf "$WORK" "$OUT"
mkdir -p "$WORK"/{build,buildroot,rpms,srpms,tmp} "$OUT"

dnf -y install git fedpkg rpm-build dnf-plugins-core
git clone --depth 1 --branch f44 https://src.fedoraproject.org/rpms/kernel.git "$DIST"
cp "$ROOT/kernel/config/liquid.config" "$DIST/kernel-local"

patch_number=10000
for patch in "$ROOT"/kernel/patches/*.patch; do
  [ -e "$patch" ] || continue
  name="$(basename "$patch")"
  cp "$patch" "$DIST/$name"
  sed -i "/^Patch999999:/i Patch${patch_number}: ${name}" "$DIST/kernel.spec"
  sed -i "/^# END OF PATCH APPLICATIONS$/i ApplyOptionalPatch ${name}" "$DIST/kernel.spec"
  patch_number=$((patch_number + 1))
done

cd "$DIST"
fedpkg sources
dnf -y builddep kernel.spec

rpmbuild -bb \
  --with baseonly \
  --without debuginfo \
  --without doc \
  --without headers \
  --without cross_headers \
  --without kmap \
  --without kernel_abi_stablelists \
  --without kabidupchk \
  --without kabidwchk \
  --define "buildid .liquid" \
  --define "_sourcedir $DIST" \
  --define "_specdir $DIST" \
  --define "_builddir $WORK/build" \
  --define "_buildrootdir $WORK/buildroot" \
  --define "_rpmdir $WORK/rpms" \
  --define "_srcrpmdir $WORK/srpms" \
  --define "_tmppath $WORK/tmp" \
  kernel.spec

shopt -s nullglob
for name in kernel kernel-core kernel-modules kernel-modules-core kernel-modules-extra; do
  packages=("$WORK"/rpms/*/"${name}"-[0-9]*.rpm)
  if [ "${#packages[@]}" -eq 0 ]; then
    echo "Required package was not built: $name" >&2
    exit 1
  fi
  cp "${packages[@]}" "$OUT/"
done

for package in "$OUT"/*.rpm; do
  rpm -qp --queryformat "%{NAME} %{VERSION}-%{RELEASE} %{ARCH}\n" "$package"
done
