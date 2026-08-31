#!/usr/bin/env python3
"""Exercise shell and remaining Python gate entry points against corrupt inputs."""

from __future__ import annotations

import subprocess
import sys
import tempfile
import hashlib
import json
import shutil
from pathlib import Path
import importlib.util
from collections.abc import Callable


ROOT = Path(__file__).resolve().parent.parent
SPEC = importlib.util.spec_from_file_location(
    "check_evidence_shell_contract", ROOT / "scripts" / "check_evidence_shell_contract.py"
)
assert SPEC is not None and SPEC.loader is not None
CONTRACT = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(CONTRACT)


def replace_manifest_digest(path: Path, relative: str, digest: str) -> None:
    lines = path.read_text(encoding="utf-8").splitlines()
    suffix = f"  {relative}"
    matches = [index for index, line in enumerate(lines) if line.endswith(suffix)]
    assert len(matches) == 1, f"expected one manifest entry for {relative}"
    lines[matches[0]] = f"{digest}{suffix}"
    path.write_text("\n".join(lines) + "\n", encoding="utf-8")


def active_record(tree: Path) -> Path:
    registry = json.loads(
        (tree / "evidence" / "RETRACTIONS.json").read_text(encoding="utf-8")
    )
    retracted = set(registry["records"])
    records = sorted(
        path
        for path in (tree / "evidence").glob("tl-syntax-v01-*")
        if path.is_dir() and path.name not in retracted
    )
    assert len(records) == 1, f"expected exactly one active evidence record, got {records}"
    return records[0]


def shell_gate_rejects(mutator: Callable[[Path, Path], None], message: str) -> None:
    with tempfile.TemporaryDirectory(prefix="tl-syntax-shell-gate-") as directory:
        tree = Path(directory) / "tree"
        added = subprocess.run(
            ["/usr/bin/git", "worktree", "add", "--detach", str(tree), "HEAD"],
            cwd=ROOT,
            check=False,
            capture_output=True,
            text=True,
        )
        assert added.returncode == 0, f"cannot create shell-gate worktree: {added.stderr}"
        try:
            # Exercise the live working-tree gate, including reviewer mutations, rather
            # than silently falling back to the committed copy in the detached worktree.
            for source in (ROOT / "scripts").iterdir():
                if source.is_file():
                    shutil.copy2(source, tree / "scripts" / source.name)
            record = active_record(tree)
            mutator(tree, record)
            result = subprocess.run(
                ["/usr/bin/bash", "scripts/verify_evidence.sh"],
                cwd=tree,
                check=False,
                stdout=subprocess.DEVNULL,
                stderr=subprocess.DEVNULL,
            )
            assert result.returncode != 0, message
        finally:
            subprocess.run(
                ["/usr/bin/git", "worktree", "remove", "--force", str(tree)],
                cwd=ROOT,
                check=False,
                stdout=subprocess.DEVNULL,
                stderr=subprocess.DEVNULL,
            )


def corrupt_inner_manifest(_tree: Path, record: Path) -> None:
    manifest = record / "evidence-manifest.json"
    manifest.write_bytes(manifest.read_bytes() + b"\n")


def corrupt_parameters_digest(_tree: Path, record: Path) -> None:
    envelope_path = record / "evidence-envelope.json"
    envelope = json.loads(envelope_path.read_text(encoding="utf-8"))
    envelope["parametersDigest"]["value"] = "0" * 64
    envelope_path.write_text(
        json.dumps(envelope, indent=2, sort_keys=True) + "\n", encoding="utf-8"
    )


def retract_only_active_record(tree: Path, record: Path) -> None:
    registry_path = tree / "evidence" / "RETRACTIONS.json"
    registry = json.loads(registry_path.read_text(encoding="utf-8"))
    registry["records"][record.name] = {
        "reason": "policy probe must not permit zero active qualified records"
    }
    registry_path.write_text(
        json.dumps(registry, indent=2, sort_keys=True) + "\n", encoding="utf-8"
    )
    static = tree / "evidence" / "STATIC.sha256"
    replace_manifest_digest(
        static,
        "evidence/RETRACTIONS.json",
        hashlib.sha256(registry_path.read_bytes()).hexdigest(),
    )
    replace_manifest_digest(
        tree / "evidence" / "ANCHORS",
        "evidence/STATIC.sha256",
        hashlib.sha256(static.read_bytes()).hexdigest(),
    )


def main() -> int:
    shell_text = (ROOT / "scripts" / "verify_evidence.sh").read_text(encoding="utf-8")
    for command in CONTRACT.REQUIRED:
        assert CONTRACT.inspect(shell_text.replace(command, "true", 1)), (
            f"evidence shell contract accepted removal of {command}"
        )
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

    shell_gate_rejects(
        corrupt_inner_manifest,
        "evidence shell gate accepted a corrupt per-record evidence manifest",
    )
    shell_gate_rejects(
        corrupt_parameters_digest,
        "evidence shell gate accepted a mutated active-record parametersDigest",
    )
    shell_gate_rejects(
        retract_only_active_record,
        "evidence shell gate accepted a census with zero active qualified records",
    )

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
