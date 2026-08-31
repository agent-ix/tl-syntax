#!/usr/bin/env python3
"""Require tl-syntax's default normal dependency graph to contain only itself."""

from __future__ import annotations

import os
import subprocess
import sys
from pathlib import Path


def main() -> int:
    if len(sys.argv) == 3 and sys.argv[1] == "--tree-output":
        output = Path(sys.argv[2]).read_text(encoding="utf-8")
    elif len(sys.argv) == 1:
        cargo = os.environ.get("CARGO", "cargo")
        result = subprocess.run(
            [cargo, "tree", "--no-default-features", "--edges", "normal", "--prefix", "none"],
            check=False, capture_output=True, text=True,
        )
        if result.returncode != 0:
            sys.stderr.write(result.stderr)
            return result.returncode
        output = result.stdout
    else:
        print("usage: check_default_dependencies.py [--tree-output FILE]", file=sys.stderr)
        return 2
    dependencies = [line for line in output.splitlines() if line.strip()]
    if len(dependencies) != 1 or not dependencies[0].startswith("tl-syntax v"):
        print("default normal dependency graph is not empty:", file=sys.stderr)
        print(output, file=sys.stderr, end="")
        return 1
    print("default normal dependency graph contains no dependencies")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
