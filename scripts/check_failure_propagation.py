#!/usr/bin/env python3
"""Prove every mandatory local-CI recipe propagates command failures."""

from __future__ import annotations

import argparse
import os
import re
import shlex
import subprocess
import sys
import tempfile
from pathlib import Path


ROOT = Path(__file__).resolve().parent.parent
PROBES = {
    "fmt-check", "check-features", "check-default-dependencies", "lint", "test",
    "check-corpus", "deny", "audit-unsafe", "evidence-tool", "spec", "verify-evidence",
}
GUARD_TARGET = "check-failure-propagation"
TARGET = re.compile(r"^([A-Za-z0-9_.-]+):(?:\s+(.*?))?\s*$")
SHELL_CONTROL = re.compile(r"&&|\|\||[;|]")
IGNORE_ATTRIBUTE = re.compile(r"#\s*\[[^\]]*\bignore\b[^\]]*\]")
MAKEFLAGS_ASSIGNMENT = re.compile(r"^\s*MAKEFLAGS\s*(?::|\+|\?)?=\s*(.*)$")


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


def makeflags_ignore_errors(value: str) -> bool:
    """Return whether GNU Make flags request ignored recipe failures."""
    try:
        tokens = shlex.split(value)
    except ValueError:
        return True
    for token in tokens:
        if token == "--ignore-errors":
            return True
        if token.startswith("-") and not token.startswith("--") and "i" in token[1:]:
            return True
        if token and not token.startswith("-") and "=" not in token and "i" in token:
            return True
    return False


def inspect_ignored_tests(root: Path) -> list[str]:
    errors: list[str] = []
    for directory in ("src", "tests"):
        source_root = root / directory
        if not source_root.exists():
            continue
        for source in source_root.rglob("*.rs"):
            if IGNORE_ATTRIBUTE.search(source.read_text(encoding="utf-8")):
                errors.append(f"{source.relative_to(root)} disables a Rust test with #[ignore]")
    return errors


def inspect_makefile(path: Path, root: Path = ROOT) -> list[str]:
    try:
        text = path.read_text(encoding="utf-8")
        dependencies, recipes = parse_makefile(text)
    except OSError as error:
        return [f"cannot read Makefile {path}: {error}"]
    errors: list[str] = []
    for number, line in enumerate(text.splitlines(), start=1):
        if re.match(r"^\s*\.IGNORE\s*(?::|$)", line):
            errors.append(f"Makefile:{number} declares .IGNORE")
        assignment = MAKEFLAGS_ASSIGNMENT.match(line)
        if assignment is not None and makeflags_ignore_errors(assignment.group(1)):
            errors.append(f"Makefile:{number} enables MAKEFLAGS ignore-errors")
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
            if SHELL_CONTROL.search(stripped):
                errors.append(
                    f"mandatory target {target} uses forbidden shell control operators: {command}"
                )
    errors.extend(inspect_ignored_tests(root))
    return errors


def inspect_expanded_recipes(makefile: Path, root: Path) -> list[str]:
    errors: list[str] = []
    make = shlex.split(os.environ.get("MAKE", "make"))
    if not make:
        return ["MAKE does not identify an executable"]
    clean_env = dict(os.environ)
    clean_env.pop("MAKEFLAGS", None)
    clean_env.pop("PYTHONOPTIMIZE", None)
    for target in sorted(PROBES):
        try:
            result = subprocess.run(
                [*make, "--no-print-directory", "-n", "-f", str(makefile), target],
                cwd=root,
                check=False,
                capture_output=True,
                text=True,
                env=clean_env,
            )
        except FileNotFoundError:
            return [f"required Make executable is unavailable: {make[0]}"]
        if result.returncode != 0:
            errors.append(f"cannot expand mandatory target {target}: {result.stderr.strip()}")
            continue
        for command in result.stdout.splitlines():
            if SHELL_CONTROL.search(command):
                errors.append(
                    f"expanded mandatory target {target} uses forbidden shell control operators: {command}"
                )
    return errors


def probe_command_positions(makefile: Path) -> list[str]:
    """Substitute false at every recipe position and require Make to fail."""
    _, recipes = parse_makefile(makefile.read_text(encoding="utf-8"))
    errors: list[str] = []
    make = shlex.split(os.environ.get("MAKE", "make"))
    if not make:
        return ["MAKE does not identify an executable"]
    with tempfile.TemporaryDirectory() as directory:
        probe = Path(directory) / "Makefile"
        clean_env = dict(os.environ)
        clean_env.pop("MAKEFLAGS", None)
        for target in sorted(PROBES):
            commands = recipes.get(target, [])
            for selected in range(len(commands)):
                lines = [f".PHONY: {target}", f"{target}:"]
                for index, command in enumerate(commands):
                    stripped = command.lstrip()
                    modifiers = ""
                    while stripped[:1] in {"@", "+", "-"}:
                        modifiers += stripped[0]
                        stripped = stripped[1:].lstrip()
                    lines.append(f"\t{modifiers}{'false' if index == selected else 'true'}")
                probe.write_text("\n".join(lines) + "\n", encoding="utf-8")
                try:
                    result = subprocess.run(
                        [*make, "--no-print-directory", "-f", str(probe), target],
                        check=False,
                        stdout=subprocess.DEVNULL,
                        stderr=subprocess.DEVNULL,
                        env=clean_env,
                    )
                except FileNotFoundError:
                    return [f"required Make executable is unavailable: {make[0]}"]
                if result.returncode == 0:
                    errors.append(
                        f"mandatory target {target} swallowed failure at recipe position "
                        f"{selected + 1}"
                    )
    return errors


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--makefile", type=Path, default=ROOT / "Makefile")
    parser.add_argument("--inspect-only", action="store_true")
    parser.add_argument("--static-only", action="store_true")
    args = parser.parse_args()
    makefile = args.makefile.resolve()
    errors = inspect_makefile(makefile)
    if makeflags_ignore_errors(os.environ.get("MAKEFLAGS", "")):
        errors.append("ambient MAKEFLAGS enables ignored recipe failures")
    if os.environ.get("PYTHONOPTIMIZE") or sys.flags.optimize:
        errors.append("optimized Python disables policy assertions")
    if not errors and not args.static_only:
        errors.extend(inspect_expanded_recipes(makefile, ROOT))
    if not errors and not args.inspect_only and not args.static_only:
        errors.extend(probe_command_positions(makefile))
    for error in errors:
        print(error, file=sys.stderr)
    if errors:
        return 1
    print(f"all {len(PROBES)} mandatory local-CI targets propagate failures")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
