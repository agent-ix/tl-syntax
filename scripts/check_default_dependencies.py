#!/usr/bin/env python3
"""Prove the no_std feature boundary this crate promises embedded consumers (NFR-001).

Two facts, one producer.

The default normal dependency graph must contain only `tl-syntax` itself, and
the crate must compile in each of the four feature combinations downstream
consumers actually use. Those are the two halves of NFR-001 and they are checked
here together, because a crate with no dependencies that does not compile
without `alloc` has not kept the promise either.

`--json` emits the structured result Quoin transcribes. It is not a verdict and
it retains nothing: it reports one entry per combination with its own outcome,
and a combination that could not be attempted is reported as `unavailable`
rather than folded into a pass or a failure.

Exit status: 0 when every entry passed, 1 when one did not, 2 on usage error.
"""

from __future__ import annotations

import json
import os
import subprocess
import sys
from pathlib import Path
from typing import Any

ROOT = Path(__file__).resolve().parent.parent

PROTOCOL = "tl-syntax.feature-boundary/v1"

# The combinations downstream consumers build. `no-default-features` is the one
# embedded consumers get; the rest are the host-tool ladder up to everything on.
FEATURE_COMBINATIONS = (
    ("no-default-features", ["--lib", "--no-default-features"]),
    ("alloc", ["--lib", "--no-default-features", "--features", "alloc"]),
    ("serde", ["--lib", "--no-default-features", "--features", "serde"]),
    ("all-features", ["--lib", "--all-features"]),
)


def cargo() -> str:
    return os.environ.get("CARGO", "cargo")


def dependency_entry(tree_output: str | None = None) -> dict[str, Any]:
    if tree_output is None:
        result = subprocess.run(
            [
                cargo(),
                "tree",
                "--no-default-features",
                "--edges",
                "normal",
                "--prefix",
                "none",
            ],
            check=False,
            capture_output=True,
            text=True,
            cwd=ROOT,
        )
        if result.returncode != 0:
            return {
                "protocol": PROTOCOL,
                "symbol": "feature-boundary::default-dependency-graph",
                "check": "default_dependency_graph",
                "outcome": "unavailable",
                "traceIds": ["NFR-001-AC-1"],
                "detail": {"stderr": result.stderr.strip()},
            }
        tree_output = result.stdout
    dependencies = [line for line in tree_output.splitlines() if line.strip()]
    empty = len(dependencies) == 1 and dependencies[0].startswith("tl-syntax v")
    return {
        "protocol": PROTOCOL,
        "symbol": "feature-boundary::default-dependency-graph",
        "check": "default_dependency_graph",
        "outcome": "pass" if empty else "fail",
        "traceIds": ["NFR-001-AC-1"],
        "detail": {"graph": dependencies},
    }


def feature_entries() -> list[dict[str, Any]]:
    entries: list[dict[str, Any]] = []
    for name, arguments in FEATURE_COMBINATIONS:
        result = subprocess.run(
            [cargo(), "check", "--quiet", *arguments],
            check=False,
            capture_output=True,
            text=True,
            cwd=ROOT,
        )
        entries.append(
            {
                "protocol": PROTOCOL,
                "symbol": f"feature-boundary::compiles::{name}",
                "check": "compiles",
                "outcome": "pass" if result.returncode == 0 else "fail",
                "traceIds": ["NFR-001-AC-2"],
                "detail": {
                    "features": name,
                    "exit_code": result.returncode,
                    "stderr": result.stderr.strip()[:2000],
                },
            }
        )
    return entries


def main(argv: list[str]) -> int:
    as_json = False
    arguments = argv[1:]
    if arguments and arguments[0] == "--json":
        as_json = True
        arguments = arguments[1:]
    tree_output: str | None = None
    if len(arguments) == 2 and arguments[0] == "--tree-output":
        tree_output = Path(arguments[1]).read_text(encoding="utf-8")
    elif arguments:
        print(
            "usage: check_default_dependencies.py [--json] [--tree-output FILE]",
            file=sys.stderr,
        )
        return 2

    entries = [dependency_entry(tree_output)]
    if as_json:
        entries.extend(feature_entries())
        print(
            json.dumps(
                {"protocol": PROTOCOL, "entries": entries}, indent=2, sort_keys=True
            )
        )
        return 0 if all(entry["outcome"] == "pass" for entry in entries) else 1

    entry = entries[0]
    if entry["outcome"] != "pass":
        print(
            f"default normal dependency graph is not empty ({entry['outcome']}):",
            file=sys.stderr,
        )
        print("\n".join(entry["detail"].get("graph", [])), file=sys.stderr)
        return 1
    print("default normal dependency graph contains no dependencies")
    return 0


if __name__ == "__main__":
    raise SystemExit(main(sys.argv))
