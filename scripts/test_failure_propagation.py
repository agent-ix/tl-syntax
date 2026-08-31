#!/usr/bin/env python3
"""Behavior tests for the mandatory-target failure-propagation policy."""

from __future__ import annotations

import importlib.util
import subprocess
import sys
import tempfile
from pathlib import Path


ROOT = Path(__file__).resolve().parent.parent
SPEC = importlib.util.spec_from_file_location(
    "check_failure_propagation", ROOT / "scripts" / "check_failure_propagation.py"
)
assert SPEC is not None and SPEC.loader is not None
MODULE = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(MODULE)


def main() -> int:
    original = (ROOT / "Makefile").read_text(encoding="utf-8")
    assert MODULE.inspect_makefile(ROOT / "Makefile") == []

    mutations = [
        original.replace(
            "\t$(CARGO) clippy --all-targets --all-features -- -D warnings",
            "\t-$(CARGO) clippy --all-targets --all-features -- -D warnings",
            1,
        ),
        original.replace(
            "\t$(CARGO) test --all-features",
            "\t$(CARGO) test --all-features || true",
            1,
        ),
        original.replace(
            "ci: check-failure-propagation",
            "ci: fabricated-gate check-failure-propagation",
            1,
        ),
    ]
    with tempfile.TemporaryDirectory() as directory:
        for index, mutated in enumerate(mutations):
            path = Path(directory) / f"Makefile.{index}"
            path.write_text(mutated, encoding="utf-8")
            assert MODULE.inspect_makefile(path), f"mutation {index} escaped inspection"
            result = subprocess.run(
                [
                    sys.executable,
                    str(ROOT / "scripts" / "check_failure_propagation.py"),
                    "--makefile",
                    str(path),
                    "--inspect-only",
                ],
                check=False,
                capture_output=True,
                text=True,
            )
            assert result.returncode != 0, f"mutation {index} produced a false pass"
    print("failure-propagation policy behavior is valid")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
