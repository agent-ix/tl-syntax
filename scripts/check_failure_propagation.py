#!/usr/bin/env python3
"""Prove every mandatory local-CI recipe propagates command failures."""

from __future__ import annotations

import argparse
import os
import re
import shlex
import subprocess
import sys
from pathlib import Path


ROOT = Path(__file__).resolve().parent.parent
PROBES = {
    "fmt-check": "CARGO",
    "check-features": "CARGO",
    "check-default-dependencies": "PYTHON",
    "lint": "CARGO",
    "test": "CARGO",
    "check-corpus": "SHA256SUM",
    "deny": "CARGO",
    "audit-unsafe": "BASH",
    "evidence-tool": "PYTHON",
    "spec": "QUIRE",
    "verify-evidence": "BASH",
}
GUARD_TARGET = "check-failure-propagation"
TARGET = re.compile(r"^([A-Za-z0-9_.-]+):(?:\s+(.*?))?\s*$")
SWALLOWED_FAILURE = re.compile(r"\|\|\s*(?:true|:)(?:\s|$)")


def parse_makefile(text: str) -> tuple[dict[str, list[str]], dict[str, list[str]]]:
    dependencies: dict[str, list[str]] = {}
    recipes: dict[str, list[str]] = {}
    current: str | None = None
    for line in text.splitlines():
        if line.startswith("\t"):
            if current is not None:
                recipes.setdefault(current, []).append(line[1:])
            continue
        current = None
        if not line or line[0].isspace() or line.startswith("#"):
            continue
        match = TARGET.fullmatch(line)
        if match is None or match.group(1).startswith("."):
            continue
        current = match.group(1)
        dependencies[current] = (match.group(2) or "").split()
    return dependencies, recipes


def inspect_makefile(path: Path) -> list[str]:
    try:
        dependencies, recipes = parse_makefile(path.read_text(encoding="utf-8"))
    except OSError as error:
        return [f"cannot read Makefile {path}: {error}"]
    errors: list[str] = []
    expected = set(PROBES) | {GUARD_TARGET}
    observed = set(dependencies.get("ci", []))
    if observed != expected:
        errors.append(
            "ci prerequisite set does not match the failure probes: "
            f"missing={sorted(expected - observed)}, extra={sorted(observed - expected)}"
        )
    for target in sorted(expected):
        commands = recipes.get(target, [])
        if not commands:
            errors.append(f"mandatory target {target} has no recipe")
            continue
        for command in commands:
            stripped = command.lstrip()
            modifiers = ""
            while stripped[:1] in {"@", "+", "-"}:
                modifiers += stripped[0]
                stripped = stripped[1:].lstrip()
            if "-" in modifiers:
                errors.append(f"mandatory target {target} ignores a recipe failure: {command}")
            if SWALLOWED_FAILURE.search(stripped):
                errors.append(f"mandatory target {target} swallows a recipe failure: {command}")
    return errors


def probe_targets(makefile: Path, root: Path) -> list[str]:
    errors: list[str] = []
    make = shlex.split(os.environ.get("MAKE", "make"))
    if not make:
        return ["MAKE does not identify an executable"]
    for target, variable in PROBES.items():
        try:
            result = subprocess.run(
                [*make, "--no-print-directory", "-f", str(makefile), target, f"{variable}=false"],
                cwd=root,
                check=False,
                stdout=subprocess.DEVNULL,
                stderr=subprocess.DEVNULL,
            )
        except FileNotFoundError:
            return [f"required Make executable is unavailable: {make[0]}"]
        if result.returncode == 0:
            errors.append(f"mandatory target {target} swallowed a deliberately failing {variable}")
    return errors


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--makefile", type=Path, default=ROOT / "Makefile")
    parser.add_argument("--inspect-only", action="store_true")
    args = parser.parse_args()
    makefile = args.makefile.resolve()
    errors = inspect_makefile(makefile)
    if not errors and not args.inspect_only:
        errors.extend(probe_targets(makefile, ROOT))
    for error in errors:
        print(error, file=sys.stderr)
    if errors:
        return 1
    print(f"all {len(PROBES)} mandatory local-CI targets propagate failures")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
