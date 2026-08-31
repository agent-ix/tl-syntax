#!/usr/bin/env python3
"""Require every independent evidence verifier to remain wired into the shell gate."""

from __future__ import annotations

import sys
from pathlib import Path


ROOT = Path(__file__).resolve().parent.parent
SCRIPT = ROOT / "scripts" / "verify_evidence.sh"
REQUIRED = (
    "/usr/bin/python3 scripts/verify_evidence_tree.py",
    "/usr/bin/python3 scripts/verify_evidence_manifest.py \"$checksum\"",
    "/usr/bin/python3 scripts/finalize_collection.py --check \"${checksum%.sha256}\"",
)


def inspect(text: str) -> list[str]:
    errors = [f"evidence shell gate omits required command: {command}" for command in REQUIRED if command not in text]
    loop = "while IFS= read -r -d '' checksum; do"
    if loop not in text or "done < <(" not in text:
        errors.append("evidence shell gate omits the retained-record census loop")
    return errors


def main() -> int:
    if len(sys.argv) != 1:
        print("usage: check_evidence_shell_contract.py", file=sys.stderr)
        return 2
    try:
        errors = inspect(SCRIPT.read_text(encoding="utf-8"))
    except OSError as error:
        print(f"cannot read evidence shell gate: {error}", file=sys.stderr)
        return 2
    for error in errors:
        print(error, file=sys.stderr)
    return 1 if errors else 0


if __name__ == "__main__":
    raise SystemExit(main())
