#!/usr/bin/env python3
"""Behavior tests for the mandatory-target failure-propagation policy."""

from __future__ import annotations

import importlib.util
import os
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
            "\tcargo clippy --all-targets --all-features -- -D warnings",
            "\t-cargo clippy --all-targets --all-features -- -D warnings",
            1,
        ),
        original.replace(
            "\tcargo test --all-features",
            "\tcargo test --all-features || true",
            1,
        ),
        original.replace(
            "\tcargo test --all-features",
            "\tcargo test --all-features; true",
            1,
        ),
        original + "\n.IGNORE: test\n",
        original + "\nMAKEFLAGS += -i\n",
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
            if index in {3, 4}:
                make_result = subprocess.run(
                    ["make", "--no-print-directory", "-f", str(path), "ci"],
                    cwd=ROOT,
                    check=False,
                    stdout=subprocess.DEVNULL,
                    stderr=subprocess.DEVNULL,
                    env={key: value for key, value in os.environ.items() if key != "MAKEFLAGS"},
                )
                assert make_result.returncode != 0, (
                    f"Make recipe-control mutation {index} converted CI to success"
                )

        ignored_root = Path(directory) / "ignored"
        (ignored_root / "tests").mkdir(parents=True)
        (ignored_root / "tests" / "disabled.rs").write_text(
            "#[test]\n#[cfg_attr(all(), ignore)]\nfn disabled() {}\n", encoding="utf-8"
        )
        assert MODULE.inspect_ignored_tests(ignored_root), "cfg_attr(ignore) escaped inspection"

        hidden = Path(directory) / "hidden.mk"
        hidden.write_text(
            original.replace(
                "cargo check --lib --no-default-features --features alloc",
                "$(ALLOC_CHECK)",
                1,
            )
            + "\nALLOC_CHECK = cargo check --lib --no-default-features --features alloc || true\n",
            encoding="utf-8",
        )
        assert MODULE.inspect_expanded_recipes(hidden, ROOT), (
            "expanded recipe hid a false-success operator"
        )

        synthetic = Path(directory) / "synthetic.mk"
        synthetic.write_text(
            "".join(
                f".PHONY: {target}\n{target}:\n\t-true\n"
                for target in MODULE.PROBES
            ),
            encoding="utf-8",
        )
        assert MODULE.probe_command_positions(synthetic), "command-position probe was gutted"

    assert MODULE.probe_command_positions(ROOT / "Makefile") == []
    for value in ("i", "ik", "-i", "--ignore-errors"):
        assert MODULE.makeflags_ignore_errors(value), f"MAKEFLAGS={value!r} escaped inspection"
    ignored_make = subprocess.run(
        ["make", "--no-print-directory", "-i", "-f", str(ROOT / "Makefile"), "ci"],
        cwd=ROOT,
        check=False,
        stdout=subprocess.DEVNULL,
        stderr=subprocess.DEVNULL,
    )
    assert ignored_make.returncode != 0, "make -i converted local CI to a false success"
    print("failure-propagation policy behavior is valid")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
