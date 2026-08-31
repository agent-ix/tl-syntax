#!/usr/bin/env python3
"""Verify exact membership and digests of a retained evidence directory."""

from __future__ import annotations

import hashlib
import re
import sys
from pathlib import Path


LINE = re.compile(r"^([0-9a-f]{64})  (.+)$")


def sha256(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def verify(checksum_path: Path) -> list[str]:
    evidence_dir = checksum_path.with_suffix("")
    errors: list[str] = []
    expected: dict[Path, str] = {}
    for number, line in enumerate(
        checksum_path.read_text(encoding="utf-8").splitlines(), start=1
    ):
        match = LINE.fullmatch(line)
        if match is None:
            errors.append(f"malformed checksum line {number}: {line!r}")
            continue
        relative = Path(match.group(2))
        try:
            member = relative.relative_to(evidence_dir)
        except ValueError:
            errors.append(f"checksum path escapes evidence directory: {relative}")
            continue
        if member.is_absolute() or ".." in member.parts or member in expected:
            errors.append(f"unsafe or duplicate checksum path: {relative}")
            continue
        expected[member] = match.group(1)

    actual = {
        path.relative_to(evidence_dir)
        for path in evidence_dir.rglob("*")
        if path.is_file() and not path.is_symlink()
    }
    symlinks = [path for path in evidence_dir.rglob("*") if path.is_symlink()]
    for path in symlinks:
        errors.append(f"retained evidence contains a symlink: {path}")
    for member in sorted(actual - set(expected)):
        errors.append(f"unlisted retained artifact: {member}")
    for member in sorted(set(expected) - actual):
        errors.append(f"missing retained artifact: {member}")
    for member in sorted(actual & set(expected)):
        if sha256(evidence_dir / member) != expected[member]:
            errors.append(f"retained artifact digest mismatch: {member}")
    return errors


def main() -> int:
    if len(sys.argv) != 2:
        print("usage: verify_evidence_manifest.py EVIDENCE.sha256", file=sys.stderr)
        return 2
    errors = verify(Path(sys.argv[1]))
    for error in errors:
        print(error, file=sys.stderr)
    return 1 if errors else 0


if __name__ == "__main__":
    raise SystemExit(main())
