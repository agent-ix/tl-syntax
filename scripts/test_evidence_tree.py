#!/usr/bin/env python3
"""Behavior tests for the repository-wide evidence integrity boundary."""

from __future__ import annotations

import hashlib
import importlib.util
import tempfile
from pathlib import Path


ROOT = Path(__file__).resolve().parent.parent
SPEC = importlib.util.spec_from_file_location(
    "verify_evidence_tree", ROOT / "scripts" / "verify_evidence_tree.py"
)
assert SPEC is not None and SPEC.loader is not None
MODULE = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(MODULE)


def digest(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def main() -> int:
    with tempfile.TemporaryDirectory() as directory:
        root = Path(directory)
        evidence = root / "evidence"
        record = evidence / "record"
        reviews = evidence / "reviews"
        record.mkdir(parents=True)
        reviews.mkdir()
        artifact = record / "artifact.txt"
        artifact.write_text("retained\n", encoding="utf-8")
        record_manifest = evidence / "record.sha256"
        record_manifest.write_text(
            f"{digest(artifact)}  evidence/record/artifact.txt\n", encoding="utf-8"
        )
        review = reviews / "review.md"
        review.write_text("reviewed\n", encoding="utf-8")
        static_manifest = evidence / "STATIC.sha256"
        static_manifest.write_text(
            f"{digest(review)}  evidence/reviews/review.md\n", encoding="utf-8"
        )
        (evidence / "ANCHORS").write_text(
            f"{digest(static_manifest)}  evidence/STATIC.sha256\n"
            f"{digest(record_manifest)}  evidence/record.sha256\n",
            encoding="utf-8",
        )
        assert MODULE.verify_tree(root) == []
        (reviews / "unlisted.md").write_text("outside boundary\n", encoding="utf-8")
        assert MODULE.verify_tree(root), "an unlisted evidence document escaped verification"
    print("repository-wide evidence integrity behavior is valid")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
