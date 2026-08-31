#!/usr/bin/env python3
"""Behavior tests for the strict traceability coverage policy gate."""

from __future__ import annotations

import importlib.util
import tempfile
from pathlib import Path


ROOT = Path(__file__).resolve().parent.parent
SPEC = importlib.util.spec_from_file_location(
    "check_traceability_coverage", ROOT / "scripts" / "check_traceability_coverage.py"
)
assert SPEC is not None and SPEC.loader is not None
MODULE = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(MODULE)


def complete_report() -> dict[str, object]:
    return {
        "totals": {"backed": 2, "total": 2},
        "groups": [{"document": "spec/test-matrix.md", "backed": 2, "total": 2}],
        "binding_census": [
            {"language": "rust", "candidates": 2, "tagged": 2, "bound": 2}
        ],
        "unbacked_rows": [],
        "status_lies": [],
        "untracked_symbols": [],
        "minted_targets": [
            {"id": "TC-001", "target": "test-case"},
            {"id": "TC-002", "target": "test-case"},
        ],
        "obligations": [
            {
                "source": "acceptance-criterion",
                "id": "FR-001-AC-1",
                "method": "Test",
                "target_ids": ["TC-001"],
            }
        ],
        "diagnostics": [],
    }


def main() -> int:
    report = complete_report()
    assert MODULE.validate_report(report) == []

    report = complete_report()
    report["totals"] = {"backed": 1, "total": 2}
    assert MODULE.validate_report(report), "an unbacked row was accepted"

    report = complete_report()
    report["groups"][0]["backed"] = 1
    assert MODULE.validate_report(report), "an incomplete matrix group was accepted"

    report = complete_report()
    report["binding_census"][0]["tagged"] = 1
    assert MODULE.validate_report(report), "an untagged evidence symbol was accepted"

    report = complete_report()
    report["status_lies"] = [{"id": "TC-fabricated"}]
    assert MODULE.validate_report(report), "a contradicted coverage status was accepted"

    report = complete_report()
    report["obligations"][0]["target_ids"] = ["TC-999"]
    assert MODULE.validate_report(report), "a fabricated verification target was accepted"

    report = complete_report()
    report["diagnostics"] = [{"reason": "uncatalogued-verification-method"}]
    assert MODULE.validate_report(report), "a skipped/undefined verification method was accepted"

    with tempfile.TemporaryDirectory() as directory:
        matrix = Path(directory) / "matrix.md"
        matrix.write_text(
            "## Functional Requirement Coverage\n\n"
            "| Functional Req | Status |\n|---|---|\n| FR-001 | 🚧 fabricated |\n",
            encoding="utf-8",
        )
        assert MODULE.validate_matrix_statuses(matrix), "a fabricated matrix status was accepted"
        status, report = MODULE.load_report(Path(directory) / "missing.json")
        assert status == 125 and report is None, "missing tooling/input was not unavailable"

        root = Path(directory) / "references"
        (root / "spec" / "requirements").mkdir(parents=True)
        (root / "spec" / "evidence").mkdir(parents=True)
        (root / "spec" / "test-matrix.md").write_text(
            "| Test Case | Status |\n|---|---|\n| TC-001 | implemented |\n",
            encoding="utf-8",
        )
        (root / "spec" / "evidence" / "suites.md").write_text(
            "| ID | Name |\n|---|---|\n| SUITE-001 | suite |\n", encoding="utf-8"
        )
        requirement = root / "spec" / "requirements" / "StR-001.md"
        requirement.write_text(
            "| ID | Criteria | Verification |\n|---|---|---|\n"
            "| StR-001-VC-1 | criterion | Test |\n",
            encoding="utf-8",
        )
        assert MODULE.validate_verification_references(root), (
            "a stakeholder Test criterion with no target was accepted"
        )
        requirement.write_text(
            "| ID | Criteria | Verification |\n|---|---|---|\n"
            "| StR-001-VC-1 | criterion | Test (TC-999) |\n",
            encoding="utf-8",
        )
        assert MODULE.validate_verification_references(root), (
            "a fabricated stakeholder target was accepted"
        )
        for verification in ("", "Demonstration"):
            requirement.write_text(
                "| ID | Criteria | Verification |\n|---|---|---|\n"
                f"| StR-001-VC-1 | criterion | {verification} |\n",
                encoding="utf-8",
            )
            assert MODULE.validate_verification_references(root), (
                f"uncatalogued stakeholder method {verification!r} was accepted"
            )
        requirement.write_text(
            "| ID | Criteria | Verification |\n|---|---|---|\n"
            "| StR-001-VC-1 | criterion | Inspection |\n",
            encoding="utf-8",
        )
        assert MODULE.validate_verification_references(root) == []
    print("strict traceability coverage behavior is valid")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
