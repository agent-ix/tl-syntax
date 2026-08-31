#!/usr/bin/env python3
"""Require tl-syntax's default normal dependency graph to contain only itself."""

from __future__ import annotations

import os
import subprocess
import sys


def main() -> int:
    cargo = os.environ.get("CARGO", "cargo")
    result = subprocess.run(
        [cargo, "tree", "--no-default-features", "--edges", "normal", "--prefix", "none"],
        check=False,
        capture_output=True,
        text=True,
    )
    if result.returncode != 0:
        sys.stderr.write(result.stderr)
        return result.returncode
    dependencies = [line for line in result.stdout.splitlines() if line.strip()]
    if len(dependencies) != 1 or not dependencies[0].startswith("tl-syntax v"):
        print("default normal dependency graph is not empty:", file=sys.stderr)
        print(result.stdout, file=sys.stderr, end="")
        return 1
    print("default normal dependency graph contains no dependencies")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
