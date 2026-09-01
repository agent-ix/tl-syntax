#!/usr/bin/env python3
"""Verify the exact executable identities used for local qualification."""

from __future__ import annotations

import hashlib
import json
import shutil
import subprocess
import sys
from pathlib import Path
from typing import Any


ROOT = Path(__file__).resolve().parent.parent
LOCK = ROOT / "tools.lock"
EXPECTED_HOME = "/home/peter"
EXPECTED_TARGET = str(ROOT / "target" / "qualification-v1")
REQUIRED = (
    "bash", "cargo", "git", "make", "python3", "quire", "rustc", "rustup",
    "sha256sum",
)


def sha256(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def validate_lock(value: Any) -> dict[str, dict[str, str]]:
    if not isinstance(value, dict) or value.get("schemaVersion") != "tl-syntax.qualified-tools/v1":
        raise ValueError("tools.lock has an unknown schema")
    tools = value.get("tools")
    if not isinstance(tools, dict) or set(tools) != set(REQUIRED):
        raise ValueError("tools.lock does not contain the exact mandatory-tool census")
    validated: dict[str, dict[str, str]] = {}
    for name in REQUIRED:
        identity = tools.get(name)
        if not isinstance(identity, dict) or set(identity) != {"path", "sha256"}:
            raise ValueError(f"tools.lock has a malformed identity for {name}")
        path = identity.get("path")
        digest = identity.get("sha256")
        if not isinstance(path, str) or not Path(path).is_absolute():
            raise ValueError(f"tools.lock path for {name} is not absolute")
        if not isinstance(digest, str) or len(digest) != 64 or any(
            character not in "0123456789abcdef" for character in digest
        ):
            raise ValueError(f"tools.lock digest for {name} is malformed")
        validated[name] = {"path": path, "sha256": digest}
    environment = value.get("environment")
    if (
        not isinstance(environment, dict)
        or set(environment) != {"home", "cargoTargetDir"}
        or environment.get("home") != EXPECTED_HOME
        or environment.get("cargoTargetDir") != EXPECTED_TARGET
    ):
        raise ValueError("tools.lock has a malformed qualification environment")
    toolchain = value.get("toolchain")
    if (
        not isinstance(toolchain, dict)
        or set(toolchain) != {"cargoVerboseSha256", "rustcVerboseSha256"}
        or any(
            not isinstance(digest, str)
            or len(digest) != 64
            or any(character not in "0123456789abcdef" for character in digest)
            for digest in toolchain.values()
        )
    ):
        raise ValueError("tools.lock has a malformed dispatched-toolchain identity")
    return validated


def load_lock(path: Path = LOCK) -> tuple[dict[str, Any], dict[str, dict[str, str]]]:
    value = json.loads(path.read_text(encoding="utf-8"))
    return value, validate_lock(value)


def trusted_path(tools: dict[str, dict[str, str]]) -> str:
    parents: list[str] = []
    for name in REQUIRED:
        parent = str(Path(tools[name]["path"]).parent)
        if parent not in parents:
            parents.append(parent)
    return ":".join(parents)


def qualified_environment(value: dict[str, Any], tools: dict[str, dict[str, str]]) -> dict[str, str]:
    return {
        "HOME": value["environment"]["home"],
        "PATH": trusted_path(tools),
        "CARGO_TARGET_DIR": value["environment"]["cargoTargetDir"],
        "USER": "qualified",
        "LANG": "C.UTF-8",
        "LC_ALL": "C.UTF-8",
    }


def verify_live(
    value: dict[str, Any],
    tools: dict[str, dict[str, str]],
    search_path: str | None = None,
) -> tuple[list[str], list[str]]:
    unavailable: list[str] = []
    mismatches: list[str] = []
    for name in REQUIRED:
        expected = tools[name]
        observed = shutil.which(name, path=search_path)
        if observed is None:
            unavailable.append(f"qualified tool is unavailable: {name}")
            continue
        if observed != expected["path"]:
            mismatches.append(
                f"qualified tool path mismatch for {name}: expected {expected['path']}, got {observed}"
            )
            continue
        try:
            observed_digest = sha256(Path(observed))
        except OSError as error:
            unavailable.append(f"cannot read qualified tool {name}: {error}")
            continue
        if observed_digest != expected["sha256"]:
            mismatches.append(
                f"qualified tool digest mismatch for {name}: expected {expected['sha256']}, "
                f"got {observed_digest}"
            )
    environment = qualified_environment(value, tools)
    for name in ("cargo", "rustc"):
        expected = value["toolchain"][f"{name}VerboseSha256"]
        result = subprocess.run(
            [tools[name]["path"], "--version", "--verbose"],
            cwd=ROOT,
            env=environment,
            check=False,
            capture_output=True,
        )
        if result.returncode != 0:
            unavailable.append(
                f"qualified {name} --version --verbose failed with {result.returncode}"
            )
        elif hashlib.sha256(result.stdout).hexdigest() != expected:
            mismatches.append(f"qualified dispatched {name} toolchain identity mismatch")
    return unavailable, mismatches


def main() -> int:
    if len(sys.argv) != 2 or sys.argv[1] not in {
        "--verify-live", "--trusted-path", "--home", "--cargo-target-dir"
    }:
        print(
            "usage: tool_identity.py "
            "{--verify-live|--trusted-path|--home|--cargo-target-dir}",
            file=sys.stderr,
        )
        return 2
    try:
        value, tools = load_lock()
    except (OSError, ValueError, json.JSONDecodeError) as error:
        print(f"qualified tool lock is unavailable: {error}", file=sys.stderr)
        return 2
    if sys.argv[1] == "--trusted-path":
        print(trusted_path(tools))
        return 0
    if sys.argv[1] == "--home":
        print(value["environment"]["home"])
        return 0
    if sys.argv[1] == "--cargo-target-dir":
        print(value["environment"]["cargoTargetDir"])
        return 0
    unavailable, mismatches = verify_live(value, tools)
    for error in unavailable + mismatches:
        print(error, file=sys.stderr)
    if unavailable:
        return 2
    return 1 if mismatches else 0


if __name__ == "__main__":
    raise SystemExit(main())
