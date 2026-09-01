#!/usr/bin/env python3
"""Resolve active and explicitly retracted evidence qualification profiles."""

from __future__ import annotations

import json
import re
import subprocess
import sys
from pathlib import Path


ROOT = Path(__file__).resolve().parent.parent
PROFILE = "tl-syntax.evidence-qualification/v2"
RETRACTIONS = Path("evidence/RETRACTIONS.json")


class EvidenceUnavailable(RuntimeError):
    """The repository lacks Git state needed to evaluate qualification."""


def retracted_records(root: Path = ROOT) -> set[str]:
    value = json.loads((root / RETRACTIONS).read_text(encoding="utf-8"))
    if value.get("schemaVersion") != "tl-syntax.evidence-retractions/v1":
        raise ValueError("evidence retraction registry has an unknown schema")
    records = value.get("records")
    if not isinstance(records, dict):
        raise ValueError("evidence retraction registry has no record map")
    for name, disposition in records.items():
        if not isinstance(name, str) or not isinstance(disposition, dict):
            raise ValueError("evidence retraction registry has a malformed record entry")
        reason = disposition.get("reason")
        if not isinstance(reason, str) or not reason.strip():
            raise ValueError(f"evidence retraction has no reason: {name}")
    return set(records)


def resolve_profile(evidence_dir: Path, root: Path = ROOT) -> str:
    if evidence_dir.name in retracted_records(root):
        return "retracted"
    value = json.loads((evidence_dir / "collection-input.json").read_text(encoding="utf-8"))
    if value.get("qualificationProfile") != PROFILE:
        raise ValueError("active evidence has an absent or unrecognized qualificationProfile")
    revision = (evidence_dir / "source-revision.txt").read_text(encoding="utf-8").strip()
    if re.fullmatch(r"[0-9a-f]{40}", revision) is None:
        raise ValueError("active evidence source revision is malformed")
    if not (root / ".git").exists():
        raise EvidenceUnavailable("repository metadata is missing")
    revision_result = subprocess.run(
        ["/usr/bin/git", "cat-file", "-e", f"{revision}^{{commit}}"],
        cwd=root,
        check=False,
        capture_output=True,
    )
    if revision_result.returncode != 0:
        raise EvidenceUnavailable(
            f"active evidence source revision is unavailable: {revision}"
        )
    result = subprocess.run(
        ["/usr/bin/git", "cat-file", "-e", f"{revision}:tools.lock"], cwd=root,
        check=False, capture_output=True,
    )
    if result.returncode != 0:
        raise ValueError("active evidence source revision has no qualified tool lock")
    return "v2"


def qualification_census(root: Path = ROOT, head: str = "HEAD") -> tuple[int, int]:
    evidence = root / "evidence"
    manifests = sorted(
        path for path in evidence.glob("tl-syntax-v01-*.sha256") if path.is_file()
    )
    record_names = {path.stem for path in manifests}
    retracted = retracted_records(root)
    unknown = retracted - record_names
    if unknown:
        raise ValueError(
            f"retraction registry names unknown retained records: {sorted(unknown)}"
        )

    active = 0
    retracted_count = 0
    active_revision = ""
    for manifest in manifests:
        record = manifest.with_suffix("")
        if not record.is_dir():
            raise ValueError(f"retained evidence record is missing: {record.name}")
        profile = resolve_profile(record, root)
        if profile == "v2":
            active += 1
            active_revision = (
                record / "source-revision.txt"
            ).read_text(encoding="utf-8").strip()
        elif profile == "retracted":
            retracted_count += 1

    if active != 1:
        raise ValueError(
            "evidence qualification requires exactly one active "
            f"tl-syntax.evidence-qualification/v2 record, found {active}"
        )
    if active + retracted_count != len(manifests):
        raise ValueError("evidence qualification census is incomplete")
    current = subprocess.run(
        [
            "/usr/bin/git",
            "diff",
            "--quiet",
            f"{active_revision}..{head}",
            "--",
            ".",
            ":(exclude)evidence",
        ],
        cwd=root,
        check=False,
        capture_output=True,
    )
    if current.returncode == 1:
        raise ValueError(
            "active evidence source revision differs from the current source head"
        )
    if current.returncode != 0:
        raise EvidenceUnavailable("cannot compare active evidence with the current source head")
    return active, retracted_count


def main() -> int:
    if sys.argv != [sys.argv[0], "--verify-census"]:
        print("usage: evidence_profile.py --verify-census", file=sys.stderr)
        return 2
    try:
        active, retracted = qualification_census()
    except EvidenceUnavailable as error:
        print(f"evidence qualification census unavailable: {error}", file=sys.stderr)
        return 2
    except (OSError, ValueError, json.JSONDecodeError) as error:
        print(f"evidence qualification census failed: {error}", file=sys.stderr)
        return 1
    print(
        "evidence qualification census: "
        f"active-v2={active} retracted={retracted} total={active + retracted}"
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
