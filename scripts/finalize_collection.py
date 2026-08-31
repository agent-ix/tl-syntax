#!/usr/bin/env python3
"""Write the post-envelope validation summary for a retained collection."""

from __future__ import annotations

import hashlib
import json
import sys
from pathlib import Path


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
        outcomes.append(
            {
                "name": name,
                "status": (
                    "skipped-unavailable"
                    if skipped
                    else "passed" if exit_code == 0 else "failed"
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
    summary_path.write_text(
        json.dumps(value, indent=2, sort_keys=True) + "\n", encoding="utf-8"
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
