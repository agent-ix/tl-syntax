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
TEST_SUCCESS = re.compile(
    r"^test result: ok\. ([1-9][0-9]*) passed; 0 failed; 0 ignored", re.MULTILINE
)
SILENT_SUCCESS = {"diff-integrity"}


def sha256(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def qualification_profile(evidence_dir: Path) -> str | None:
    try:
        value = json.loads((evidence_dir / "collection-input.json").read_text(encoding="utf-8"))
    except (OSError, json.JSONDecodeError):
        return "invalid"
    profile = value.get("qualificationProfile")
    return profile if isinstance(profile, str) else None


def positive_output(evidence_dir: Path, name: str) -> bool:
    stdout = evidence_dir / f"{name}.stdout"
    stderr = evidence_dir / f"{name}.stderr"
    if name == "make-ci":
        text = stdout.read_text(encoding="utf-8", errors="replace") if stdout.exists() else ""
        return len(TEST_SUCCESS.findall(text)) >= 5
    if name in SILENT_SUCCESS:
        return True
    return any(path.exists() and path.stat().st_size > 0 for path in (stdout, stderr))


def summary(evidence_dir: Path) -> dict[str, object]:
    require_positive = qualification_profile(evidence_dir) is not None
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
        positive_missing = exit_code == 0 and require_positive and not positive_output(evidence_dir, name)
        outcomes.append(
            {
                "name": name,
                "status": (
                    "skipped-unavailable"
                    if skipped
                    else "failed"
                    if validator_contradiction or output_contradiction or positive_missing
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


def historical_parameters_digest(revision: str, source_builder: bytes) -> str:
    tree = set(
        subprocess.run(
            ["git", "ls-tree", "-r", "--name-only", revision], cwd=builder.ROOT,
            check=True, capture_output=True, text=True,
        ).stdout.splitlines()
    )
    function = source_builder.split(b"def hash_parameter_files", 1)[1].split(b"\ndef ", 1)[0]
    ordered = [
        (b'ROOT / "Cargo.toml"', "Cargo.toml"),
        (b'ROOT / "Cargo.lock"', "Cargo.lock"),
        (b'ROOT / "Makefile"', "Makefile"),
        (b'ROOT / "rust-toolchain.toml"', "rust-toolchain.toml"),
        (b'ROOT / "corpus" / "SHA256SUMS"', "corpus/SHA256SUMS"),
        (b"COLLECTOR", "scripts/collect_evidence.sh"),
        (b"BUILDER", "scripts/build_evidence_envelope.py"),
        (b"SCHEMA_VALIDATOR", "scripts/validate_json_schema.py"),
        (b"COLLECTION_FINALIZER", "scripts/finalize_collection.py"),
        (b"CORPUS_VALIDATOR", "scripts/validate_corpus.py"),
        (b"TRACEABILITY_VALIDATOR", "scripts/check_traceability_coverage.py"),
        (b"EVIDENCE_VERIFIER", "scripts/verify_evidence_manifest.py"),
        (b"EVIDENCE_ANCHORS", "evidence/ANCHORS"),
        (b"INPUT_SCHEMA", "schemas/tl-syntax-evidence-input-v1.schema.json"),
        (b"MANIFEST_SCHEMA", "schemas/tl-syntax-evidence-manifest-v1.schema.json"),
    ]
    paths = [path for marker, path in ordered if marker in function]
    if b'(ROOT / "scripts").iterdir()' in function:
        paths = sorted(set(paths) | {
            path for path in tree
            if path.startswith("scripts/") and Path(path).suffix in {".py", ".sh"}
        })
    if b"def parameter_paths" in source_builder:
        paths = sorted({
            "Cargo.toml", "Cargo.lock", "Makefile", "rust-toolchain.toml",
            "corpus/SHA256SUMS", "scripts/collect_evidence.sh",
            "scripts/build_evidence_envelope.py", "scripts/validate_json_schema.py",
            "scripts/finalize_collection.py", "scripts/validate_corpus.py",
            "scripts/check_traceability_coverage.py", "scripts/verify_evidence_manifest.py",
            "schemas/tl-syntax-evidence-input-v1.schema.json",
            "schemas/tl-syntax-evidence-manifest-v1.schema.json",
        } | {
            path for path in tree
            if path.startswith("scripts/") and Path(path).suffix in {".py", ".sh"}
        })
    missing = set(paths) - tree
    if missing:
        raise OSError(f"source revision lacks parameter paths: {sorted(missing)}")
    state = hashlib.sha256()
    for relative in paths:
        state.update(relative.encode("utf-8"))
        state.update(b"\0")
        state.update(git_bytes(revision, builder.ROOT / relative))
        state.update(b"\0")
    return state.hexdigest()


def validate_parameter_identity(evidence_dir: Path) -> list[str]:
    try:
        envelope = json.loads((evidence_dir / "evidence-envelope.json").read_text(encoding="utf-8"))
        revision = (evidence_dir / "source-revision.txt").read_text(encoding="utf-8").strip()
        source_builder = git_bytes(revision, builder.BUILDER)
        expected_digest = historical_parameters_digest(revision, source_builder)
    except (KeyError, OSError, json.JSONDecodeError, subprocess.CalledProcessError) as error:
        return [f"cannot rederive retained parameter identity: {error}"]
    if envelope.get("parametersDigest", {}).get("value") != expected_digest:
        return [f"envelope parameters digest disagrees with source revision: {evidence_dir}"]
    return []


def validate_envelope(evidence_dir: Path, value: dict[str, object]) -> list[str]:
    try:
        envelope = json.loads((evidence_dir / "evidence-envelope.json").read_text(encoding="utf-8"))
        revision = (evidence_dir / "source-revision.txt").read_text(encoding="utf-8").strip()
        outcomes = value["outcomes"]
        if not isinstance(outcomes, list):
            raise TypeError("collection summary outcomes are not a list")
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
    errors.extend(validate_parameter_identity(evidence_dir))
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
        parameter_errors = validate_parameter_identity(evidence_dir)
        if parameter_errors:
            for error in parameter_errors:
                print(error, file=sys.stderr)
            return 1
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
