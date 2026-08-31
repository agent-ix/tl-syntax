#!/usr/bin/env bash
set -euo pipefail

python3 scripts/verify_evidence_tree.py
found=0
while IFS= read -r -d '' checksum; do
  found=1
  python3 scripts/verify_evidence_manifest.py "$checksum"
  python3 scripts/finalize_collection.py --check "${checksum%.sha256}"
done < <(find evidence -maxdepth 1 -type f -name '*.sha256' ! -name 'STATIC.sha256' -print0 | sort -z)

if [[ $found -eq 0 ]]; then
  echo "no retained evidence checksum manifests found" >&2
  exit 1
fi
