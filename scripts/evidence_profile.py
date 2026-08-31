#!/usr/bin/env python3
"""Resolve active and explicitly retracted evidence qualification profiles."""

from __future__ import annotations

import json
import subprocess
import sys
from pathlib import Path


ROOT = Path(__file__).resolve().parent.parent
PROFILE = "tl-syntax.evidence-qualification/v2"
RETRACTIONS = Path("evidence/RETRACTIONS.json")


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
    result = subprocess.run(
        ["/usr/bin/git", "cat-file", "-e", f"{revision}:tools.lock"], cwd=root,
        check=False, capture_output=True,
    )
    if result.returncode != 0:
        raise ValueError("active evidence source revision has no qualified tool lock")
    return "v2"


def qualification_census(root: Path = ROOT) -> tuple[int, int]:
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
    for manifest in manifests:
        record = manifest.with_suffix("")
        if not record.is_dir():
            raise ValueError(f"retained evidence record is missing: {record.name}")
        profile = resolve_profile(record, root)
        if profile == "v2":
            active += 1
        elif profile == "retracted":
            retracted_count += 1
        else:
            raise ValueError(f"unrecognized evidence disposition: {record.name}: {profile}")

    if active == 0:
        raise ValueError(
            "no active tl-syntax.evidence-qualification/v2 record remains"
        )
    if active + retracted_count != len(manifests):
        raise ValueError("evidence qualification census is incomplete")
    return active, retracted_count


def main() -> int:
    if sys.argv != [sys.argv[0], "--verify-census"]:
        print("usage: evidence_profile.py --verify-census", file=sys.stderr)
        return 2
    try:
        active, retracted = qualification_census()
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
