#!/usr/bin/env bash
set -euo pipefail

/usr/bin/python3 scripts/verify_evidence_tree.py
/usr/bin/python3 scripts/evidence_profile.py --verify-census
found=0
while IFS= read -r -d '' checksum; do
  found=1
  /usr/bin/python3 scripts/verify_evidence_manifest.py "$checksum"
  /usr/bin/python3 scripts/finalize_collection.py --check "${checksum%.sha256}"
done < <(/usr/bin/find evidence -maxdepth 1 -type f -name '*.sha256' ! -name 'STATIC.sha256' -print0 | /usr/bin/sort -z)

if [[ $found -eq 0 ]]; then
  echo "no retained evidence checksum manifests found" >&2
  exit 1
fi
