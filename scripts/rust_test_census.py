#!/usr/bin/env python3
"""Bind requirement-tagged Rust tests to the tests Cargo actually compiles."""

from __future__ import annotations

import re
import subprocess
from pathlib import Path


TRACE_TEST = re.compile(
    r"//\s*Trace:[^\n]*\n(?:\s*#\[[^\n]+\]\n)*\s*fn\s+([A-Za-z_][A-Za-z0-9_]*)\s*\(",
    re.MULTILINE,
)
TEST_LINE = re.compile(r"^(.+): test$")


def tagged_test_names(root: Path) -> set[str]:
    names: set[str] = set()
    for source in root.rglob("*.rs"):
        relative = source.relative_to(root)
        if relative.parts and relative.parts[0] in {".git", "target"}:
            continue
        for name in TRACE_TEST.findall(source.read_text(encoding="utf-8")):
            if name in names:
                raise ValueError(f"duplicate requirement-tagged Rust test name: {name}")
            names.add(name)
    return names


def git_tagged_test_names(root: Path, revision: str) -> set[str]:
    paths = subprocess.run(
        ["/usr/bin/git", "ls-tree", "-r", "--name-only", revision], cwd=root,
        check=True, capture_output=True, text=True,
    ).stdout.splitlines()
    names: set[str] = set()
    for path in paths:
        if not path.endswith(".rs") or path.startswith("target/"):
            continue
        text = subprocess.run(
            ["/usr/bin/git", "show", f"{revision}:{path}"], cwd=root,
            check=True, capture_output=True, text=True,
        ).stdout
        for name in TRACE_TEST.findall(text):
            if name in names:
                raise ValueError(f"duplicate requirement-tagged Rust test name: {name}")
            names.add(name)
    return names


def listed_test_names(output: str) -> tuple[set[str], list[str]]:
    qualified: list[str] = []
    names: set[str] = set()
    for line in output.splitlines():
        match = TEST_LINE.fullmatch(line.strip())
        if match is None or " - (line " in match.group(1):
            continue
        qualified.append(match.group(1))
        name = match.group(1).rsplit("::", 1)[-1]
        if name in names:
            raise ValueError(f"compiled Rust test names are not unique: {name}")
        names.add(name)
    return names, qualified


def cargo_list(cargo: str, root: Path, environment: dict[str, str], ignored: bool = False) -> str:
    arguments = [cargo, "test", "--all-features", "--"]
    if ignored:
        arguments.append("--ignored")
    arguments.append("--list")
    result = subprocess.run(
        arguments, cwd=root, check=False, capture_output=True, text=True, env=environment,
    )
    if result.returncode != 0:
        raise RuntimeError(
            f"cargo test census failed ({result.returncode}): {(result.stdout + result.stderr).strip()}"
        )
    return result.stdout


def inspect_live(root: Path, cargo: str, environment: dict[str, str]) -> list[str]:
    try:
        expected = tagged_test_names(root)
        observed, _ = listed_test_names(cargo_list(cargo, root, environment))
        ignored, ignored_qualified = listed_test_names(
            cargo_list(cargo, root, environment, ignored=True)
        )
    except (OSError, RuntimeError, ValueError) as error:
        return [f"cannot derive the compiled Rust test census: {error}"]
    errors: list[str] = []
    if observed != expected:
        errors.append(
            "compiled Rust test census disagrees with requirement-tagged source tests: "
            f"missing={sorted(expected - observed)}, extra={sorted(observed - expected)}"
        )
    if ignored:
        errors.append(f"compiled Rust tests are ignored: {ignored_qualified}")
    return errors
