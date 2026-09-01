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


def healthy_test_output(repetitions: int) -> str:
    markers = ["Running unittests src/lib.rs"] + [
        f"Running tests/{path.name}" for path in sorted((ROOT / "tests").glob("*.rs"))
    ] + ["Doc-tests tl_syntax"]
    counts = (10, 3, 1, 10, 1)
    assert len(markers) == len(counts)
    return "".join(
        "".join(
            f"{marker}\ntest result: ok. {count} passed; 0 failed; 0 ignored\n"
            for marker, count in zip(markers, counts, strict=True)
        )
        for _ in range(repetitions)
    )


def main() -> int:
    with tempfile.TemporaryDirectory() as directory:
        evidence_dir = Path(directory)
        (evidence_dir / "candidate-gates.status.txt").write_text("0\n", encoding="utf-8")
        (evidence_dir / "candidate-gates.stdout").write_text("passed\n", encoding="utf-8")
        (evidence_dir / "pgm01-schema.status.txt").write_text("125\n", encoding="utf-8")
        (evidence_dir / "pgm01-schema.stdout").write_text("tool absent\n", encoding="utf-8")
        (evidence_dir / "pgm01-validator.status.txt").write_text("3\n", encoding="utf-8")
        outcomes = {item["name"]: item for item in MODULE.command_outcomes(evidence_dir)}
        assert outcomes["candidate-gates"] == {
            "name": "candidate-gates",
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
        assert MODULE.classify_result("final", [outcomes["candidate-gates"]])[0] == "inconclusive"
        assert MODULE.classify_result("provisional", [outcomes["candidate-gates"]])[0] == "inconclusive"
        assert MODULE.classify_result("sealed-failed", [outcomes["candidate-gates"]])[0] == "error"
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
            "candidate-gates", "make-spec", "quire-coverage", "rustdoc",
            "default-dependencies", "diff-integrity", "input-schema",
            "manifest-schema", "pgm01-schema", "pgm01-validator",
            "sealed-pgm01-schema", "sealed-pgm01-validator",
        ):
            (evidence_dir / f"{name}.status.txt").write_text("0\n", encoding="utf-8")
            (evidence_dir / f"{name}.stdout").write_text("verified\n", encoding="utf-8")
            (evidence_dir / f"{name}.stderr").write_text("", encoding="utf-8")
        (evidence_dir / "candidate-gates.stdout").write_text(
            healthy_test_output(2), encoding="utf-8"
        )
        (evidence_dir / "rustdoc.stderr").write_text(
            "Generated /tmp/doc/tl_syntax/index.html\n", encoding="utf-8"
        )
        revision = subprocess.run(
            ["/usr/bin/git", "rev-parse", "HEAD"], cwd=ROOT, check=True,
            capture_output=True, text=True,
        ).stdout.strip()
        source_lock = json.loads(
            subprocess.run(
                ["/usr/bin/git", "show", f"{revision}:tools.lock"], cwd=ROOT,
                check=True, capture_output=True, text=True,
            ).stdout
        )
        (evidence_dir / "collection-input.json").write_text(
            json.dumps(
                {
                    "qualificationProfile": "tl-syntax.evidence-qualification/v2",
                    "tools": {"identities": source_lock["tools"]},
                }
            ),
            encoding="utf-8",
        )
        (evidence_dir / "source-revision.txt").write_text(revision + "\n", encoding="utf-8")
        for name in ("cargo", "rustc"):
            output = subprocess.run(
                [source_lock["tools"][name]["path"], "--version", "--verbose"],
                cwd=ROOT, check=True, capture_output=True,
            ).stdout
            (evidence_dir / f"{name}-version.txt").write_bytes(output)
        finalizer = importlib.util.spec_from_file_location(
            "finalize_collection", ROOT / "scripts" / "finalize_collection.py"
        )
        assert finalizer is not None and finalizer.loader is not None
        finalizer_module = importlib.util.module_from_spec(finalizer)
        finalizer.loader.exec_module(finalizer_module)
        source_builder = finalizer_module.git_bytes(revision, finalizer_module.builder.BUILDER)
        parameters = finalizer_module.historical_parameters_digest(revision, source_builder)
        (evidence_dir / "evidence-envelope.json").write_text(
            json.dumps(
                {
                    "parametersDigest": {"value": parameters},
                    "result": {"status": "conclusive", "summary": "fabricated"},
                }
            )
            + "\n",
            encoding="utf-8",
        )
        healthy_summary = finalizer_module.summary(evidence_dir)
        assert healthy_summary["overallStatus"] == "passed"
        assert not finalizer_module.validate_parameter_identity(evidence_dir)
        assert not finalizer_module.validate_tool_identity(evidence_dir)
        collection_input = json.loads(
            (evidence_dir / "collection-input.json").read_text(encoding="utf-8")
        )
        collection_input["tools"]["identities"]["cargo"]["sha256"] = "0" * 64
        (evidence_dir / "collection-input.json").write_text(
            json.dumps(collection_input) + "\n", encoding="utf-8"
        )
        assert finalizer_module.validate_tool_identity(evidence_dir), (
            "mutated retained tool identity was accepted"
        )
        collection_input["tools"]["identities"] = source_lock["tools"]
        (evidence_dir / "collection-input.json").write_text(
            json.dumps(collection_input) + "\n", encoding="utf-8"
        )
        envelope = json.loads(
            (evidence_dir / "evidence-envelope.json").read_text(encoding="utf-8")
        )
        envelope["parametersDigest"]["value"] = "0" * 64
        (evidence_dir / "evidence-envelope.json").write_text(
            json.dumps(envelope) + "\n", encoding="utf-8"
        )
        assert finalizer_module.validate_parameter_identity(evidence_dir), (
            "mutated parametersDigest was accepted"
        )
        envelope["parametersDigest"]["value"] = parameters
        (evidence_dir / "evidence-envelope.json").write_text(
            json.dumps(envelope) + "\n", encoding="utf-8"
        )
        (evidence_dir / "candidate-gates.stdout").write_text("", encoding="utf-8")
        assert finalizer_module.summary(evidence_dir)["overallStatus"] == "failed", (
            "an empty successful CI transcript was accepted"
        )
        (evidence_dir / "candidate-gates.stdout").write_text(
            healthy_test_output(2), encoding="utf-8"
        )
        (evidence_dir / "collection-input.json").write_text("{}\n", encoding="utf-8")
        try:
            finalizer_module.summary(evidence_dir)
        except ValueError:
            pass
        else:
            raise AssertionError("evidence without a qualification profile stayed active")
        (evidence_dir / "collection-input.json").write_text(
            json.dumps(
                {
                    "qualificationProfile": "tl-syntax.evidence-qualification/v2",
                    "tools": {"identities": source_lock["tools"]},
                }
            ),
            encoding="utf-8",
        )
        envelope = json.loads(
            (evidence_dir / "evidence-envelope.json").read_text(encoding="utf-8")
        )
        expected_status, expected_result_summary = MODULE.classify_result(
            "final", healthy_summary["outcomes"]
        )
        envelope["result"] = {
            "status": expected_status,
            "summary": expected_result_summary,
        }
        (evidence_dir / "evidence-envelope.json").write_text(
            json.dumps(envelope) + "\n", encoding="utf-8"
        )
        missing_summary = subprocess.run(
            [
                sys.executable,
                str(ROOT / "scripts" / "finalize_collection.py"),
                "--check",
                str(evidence_dir),
            ],
            check=False,
            capture_output=True,
            text=True,
        )
        assert missing_summary.returncode == 1 and (
            "active qualification summary is missing" in missing_summary.stderr
        ), "finalizer did not specifically reject an active record with no summary"
        envelope["result"] = {"status": "conclusive", "summary": "fabricated"}
        (evidence_dir / "evidence-envelope.json").write_text(
            json.dumps(envelope) + "\n", encoding="utf-8"
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
