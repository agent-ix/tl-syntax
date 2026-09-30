#!/usr/bin/env python3
"""Prove the no_std feature boundary this crate promises embedded consumers (NFR-001).

The default normal dependency graph must contain only `tl-syntax` itself.
`make check-features` compiles the feature combinations.

Exit status: 0 when the graph is empty, 1 when it is not, 2 on usage error.
"""

from __future__ import annotations

import os
import subprocess
import sys
from pathlib import Path
from typing import Any

ROOT = Path(__file__).resolve().parent.parent


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
                "outcome": "unavailable",
                "detail": {"stderr": result.stderr.strip()},
            }
        tree_output = result.stdout
    dependencies = [line for line in tree_output.splitlines() if line.strip()]
    empty = len(dependencies) == 1 and dependencies[0].startswith("tl-syntax v")
    return {
        "outcome": "pass" if empty else "fail",
        "detail": {"graph": dependencies},
    }


def main(argv: list[str]) -> int:
    arguments = argv[1:]
    tree_output: str | None = None
    if len(arguments) == 2 and arguments[0] == "--tree-output":
        tree_output = Path(arguments[1]).read_text(encoding="utf-8")
    elif arguments:
        print(
            "usage: check_default_dependencies.py [--tree-output FILE]",
            file=sys.stderr,
        )
        return 2

    entry = dependency_entry(tree_output)
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
