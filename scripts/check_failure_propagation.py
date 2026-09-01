#!/usr/bin/env python3
"""Prove every mandatory local-CI recipe propagates command failures."""

from __future__ import annotations

import argparse
import os
import re
import shlex
import shutil
import subprocess
import sys
import tempfile
from pathlib import Path

import rust_test_census
import tool_identity


ROOT = Path(__file__).resolve().parent.parent
PROBES = {
    "fmt-check", "check-features", "check-default-dependencies", "lint", "test",
    "check-corpus", "deny", "audit-unsafe", "evidence-tool", "spec", "msrv", "rustdoc",
    "verify-evidence",
}
COLLECTION_PROBES = PROBES - {"verify-evidence"}
QUALIFICATION_TARGET = "check-tool-identities"
GUARD_TARGET = "check-failure-propagation"
TARGET = re.compile(r"^([A-Za-z0-9_.-]+):(?:\s+(.*?))?\s*$")
SHELL_CONTROL = re.compile(r"&&|\|\||&(?!&)|[;|]")
CONTROL_ASSIGNMENT = re.compile(
    r"^\s*(?:(?:export|override|unexport|private)\s+)*"
    r"(MAKEFLAGS|SHELL|\.SHELLFLAGS|MAKE)\s*(?:::?=|:::=|\+=|\?=|!=|=)\s*(.*)$"
)
CONTROL_DIRECTIVE = re.compile(r"^\s*\.(IGNORE|SILENT|ONESHELL|DEFAULT)\s*(?::|$)")
CONTROL_DEFINE = re.compile(
    r"^\s*(?:(?:override|export|private)\s+)*define\s+"
    r"(MAKEFLAGS|SHELL|\.SHELLFLAGS|MAKE)\b"
)
CONTROL_EVAL = re.compile(r"\$\s*[({]\s*eval\b")
TARGET_SCOPED_CONTROL = re.compile(
    r"^\s*[^:#=]+:\s*(?:(?:export|override|unexport|private)\s+)*"
    r"(MAKEFLAGS|SHELL|\.SHELLFLAGS|MAKE)\s*(?:::?=|:::=|\+=|\?=|!=|=)"
)


class CensusUnavailable(RuntimeError):
    """The portable compiled-test census cannot execute on this host."""


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
        if current in dependencies:
            raise ValueError(f"duplicate target rule can overwrite policy state: {current}")
        dependencies[current] = (match.group(2) or "").split()
    return dependencies, recipes


def makeflags_ignore_errors(value: str) -> bool:
    """Return whether GNU Make flags can change what CI executes."""
    try:
        tokens = shlex.split(value)
    except ValueError:
        return True
    for token in tokens:
        if token.startswith(("--jobs", "--jobserver-", "--load-average", "--output-sync")):
            continue
        if token in {"--print-directory", "--no-print-directory"}:
            continue
        if re.fullmatch(r"-(?:j|l|O)(?:[0-9.]+|[A-Za-z]+)?", token):
            continue
        if token == "-w":
            continue
        if token:
            return True
    return False


def portable_census_environment() -> dict[str, str]:
    """Return the complete ambient allowlist for the portable Rust test census."""
    allowed = (
        "HOME", "PATH", "USER", "LANG", "LC_ALL", "TMPDIR",
    )
    environment = {name: os.environ[name] for name in allowed if name in os.environ}
    environment.setdefault("PATH", os.defpath)
    return environment


def inspect_test_census() -> list[str]:
    cargo = shutil.which("cargo")
    if cargo is None:
        raise CensusUnavailable("Cargo is unavailable for the compiled Rust test census")
    return rust_test_census.inspect_live(
        ROOT, cargo, portable_census_environment(),
    )


def inspect_execution_controls(text: str) -> list[str]:
    """Reject file-wide and target-scoped state that can change recipe execution."""
    errors: list[str] = []
    for number, line in enumerate(text.splitlines(), start=1):
        target_scoped = TARGET_SCOPED_CONTROL.match(line)
        if target_scoped is not None:
            errors.append(
                f"Makefile:{number} assigns target-scoped execution control "
                f"{target_scoped.group(1)}"
            )
        directive = CONTROL_DIRECTIVE.match(line)
        if directive is not None:
            errors.append(f"Makefile:{number} declares .{directive.group(1)}")
        assignment = CONTROL_ASSIGNMENT.match(line)
        if assignment is not None:
            name, value = assignment.groups()
            if name != "MAKEFLAGS" or makeflags_ignore_errors(value):
                errors.append(f"Makefile:{number} assigns execution control {name}")
        define = CONTROL_DEFINE.match(line)
        if define is not None:
            errors.append(f"Makefile:{number} defines execution control {define.group(1)}")
        if CONTROL_EVAL.search(line):
            errors.append(f"Makefile:{number} uses eval, which can hide execution controls")
        if re.match(r"^\s*-?include\b", line):
            errors.append(f"Makefile:{number} includes an uninspected Make fragment")
    return errors


def inspect_makefile(path: Path, root: Path = ROOT) -> list[str]:
    try:
        text = path.read_text(encoding="utf-8")
    except OSError as error:
        return [f"cannot read Makefile {path}: {error}"]
    errors = inspect_execution_controls(text)
    try:
        dependencies, recipes = parse_makefile(text)
    except ValueError as error:
        errors.append(f"Makefile structure is ambiguous: {error}")
        return errors
    expected = set(PROBES) | {GUARD_TARGET}
    observed = set(dependencies.get("ci", []))
    if observed != expected:
        errors.append(
            "ci prerequisite set does not match the failure probes: "
            f"missing={sorted(expected - observed)}, extra={sorted(observed - expected)}"
        )
    candidate_expected = COLLECTION_PROBES | {GUARD_TARGET, QUALIFICATION_TARGET}
    candidate_observed = set(dependencies.get("ci-for-evidence", []))
    if candidate_observed != candidate_expected:
        errors.append(
            "candidate CI prerequisite set does not match the failure probes: "
            f"missing={sorted(candidate_expected - candidate_observed)}, "
            f"extra={sorted(candidate_observed - candidate_expected)}"
        )
    for target in sorted(expected | {QUALIFICATION_TARGET}):
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
    return errors


def inspect_expanded_recipes(makefile: Path, root: Path) -> list[str]:
    errors: list[str] = []
    make = ["/usr/bin/make"]
    clean_env = dict(os.environ)
    clean_env.pop("MAKEFLAGS", None)
    clean_env.pop("PYTHONOPTIMIZE", None)
    for target in sorted(PROBES | {QUALIFICATION_TARGET}):
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
    text = makefile.read_text(encoding="utf-8")
    control_errors = inspect_execution_controls(text)
    if control_errors:
        return [
            "command-position probe refuses a Makefile with execution controls: " + error
            for error in control_errors
        ]
    _, recipes = parse_makefile(text)
    errors: list[str] = []
    make = ["/usr/bin/make"]
    with tempfile.TemporaryDirectory() as directory:
        probe = Path(directory) / "Makefile"
        clean_env = dict(os.environ)
        clean_env.pop("MAKEFLAGS", None)
        for target in sorted(PROBES | {QUALIFICATION_TARGET}):
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
    if os.environ.get("MAKE"):
        errors.append("ambient MAKE override is not permitted")
    if not args.static_only:
        try:
            errors.extend(inspect_test_census())
        except CensusUnavailable as error:
            print(error, file=sys.stderr)
            return 2
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
