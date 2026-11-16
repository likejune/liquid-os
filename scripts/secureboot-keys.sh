#!/usr/bin/env bash
# Generate Liquid OS signing keys (Secure Boot, dm-verity, update signatures).
# Run ONCE on an offline machine. Never commit the private keys.
set -euo pipefail
OUT="${1:-keys}"
mkdir -p "$OUT" && chmod 700 "$OUT"
# 1. Secure Boot / UKI / verity signing key (used by mkosi)
openssl req -new -x509 -newkey rsa:4096 -sha256 -days 3650 -nodes \
  -subj "/CN=Liquid OS Secure Boot/" \
  -keyout "$OUT/mkosi.key" -out "$OUT/mkosi.crt"
# 2. Update signing key (systemd-sysupdate verifies SHA256SUMS.gpg)
gpg --batch --homedir "$OUT/gnupg" --passphrase '' --quick-gen-key \
  "Liquid OS Updates <updates@liquid-os.invalid>" ed25519 sign 5y
gpg --batch --homedir "$OUT/gnupg" --export > "$OUT/import-pubring.gpg"
echo "Keys written to $OUT/."
echo "Copy $OUT/import-pubring.gpg to mkosi.extra/usr/lib/systemd/import-pubring.gpg"
echo "Store mkosi.key and gnupg/ as GitHub secrets (see docs/SECURE-BOOT.md)."
