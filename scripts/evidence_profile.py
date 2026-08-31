#!/usr/bin/env python3
"""Resolve active and explicitly retracted evidence qualification profiles."""

from __future__ import annotations

import json
import subprocess
from pathlib import Path


ROOT = Path(__file__).resolve().parent.parent
PROFILE = "tl-syntax.evidence-qualification/v2"
RETRACTIONS = ROOT / "evidence" / "RETRACTIONS.json"


def retracted_records() -> set[str]:
    value = json.loads(RETRACTIONS.read_text(encoding="utf-8"))
    if value.get("schemaVersion") != "tl-syntax.evidence-retractions/v1":
        raise ValueError("evidence retraction registry has an unknown schema")
    records = value.get("records")
    if not isinstance(records, dict):
        raise ValueError("evidence retraction registry has no record map")
    return set(records)


def resolve_profile(evidence_dir: Path) -> str:
    if evidence_dir.name in retracted_records():
        return "retracted"
    value = json.loads((evidence_dir / "collection-input.json").read_text(encoding="utf-8"))
    if value.get("qualificationProfile") != PROFILE:
        raise ValueError("active evidence has an absent or unrecognized qualificationProfile")
    revision = (evidence_dir / "source-revision.txt").read_text(encoding="utf-8").strip()
    result = subprocess.run(
        ["/usr/bin/git", "cat-file", "-e", f"{revision}:tools.lock"], cwd=ROOT,
        check=False, capture_output=True,
    )
    if result.returncode != 0:
        raise ValueError("active evidence source revision has no qualified tool lock")
    return "v2"
