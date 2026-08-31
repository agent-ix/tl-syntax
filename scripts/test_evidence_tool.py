#!/usr/bin/env python3
"""Behavior tests for evidence outcome classification."""

from __future__ import annotations

import importlib.util
import hashlib
import json
import subprocess
import sys
import tempfile
from pathlib import Path


ROOT = Path(__file__).resolve().parent.parent
SPEC = importlib.util.spec_from_file_location(
    "build_evidence_envelope", ROOT / "scripts" / "build_evidence_envelope.py"
)
assert SPEC is not None and SPEC.loader is not None
MODULE = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(MODULE)
VERIFY_SPEC = importlib.util.spec_from_file_location(
    "verify_evidence_manifest", ROOT / "scripts" / "verify_evidence_manifest.py"
)
assert VERIFY_SPEC is not None and VERIFY_SPEC.loader is not None
VERIFY_MODULE = importlib.util.module_from_spec(VERIFY_SPEC)
VERIFY_SPEC.loader.exec_module(VERIFY_MODULE)


def main() -> int:
    with tempfile.TemporaryDirectory() as directory:
        evidence_dir = Path(directory)
        (evidence_dir / "make-ci.status.txt").write_text("0\n", encoding="utf-8")
        (evidence_dir / "make-ci.stdout").write_text("passed\n", encoding="utf-8")
        (evidence_dir / "pgm01-schema.status.txt").write_text("125\n", encoding="utf-8")
        (evidence_dir / "pgm01-schema.stdout").write_text("tool absent\n", encoding="utf-8")
        (evidence_dir / "pgm01-validator.status.txt").write_text("3\n", encoding="utf-8")
        outcomes = {item["name"]: item for item in MODULE.command_outcomes(evidence_dir)}
        assert outcomes["make-ci"] == {
            "name": "make-ci",
            "status": "passed",
            "exitCode": 0,
        }
        assert outcomes["pgm01-schema"] == {
            "name": "pgm01-schema",
            "status": "skipped-unavailable",
            "exitCode": 125,
        }
        assert outcomes["pgm01-validator"] == {
            "name": "pgm01-validator",
            "status": "failed",
            "exitCode": 3,
        }
        assert outcomes["make-spec"] == {
            "name": "make-spec",
            "status": "inconclusive",
            "exitCode": None,
        }
        assert MODULE.classify_result("final", [outcomes["make-ci"]])[0] == "inconclusive"
        assert MODULE.classify_result("provisional", [outcomes["make-ci"]])[0] == "inconclusive"
        assert MODULE.classify_result("sealed-failed", [outcomes["make-ci"]])[0] == "error"
        assert MODULE.classify_result("final", [outcomes["pgm01-schema"]])[0] == "inconclusive"
        assert MODULE.classify_result("final", [outcomes["pgm01-validator"]])[0] == "error"

    with tempfile.TemporaryDirectory() as directory:
        root = Path(directory)
        evidence_dir = root / "record"
        evidence_dir.mkdir()
        retained = evidence_dir / "retained.txt"
        retained.write_text("sealed\n", encoding="utf-8")
        checksum = root / "record.sha256"
        checksum.write_text(
            f"{hashlib.sha256(retained.read_bytes()).hexdigest()}  {retained}\n",
            encoding="utf-8",
        )
        assert VERIFY_MODULE.verify(checksum) == []
        (evidence_dir / "unlisted.txt").write_text("not sealed\n", encoding="utf-8")
        assert any(
            "unlisted retained artifact" in error
            for error in VERIFY_MODULE.verify(checksum)
        ), "an added file escaped exact-membership verification"
        rejected = subprocess.run(
            [sys.executable, str(ROOT / "scripts" / "verify_evidence_manifest.py"),
             str(checksum)], check=False, capture_output=True,
        )
        assert rejected.returncode != 0, "manifest verifier exit contract accepted an extra file"

    with tempfile.TemporaryDirectory() as directory:
        evidence_dir = Path(directory)
        for name in (
            "make-ci", "make-spec", "quire-coverage", "rustdoc",
            "default-dependencies", "diff-integrity", "input-schema",
            "manifest-schema", "pgm01-schema", "pgm01-validator",
            "sealed-pgm01-schema", "sealed-pgm01-validator",
        ):
            (evidence_dir / f"{name}.status.txt").write_text("0\n", encoding="utf-8")
            (evidence_dir / f"{name}.stdout").write_text("verified\n", encoding="utf-8")
            (evidence_dir / f"{name}.stderr").write_text("", encoding="utf-8")
        (evidence_dir / "make-ci.stdout").write_text(
            "test result: ok. 1 passed; 0 failed; 0 ignored\n" * 5,
            encoding="utf-8",
        )
        (evidence_dir / "collection-input.json").write_text(
            json.dumps({"qualificationProfile": "tl-syntax.evidence-qualification/v2"}),
            encoding="utf-8",
        )
        (evidence_dir / "source-revision.txt").write_text(
            subprocess.run(
                ["git", "rev-parse", "HEAD"], cwd=ROOT, check=True,
                capture_output=True, text=True,
            ).stdout,
            encoding="utf-8",
        )
        (evidence_dir / "evidence-envelope.json").write_text(
            json.dumps({"result": {"status": "conclusive", "summary": "fabricated"}}) + "\n",
            encoding="utf-8",
        )
        finalizer = importlib.util.spec_from_file_location(
            "finalize_collection", ROOT / "scripts" / "finalize_collection.py"
        )
        assert finalizer is not None and finalizer.loader is not None
        finalizer_module = importlib.util.module_from_spec(finalizer)
        finalizer.loader.exec_module(finalizer_module)
        healthy_summary = finalizer_module.summary(evidence_dir)
        assert healthy_summary["overallStatus"] == "passed"
        (evidence_dir / "make-ci.stdout").write_text("", encoding="utf-8")
        assert finalizer_module.summary(evidence_dir)["overallStatus"] == "failed", (
            "an empty successful CI transcript was accepted"
        )
        (evidence_dir / "make-ci.stdout").write_text(
            "test result: ok. 1 passed; 0 failed; 0 ignored\n" * 5,
            encoding="utf-8",
        )
        (evidence_dir / "collection-summary.json").write_text(
            json.dumps(finalizer_module.summary(evidence_dir), indent=2, sort_keys=True) + "\n",
            encoding="utf-8",
        )
        rejected = subprocess.run(
            [
                sys.executable,
                str(ROOT / "scripts" / "finalize_collection.py"),
                "--check",
                str(evidence_dir),
            ],
            check=False,
            capture_output=True,
        )
        assert rejected.returncode != 0, "finalizer exit contract accepted a fabricated result"
    print("evidence outcome behavior is valid")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
