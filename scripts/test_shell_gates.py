#!/usr/bin/env python3
"""Exercise shell and remaining Python gate entry points against corrupt inputs."""

from __future__ import annotations

import subprocess
import sys
import tempfile
from pathlib import Path


ROOT = Path(__file__).resolve().parent.parent


def main() -> int:
    planted_evidence = ROOT / "evidence" / ".POLICY-SHELL-PROBE"
    planted_evidence.write_text("fabricated\n", encoding="utf-8")
    try:
        result = subprocess.run(
            ["/usr/bin/bash", "scripts/verify_evidence.sh"], cwd=ROOT,
            check=False, stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL,
        )
        assert result.returncode != 0, "evidence shell gate accepted a planted root artifact"
    finally:
        planted_evidence.unlink(missing_ok=True)

    planted_rust = ROOT / "src" / ".policy_unsafe_probe.rs"
    planted_rust.write_text("fn probe() { unsafe { core::hint::unreachable_unchecked() } }\n", encoding="utf-8")
    try:
        result = subprocess.run(
            ["/usr/bin/bash", "scripts/check_unsafe_comments.sh"], cwd=ROOT,
            check=False, stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL,
        )
        assert result.returncode != 0, "unsafe-comment shell gate accepted an unreviewed block"
    finally:
        planted_rust.unlink(missing_ok=True)

    with tempfile.TemporaryDirectory() as directory:
        tree = Path(directory) / "tree.txt"
        tree.write_text("tl-syntax v0.1.0 (/x)\nforged v1.0.0\n", encoding="utf-8")
        result = subprocess.run(
            [sys.executable, str(ROOT / "scripts" / "check_default_dependencies.py"),
             "--tree-output", str(tree)], check=False, capture_output=True,
        )
        assert result.returncode != 0, "default-dependency gate exit contract accepted a dependency"

    print("shell and Python gate entry-point behavior is valid")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
