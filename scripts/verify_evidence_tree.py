#!/usr/bin/env python3
"""Verify that every retained evidence entry is inside an anchored boundary."""

from __future__ import annotations

import hashlib
import json
import os
import re
import subprocess
import sys
from pathlib import Path


ROOT = Path(__file__).resolve().parent.parent
LINE = re.compile(r"^([0-9a-f]{64})  (.+)$")
ANCHORS = Path("evidence/ANCHORS")
STATIC_MANIFEST = Path("evidence/STATIC.sha256")


def sha256(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def parse_manifest(path: Path, root: Path) -> tuple[dict[Path, str], list[str]]:
    entries: dict[Path, str] = {}
    errors: list[str] = []
    try:
        lines = path.read_text(encoding="utf-8").splitlines()
    except OSError as error:
        return {}, [f"cannot read evidence manifest {path}: {error}"]
    for number, line in enumerate(lines, start=1):
        match = LINE.fullmatch(line)
        if match is None:
            errors.append(f"{path}:{number} is malformed")
            continue
        relative = Path(match.group(2))
        if relative.is_absolute() or ".." in relative.parts or relative in entries:
            errors.append(f"{path}:{number} has an unsafe or duplicate path: {relative}")
            continue
        try:
            (root / relative).resolve().relative_to(root.resolve())
        except ValueError:
            errors.append(f"{path}:{number} escapes the repository: {relative}")
            continue
        entries[relative] = match.group(1)
    return entries, errors


def active_collection(path: Path, root: Path) -> bool:
    marker = path / ".collecting"
    token = os.environ.get("TL_SYNTAX_COLLECTION_TOKEN")
    if not marker.is_file() or not token:
        return False
    tracked = subprocess.run(
        ["git", "ls-files", "--error-unmatch", str(marker.relative_to(root))],
        cwd=root,
        check=False,
        stdout=subprocess.DEVNULL,
        stderr=subprocess.DEVNULL,
    )
    if tracked.returncode == 0:
        return False
    try:
        value = json.loads(marker.read_text(encoding="utf-8"))
        revision = subprocess.run(
            ["git", "rev-parse", "HEAD"],
            cwd=root,
            check=True,
            capture_output=True,
            text=True,
        ).stdout.strip()
        pid = value["pid"]
        if value["token"] != token or value["sourceRevision"] != revision:
            return False
        if not isinstance(pid, int) or pid <= 1:
            return False
        os.kill(pid, 0)
        command_line = Path(f"/proc/{pid}/cmdline").read_bytes().replace(b"\0", b" ")
        if b"scripts/collect_evidence.sh" not in command_line:
            return False
    except (KeyError, OSError, ValueError, json.JSONDecodeError, subprocess.CalledProcessError):
        return False
    return True


def verify_tree(root: Path = ROOT) -> list[str]:
    evidence = root / "evidence"
    errors: list[str] = []
    if not evidence.is_dir():
        return ["evidence directory is missing"]

    for path in evidence.rglob("*"):
        if path.is_symlink():
            errors.append(f"retained evidence contains a symlink: {path.relative_to(root)}")

    record_dirs: set[Path] = set()
    active_dirs: set[Path] = set()
    for path in sorted(item for item in evidence.iterdir() if item.is_dir()):
        checksum = path.with_name(f"{path.name}.sha256")
        if checksum.is_file():
            record_dirs.add(path)
        elif active_collection(path, root):
            active_dirs.add(path)

    record_manifests = {
        path.relative_to(root)
        for path in evidence.glob("*.sha256")
        if path.name != STATIC_MANIFEST.name
    }
    for relative in sorted(record_manifests):
        if not (root / relative).with_suffix("").is_dir():
            errors.append(f"record manifest has no sibling directory: {relative}")
    for path in sorted(record_dirs):
        relative = path.with_name(f"{path.name}.sha256").relative_to(root)
        if relative not in record_manifests:
            errors.append(f"retained record has no manifest: {path.relative_to(root)}")

    expected_anchors = record_manifests | {STATIC_MANIFEST}
    anchors, anchor_errors = parse_manifest(root / ANCHORS, root)
    errors.extend(anchor_errors)
    if set(anchors) != expected_anchors:
        errors.append(
            "anchor census mismatch: "
            f"missing={sorted(map(str, expected_anchors - set(anchors)))}, "
            f"extra={sorted(map(str, set(anchors) - expected_anchors))}"
        )
    for relative, expected in anchors.items():
        path = root / relative
        if not path.is_file():
            errors.append(f"anchored evidence entry is missing: {relative}")
        elif sha256(path) != expected:
            errors.append(f"anchored evidence digest mismatch: {relative}")

    static, static_errors = parse_manifest(root / STATIC_MANIFEST, root)
    errors.extend(static_errors)
    excluded_files = {ANCHORS, STATIC_MANIFEST} | record_manifests
    actual_static: set[Path] = set()
    for path in evidence.rglob("*"):
        if not path.is_file() or path.is_symlink():
            continue
        if any(record in path.parents for record in record_dirs):
            continue
        if any(active in path.parents for active in active_dirs):
            continue
        relative = path.relative_to(root)
        if relative not in excluded_files:
            actual_static.add(relative)
    if set(static) != actual_static:
        errors.append(
            "static evidence census mismatch: "
            f"missing={sorted(map(str, actual_static - set(static)))}, "
            f"extra={sorted(map(str, set(static) - actual_static))}"
        )
    for relative, expected in static.items():
        path = root / relative
        if not path.is_file():
            errors.append(f"static evidence entry is missing: {relative}")
        elif sha256(path) != expected:
            errors.append(f"static evidence digest mismatch: {relative}")
    return errors


def main() -> int:
    if len(sys.argv) == 1:
        root = ROOT
    elif len(sys.argv) == 3 and sys.argv[1] == "--root":
        root = Path(sys.argv[2])
    else:
        print("usage: verify_evidence_tree.py [--root REPOSITORY]", file=sys.stderr)
        return 2
    errors = verify_tree(root)
    for error in errors:
        print(error, file=sys.stderr)
    return 1 if errors else 0


if __name__ == "__main__":
    raise SystemExit(main())
