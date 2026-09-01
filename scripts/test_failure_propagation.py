#!/usr/bin/env python3
"""Behavior tests for the mandatory-target failure-propagation policy."""

from __future__ import annotations

import importlib.util
import json
import os
import shutil
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


def stub_unrelated_recipes(path: Path) -> None:
    """Keep the real CI graph while isolating the census wiring under test."""
    lines = path.read_text(encoding="utf-8").splitlines()
    current_target: str | None = None
    rewritten: list[str] = []
    for line in lines:
        if not line.startswith((" ", "\t")):
            match = MODULE.TARGET.match(line)
            current_target = match.group(1) if match is not None else None
        if line.startswith("\t") and current_target not in {
            None, MODULE.GUARD_TARGET,
        }:
            rewritten.append("\t@true")
        else:
            rewritten.append(line)
    path.write_text("\n".join(rewritten) + "\n", encoding="utf-8")


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
        original.replace(
            "ci: check-failure-propagation",
            "ci: fabricated-gate check-failure-propagation",
            1,
        ),
    ]
    for operator in ("=", ":=", "::=", ":::=", "+=", "?=", "!="):
        mutations.append(original + f"\nexport override MAKEFLAGS {operator} -i\n")
    for assignment in (
        "MAKEFLAGS = -i",
        "MAKEFLAGS := -i",
        "export MAKEFLAGS += -i",
        "override MAKEFLAGS ?= -i",
    ):
        mutations.append(original + f"\n{assignment}\n")
    for assignment in (
        "SHELL := /usr/bin/true",
        ".SHELLFLAGS := -c true",
        "MAKE := /usr/bin/true",
        "define MAKEFLAGS\n-i\nendef",
        "override define SHELL\n/usr/bin/true\nendef",
        "$(eval MAKEFLAGS := -i)",
        "${eval SHELL := /usr/bin/true}",
        "include hidden-execution-controls.mk",
    ):
        mutations.append(original + f"\n{assignment}\n")
    for directive in (".SILENT:", ".ONESHELL:", ".DEFAULT:"):
        mutations.append(original + f"\n{directive}\n")
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
            if index >= 3:
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
    for value in ("i", "ik", "-i", "--ignore-errors", "-t", "-n", "--eval=.IGNORE:"):
        assert MODULE.makeflags_ignore_errors(value), f"MAKEFLAGS={value!r} escaped inspection"
    for value in ("-j4", "--jobs=4 --jobserver-auth=3,4", "-l2 -Otarget", "-w"):
        assert not MODULE.makeflags_ignore_errors(value), f"safe MAKEFLAGS={value!r} was rejected"
    allowed_environment = MODULE.portable_census_environment()
    assert set(allowed_environment) <= {
        "HOME", "PATH", "USER", "LANG", "LC_ALL", "TMPDIR",
    }, "portable Rust census inherited an unreviewed ambient variable"
    assert "PATH" in allowed_environment, "portable Rust census has no executable path"
    observed_census: dict[str, object] = {}
    original_inspect_live = MODULE.rust_test_census.inspect_live
    try:
        def census_spy(root: Path, cargo: str, environment: dict[str, str]) -> list[str]:
            observed_census.update(
                {"root": root, "cargo": cargo, "environment": environment}
            )
            return []

        MODULE.rust_test_census.inspect_live = census_spy
        assert MODULE.inspect_test_census() == []
    finally:
        MODULE.rust_test_census.inspect_live = original_inspect_live
    assert observed_census.get("environment") == allowed_environment, (
        "compiled Rust census did not receive the portable environment allowlist"
    )
    ignored_make = subprocess.run(
        ["make", "--no-print-directory", "-i", "-f", str(ROOT / "Makefile"), "ci"],
        cwd=ROOT,
        check=False,
        stdout=subprocess.DEVNULL,
        stderr=subprocess.DEVNULL,
    )
    assert ignored_make.returncode != 0, "make -i converted local CI to a false success"
    unqualified_target_env = dict(os.environ)
    unqualified_target_env.pop("MAKEFLAGS", None)
    unqualified_target_env["CARGO_TARGET_DIR"] = "/tmp/tl-syntax-unqualified-target"
    unqualified_target = subprocess.run(
        ["/usr/bin/make", "--no-print-directory", "ci-for-evidence"],
        cwd=ROOT,
        check=False,
        stdout=subprocess.DEVNULL,
        stderr=subprocess.DEVNULL,
        env=unqualified_target_env,
    )
    assert unqualified_target.returncode != 0, (
        "candidate CI accepted an unqualified ambient Cargo target"
    )
    with tempfile.TemporaryDirectory() as directory:
        fake_home = Path(directory)
        shim = fake_home / ".cargo" / "bin" / "cargo"
        shim.parent.mkdir(parents=True)
        shim.write_text("#!/bin/sh\nexit 0\n", encoding="utf-8")
        shim.chmod(0o755)
        shadowed_env = dict(os.environ)
        shadowed_env.pop("MAKEFLAGS", None)
        shadowed_env["HOME"] = directory
        shadowed_env["PATH"] = f"{shim.parent}:{shadowed_env['PATH']}"
        shadowed = subprocess.run(
            ["/usr/bin/make", "--no-print-directory", "ci"], cwd=ROOT,
            check=False, stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL,
            env=shadowed_env,
        )
        assert shadowed.returncode != 0, "HOME/PATH-shadowed Cargo bypassed local CI"
    with tempfile.TemporaryDirectory(prefix="tl-syntax-census-") as directory:
        tree = Path(directory) / "tree"
        added = subprocess.run(
            ["/usr/bin/git", "worktree", "add", "--detach", str(tree), "HEAD"],
            cwd=ROOT,
            check=False,
            capture_output=True,
            text=True,
        )
        assert added.returncode == 0, f"cannot create census worktree: {added.stderr}"
        try:
            shutil.copy2(ROOT / "Makefile", tree / "Makefile")
            for source in (ROOT / "scripts").iterdir():
                if source.is_file():
                    shutil.copy2(source, tree / "scripts" / source.name)
            stub_unrelated_recipes(tree / "Makefile")
            probe_environment = dict(os.environ)
            for name in (
                "MAKEFLAGS", "PYTHONOPTIMIZE", "RUSTUP_TOOLCHAIN", "RUSTUP_HOME",
                "CARGO_HOME", "CARGO_TARGET_DIR", "RUSTC", "RUSTDOC",
                "RUSTC_WRAPPER", "RUSTC_WORKSPACE_WRAPPER", "RUSTFLAGS",
                "CARGO_ENCODED_RUSTFLAGS", "RUSTDOCFLAGS", "LD_PRELOAD",
                "LD_LIBRARY_PATH", "PYTHONPATH",
            ):
                probe_environment.pop(name, None)
            baseline = subprocess.run(
                ["/usr/bin/make", "--no-print-directory", "ci"],
                cwd=tree,
                check=False,
                capture_output=True,
                text=True,
                env=probe_environment,
            )
            assert baseline.returncode == 0, (
                "isolated end-to-end census control is not baseline-green:\n"
                f"stdout:\n{baseline.stdout}\nstderr:\n{baseline.stderr}"
            )
            probe = tree / "src" / ".policy_orphan_probe.rs"
            probe.write_text(
                "// Trace: TC-020, FR-004-AC-4\n#[test]\n"
                "fn orphaned_requirement_test() {}\n",
                encoding="utf-8",
            )
            result = subprocess.run(
                ["/usr/bin/make", "--no-print-directory", "ci"],
                cwd=tree,
                check=False,
                stdout=subprocess.DEVNULL,
                stderr=subprocess.DEVNULL,
                env=probe_environment,
            )
            assert result.returncode != 0, (
                "an uncompiled traced test escaped the end-to-end local CI census"
            )
            probe.unlink()
            integration = tree / "tests" / "integration.rs"
            integration_text = integration.read_text(encoding="utf-8")
            assert integration_text.startswith('#![cfg(feature = "serde")]')
            integration.write_text(
                integration_text.replace(
                    '#![cfg(feature = "serde")]', "#![cfg(any())]", 1
                ),
                encoding="utf-8",
            )
            excluded_binary = subprocess.run(
                ["/usr/bin/make", "--no-print-directory", "ci"],
                cwd=tree,
                check=False,
                stdout=subprocess.DEVNULL,
                stderr=subprocess.DEVNULL,
                env=probe_environment,
            )
            assert excluded_binary.returncode != 0, (
                "an empty cfg-excluded integration binary escaped the compiled-test census"
            )
        finally:
            subprocess.run(
                ["/usr/bin/git", "worktree", "remove", "--force", str(tree)],
                cwd=ROOT,
                check=False,
                stdout=subprocess.DEVNULL,
                stderr=subprocess.DEVNULL,
            )

    lock_value, locked_tools = MODULE.tool_identity.load_lock()
    qualified = MODULE.tool_identity.qualified_environment(lock_value, locked_tools)
    assert set(qualified) == {
        "HOME", "PATH", "CARGO_TARGET_DIR", "USER", "LANG", "LC_ALL"
    }, "qualified environment is not a closed allowlist"
    changed = dict(lock_value)
    changed["toolchain"] = dict(lock_value["toolchain"])
    changed["toolchain"]["rustcVerboseSha256"] = "0" * 64
    trusted_path = MODULE.tool_identity.trusted_path(locked_tools)
    _, mismatches = MODULE.tool_identity.verify_live(
        changed, locked_tools, search_path=trusted_path
    )
    assert any("rustc toolchain" in error for error in mismatches), (
        "dispatched rustc toolchain digest mutation was accepted"
    )
    with tempfile.TemporaryDirectory(prefix="tl-syntax-tool-cli-") as directory:
        fixture_root = Path(directory)
        fixture_scripts = fixture_root / "scripts"
        fixture_scripts.mkdir()
        shutil.copy2(ROOT / "scripts" / "tool_identity.py", fixture_scripts)
        fixture_value = json.loads(json.dumps(changed))
        fixture_value["environment"]["cargoTargetDir"] = str(
            fixture_root / "target" / "qualification-v1"
        )
        (fixture_root / "tools.lock").write_text(
            json.dumps(fixture_value, indent=2, sort_keys=True) + "\n",
            encoding="utf-8",
        )
        cli_mismatch = subprocess.run(
            [sys.executable, str(fixture_scripts / "tool_identity.py"), "--verify-live"],
            check=False,
            capture_output=True,
            text=True,
            env={**os.environ, "PATH": trusted_path},
        )
        assert cli_mismatch.returncode == 1 and (
            "qualified dispatched rustc toolchain identity mismatch"
            in cli_mismatch.stderr
        ), "tool-identity CLI accepted a dispatched rustc digest mutation"
    print("failure-propagation policy behavior is valid")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
