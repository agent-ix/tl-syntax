#!/usr/bin/env python3
"""Write the post-envelope validation summary for a retained collection."""

from __future__ import annotations

import hashlib
import json
import re
import subprocess
import sys
from pathlib import Path

import build_evidence_envelope as builder


CHECKS = (
    "make-ci",
    "make-spec",
    "quire-coverage",
    "rustdoc",
    "default-dependencies",
    "diff-integrity",
    "input-schema",
    "manifest-schema",
    "pgm01-schema",
    "pgm01-validator",
    "sealed-pgm01-schema",
    "sealed-pgm01-validator",
)
CONTRADICTION = re.compile(
    r"test result: FAILED|Error [0-9]+ \(ignored\)|\b[1-9][0-9]* ignored\b"
)


def sha256(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def summary(evidence_dir: Path) -> dict[str, object]:
    outcomes = []
    observed = {
        path.name[: -len(".status.txt")]
        for path in evidence_dir.glob("*.status.txt")
        if path.is_file()
    }
    for name in list(CHECKS) + sorted(observed - set(CHECKS)):
        status_path = evidence_dir / f"{name}.status.txt"
        if not status_path.exists():
            outcomes.append({"name": name, "status": "inconclusive", "exitCode": None})
            continue
        exit_code = int(status_path.read_text(encoding="utf-8").strip())
        skipped = exit_code == 125
        stderr_path = evidence_dir / f"{name}.stderr"
        validator_contradiction = (
            exit_code == 0
            and name in {"pgm01-validator", "sealed-pgm01-validator"}
            and stderr_path.exists()
            and bool(stderr_path.read_text(encoding="utf-8").strip())
        )
        output_contradiction = any(
            path.exists()
            and CONTRADICTION.search(path.read_text(encoding="utf-8", errors="replace"))
            for path in (evidence_dir / f"{name}.stdout", stderr_path)
        )
        outcomes.append(
            {
                "name": name,
                "status": (
                    "skipped-unavailable"
                    if skipped
                    else "failed"
                    if validator_contradiction or output_contradiction
                    else "passed"
                    if exit_code == 0
                    else "failed"
                ),
                "exitCode": exit_code,
            }
        )
    statuses = {item["status"] for item in outcomes}
    if "failed" in statuses:
        overall = "failed"
    elif "skipped-unavailable" in statuses or "inconclusive" in statuses:
        overall = "inconclusive"
    else:
        overall = "passed"
    envelope = evidence_dir / "evidence-envelope.json"
    post_seal_artifacts = []
    for path in sorted(evidence_dir.glob("sealed-*")):
        if path.is_file() and not path.is_symlink():
            post_seal_artifacts.append(
                {"path": path.name, "sha256": sha256(path), "size": path.stat().st_size}
            )
    return {
        "schemaVersion": "tl-syntax.collection-summary/v1",
        "overallStatus": overall,
        "finalEnvelopeSha256": sha256(envelope),
        "finalEnvelopeValidated": all(
            item["status"] == "passed"
            for item in outcomes
            if item["name"].startswith("sealed-")
        ),
        "postSealArtifacts": post_seal_artifacts,
        "integrityBoundary": (
            "the sibling .sha256 manifest exactly enumerates every retained file, "
            "including this summary"
        ),
        "outcomes": outcomes,
    }


def git_bytes(revision: str, path: Path) -> bytes:
    return subprocess.run(
        ["git", "show", f"{revision}:{path.relative_to(builder.ROOT)}"],
        cwd=builder.ROOT,
        check=True,
        capture_output=True,
    ).stdout


def validate_envelope(evidence_dir: Path, value: dict[str, object]) -> list[str]:
    try:
        envelope = json.loads((evidence_dir / "evidence-envelope.json").read_text(encoding="utf-8"))
        revision = (evidence_dir / "source-revision.txt").read_text(encoding="utf-8").strip()
        outcomes = value["outcomes"]
        assert isinstance(outcomes, list)
        sealed_not_passed = any(
            item["name"].startswith("sealed-") and item["status"] != "passed"
            for item in outcomes
        )
        phase = "sealed-failed" if value["overallStatus"] == "failed" or sealed_not_passed else "final"
        expected_status, expected_summary = builder.classify_result(phase, outcomes)
        result = envelope["result"]
    except (AssertionError, KeyError, OSError, json.JSONDecodeError, TypeError) as error:
        return [f"cannot rederive retained envelope result: {error}"]
    errors: list[str] = []
    if result.get("status") != expected_status or result.get("summary") != expected_summary:
        errors.append(f"envelope result disagrees with retained outcomes: {evidence_dir}")
    try:
        source_builder = git_bytes(revision, builder.BUILDER)
        if b"def parameter_paths" in source_builder:
            expected_digest = builder.hash_parameter_files(
                lambda path: git_bytes(revision, path)
            )
            if envelope.get("parametersDigest", {}).get("value") != expected_digest:
                errors.append(f"envelope parameters digest disagrees with source revision: {evidence_dir}")
    except (OSError, subprocess.CalledProcessError) as error:
        errors.append(f"cannot rederive retained parameter identity: {error}")
    return errors


def main() -> int:
    check = len(sys.argv) == 3 and sys.argv[1] == "--check"
    if len(sys.argv) != 2 and not check:
        print("usage: finalize_collection.py [--check] EVIDENCE_DIR", file=sys.stderr)
        return 2
    evidence_dir = Path(sys.argv[2] if check else sys.argv[1])
    value = summary(evidence_dir)
    summary_path = evidence_dir / "collection-summary.json"
    if check:
        if not summary_path.exists():
            input_path = evidence_dir / "collection-input.json"
            try:
                collection_input = json.loads(input_path.read_text(encoding="utf-8"))
            except (OSError, json.JSONDecodeError) as error:
                print(f"cannot verify legacy collection input {input_path}: {error}", file=sys.stderr)
                return 1
            commands = collection_input.get("commands", [])
            if any("finalize_collection.py" in command for command in commands):
                print(f"promised retained summary is missing: {evidence_dir}", file=sys.stderr)
                return 1
            return 0
        envelope_errors = validate_envelope(evidence_dir, value)
        if envelope_errors:
            for error in envelope_errors:
                print(error, file=sys.stderr)
            return 1
        try:
            actual = json.loads(summary_path.read_text(encoding="utf-8"))
        except (OSError, json.JSONDecodeError) as error:
            print(f"cannot read retained summary {summary_path}: {error}", file=sys.stderr)
            return 1
        expected_projection = {key: value.get(key) for key in actual}
        if actual != expected_projection:
            print(f"retained summary disagrees with status files: {evidence_dir}", file=sys.stderr)
            return 1
        return 0
    envelope_errors = validate_envelope(evidence_dir, value)
    if envelope_errors:
        for error in envelope_errors:
            print(error, file=sys.stderr)
        return 1
    summary_path.write_text(
        json.dumps(value, indent=2, sort_keys=True) + "\n", encoding="utf-8"
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
