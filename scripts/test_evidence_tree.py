#!/usr/bin/env python3
"""Behavior tests for the repository-wide evidence integrity boundary."""

from __future__ import annotations

import hashlib
import importlib.util
import os
import shutil
import subprocess
import sys
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
        record_name = "tl-syntax-v01-000000000000-20260831T200000Z"
        record = evidence / record_name
        reviews = evidence / "reviews"
        record.mkdir(parents=True)
        reviews.mkdir()
        artifact = record / "artifact.txt"
        artifact.write_text("retained\n", encoding="utf-8")
        source_revision = record / "source-revision.txt"
        source_revision.write_text("0" * 40 + "\n", encoding="utf-8")
        envelope = record / "evidence-envelope.json"
        envelope.write_text('{"recordedAt":"2026-08-31T20:00:01Z"}\n', encoding="utf-8")
        record_manifest = evidence / f"{record_name}.sha256"
        record_manifest.write_text(
            f"{digest(artifact)}  evidence/{record_name}/artifact.txt\n"
            f"{digest(envelope)}  evidence/{record_name}/evidence-envelope.json\n"
            f"{digest(source_revision)}  evidence/{record_name}/source-revision.txt\n",
            encoding="utf-8",
        )
        review = reviews / "review.md"
        review.write_text("reviewed\n", encoding="utf-8")
        static_manifest = evidence / "STATIC.sha256"
        static_manifest.write_text(
            f"{digest(review)}  evidence/reviews/review.md\n", encoding="utf-8"
        )
        (evidence / "ANCHORS").write_text(
            f"{digest(static_manifest)}  evidence/STATIC.sha256\n"
            f"{digest(record_manifest)}  evidence/{record_name}.sha256\n",
            encoding="utf-8",
        )
        subprocess.run(["/usr/bin/git", "init", "-q"], cwd=root, check=True)
        subprocess.run(["/usr/bin/git", "config", "user.name", "Policy Test"], cwd=root, check=True)
        subprocess.run(
            ["/usr/bin/git", "config", "user.email", "policy-test@example.invalid"],
            cwd=root, check=True,
        )
        subprocess.run(
            ["/usr/bin/git", "add", "."], cwd=root, check=True,
            env={**os.environ, "GIT_AUTHOR_DATE": "2026-08-31T20:00:01Z", "GIT_COMMITTER_DATE": "2026-08-31T20:00:01Z"},
        )
        subprocess.run(
            ["/usr/bin/git", "commit", "-qm", "fixture"], cwd=root, check=True,
            env={**os.environ, "GIT_AUTHOR_DATE": "2026-08-31T20:00:01Z", "GIT_COMMITTER_DATE": "2026-08-31T20:00:01Z"},
        )
        assert MODULE.verify_tree(root) == []
        healthy = subprocess.run(
            [sys.executable, str(ROOT / "scripts" / "verify_evidence_tree.py"), "--root", str(root)],
            check=False,
            capture_output=True,
        )
        assert healthy.returncode == 0
        clone_name = "tl-syntax-v01-000000000000-20990101T000000Z"
        shutil.copytree(record, evidence / clone_name)
        shutil.copy2(record_manifest, evidence / f"{clone_name}.sha256")
        assert MODULE.validate_record_identity(
            root, Path("evidence") / f"{clone_name}.sha256"
        ), "a cloned record with a fabricated timestamp kept a valid identity"
        (reviews / "unlisted.md").write_text("outside boundary\n", encoding="utf-8")
        assert MODULE.verify_tree(root), "an unlisted evidence document escaped verification"
        rejected = subprocess.run(
            [sys.executable, str(ROOT / "scripts" / "verify_evidence_tree.py"), "--root", str(root)],
            check=False,
            capture_output=True,
        )
        assert rejected.returncode != 0, "evidence verifier exit contract was gutted"

    with tempfile.TemporaryDirectory() as directory:
        root = Path(directory)
        manifest = Path("evidence/tl-syntax-v01-000000000000-20260831T200000Z.sha256")
        path = root / manifest
        path.parent.mkdir(parents=True)
        path.write_text("introduced\n", encoding="utf-8")
        subprocess.run(["git", "init", "-q"], cwd=root, check=True)
        subprocess.run(["git", "config", "user.name", "Policy Test"], cwd=root, check=True)
        subprocess.run(
            ["git", "config", "user.email", "policy-test@example.invalid"], cwd=root, check=True
        )
        subprocess.run(["git", "add", "."], cwd=root, check=True)
        subprocess.run(
            ["git", "commit", "-qm", "introduce record"], cwd=root, check=True,
            env={
                **os.environ,
                "GIT_AUTHOR_DATE": "2026-08-31T20:00:01Z",
                "GIT_COMMITTER_DATE": "2026-08-31T20:00:01Z",
            },
        )
        assert MODULE.validate_record_history(root, {manifest}) == []
        path.write_text("mutated\n", encoding="utf-8")
        assert MODULE.validate_record_history(root, {manifest}), (
            "a record manifest mutation after introduction escaped verification"
        )
        assert MODULE.validate_record_history(root, set()), (
            "a historically introduced record deletion escaped verification"
        )
    with tempfile.TemporaryDirectory() as directory:
        root = Path(directory)
        assert MODULE.validate_record_history(root, set()), (
            "missing Git metadata silently disabled record-history validation"
        )
    print("repository-wide evidence integrity behavior is valid")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
