#!/usr/bin/env python3
"""Require complete Quire traceability coverage from its stable JSON report."""

from __future__ import annotations

import json
import re
import subprocess
import sys
from pathlib import Path
from typing import Any


ROOT = Path(__file__).resolve().parent.parent
TARGET_ID = re.compile(r"^TC-[0-9]{3}$")
TEST_METHOD = re.compile(r"^Test \(TC-[0-9]{3}(?:, TC-[0-9]{3})*\)$")
INSPECTION_METHOD = re.compile(r"^Inspection \((evidence/reviews/[^)]+\.md)\)$")
REFERENCE = re.compile(r"\b(?:TC|SUITE)-[0-9]{3}\b")
CRITERION_SOURCES = {
    "acceptance-criterion",
    "nfr-acceptance-criterion",
    "stakeholder-validation-criterion",
}
ALLOWED_DIAGNOSTICS = {
    "archetype-matches-nothing",
    "catch-all-universal",
    # The installed module requires a structurally invalid alternate header.
    # validate_matrix_statuses supplies the skipped classification independently.
    "status-column-matches-nothing",
}
MAX_INSPECTION_DISTANCE = 5
INSPECTED_PATHS = ("Cargo.toml", "Cargo.lock", "src", "corpus")


def validate_report(report: dict[str, Any]) -> list[str]:
    """Return every condition that prevents the report from being complete."""
    errors: list[str] = []
    totals = report.get("totals")
    if not isinstance(totals, dict):
        return ["coverage report has no totals object"]
    if totals.get("backed") != totals.get("total"):
        errors.append(
            f"coverage total is incomplete: {totals.get('backed')}/{totals.get('total')} backed"
        )

    groups = report.get("groups")
    if not isinstance(groups, list) or not groups:
        errors.append("coverage report has no document groups")
    else:
        for group in groups:
            if not isinstance(group, dict):
                errors.append("coverage report contains a malformed group")
                continue
            if group.get("backed") != group.get("total"):
                errors.append(
                    f"{group.get('document', '<unknown>')} is incomplete: "
                    f"{group.get('backed')}/{group.get('total')} backed"
                )

    census = report.get("binding_census")
    if not isinstance(census, list) or not census:
        errors.append("coverage report has no binding census")
    else:
        for item in census:
            if not isinstance(item, dict):
                errors.append("coverage report contains a malformed binding census entry")
                continue
            candidates = item.get("candidates")
            if item.get("tagged") != candidates or item.get("bound") != candidates:
                errors.append(
                    f"{item.get('language', '<unknown>')} binding census is incomplete: "
                    f"{item.get('bound')}/{item.get('tagged')}/{candidates} "
                    "bound/tagged/candidates"
                )

    for field in ("unbacked_rows", "status_lies", "untracked_symbols"):
        findings = report.get(field)
        if not isinstance(findings, list):
            errors.append(f"coverage report has no {field} list")
        elif findings:
            errors.append(f"coverage report contains {len(findings)} {field}")

    minted = report.get("minted_targets")
    if not isinstance(minted, list):
        errors.append("coverage report has no minted_targets list")
        test_targets: set[str] = set()
    else:
        test_targets = {
            item.get("id")
            for item in minted
            if isinstance(item, dict)
            and item.get("target") == "test-case"
            and isinstance(item.get("id"), str)
        }
    obligations = report.get("obligations")
    if not isinstance(obligations, list):
        errors.append("coverage report has no obligations list")
    else:
        for obligation in obligations:
            if not isinstance(obligation, dict) or obligation.get("source") not in CRITERION_SOURCES:
                continue
            identity = obligation.get("id", "<unknown>")
            targets = obligation.get("target_ids")
            if obligation.get("method") != "Test" or not isinstance(targets, list) or not targets:
                errors.append(f"{identity} does not declare one or more Test targets")
                continue
            for target in targets:
                if not isinstance(target, str) or TARGET_ID.fullmatch(target) is None:
                    errors.append(f"{identity} declares malformed verification target {target!r}")
                elif target not in test_targets:
                    errors.append(f"{identity} declares nonexistent verification target {target}")

    diagnostics = report.get("diagnostics")
    if not isinstance(diagnostics, list):
        errors.append("coverage report has no diagnostics list")
    else:
        for diagnostic in diagnostics:
            reason = diagnostic.get("reason") if isinstance(diagnostic, dict) else None
            if reason not in ALLOWED_DIAGNOSTICS:
                errors.append(f"coverage report contains blocking diagnostic {reason!r}")
    return errors


def validate_matrix_statuses(path: Path) -> list[str]:
    """Enforce status values that the installed module cannot classify safely."""
    errors: list[str] = []
    section = ""
    header: list[str] | None = None
    for number, line in enumerate(path.read_text(encoding="utf-8").splitlines(), start=1):
        if line.startswith("## "):
            section = line[3:].strip()
            header = None
            continue
        if not line.startswith("|"):
            continue
        cells = [cell.strip() for cell in line.strip().strip("|").split("|")]
        if header is None:
            header = cells
            continue
        if all(set(cell) <= {"-", ":"} for cell in cells):
            continue
        status_name = "Status" if "Status" in header else "Coverage Status"
        if status_name not in header or len(cells) != len(header):
            errors.append(f"{path}:{number} has no parseable status column")
            continue
        for index, cell in enumerate(cells):
            if index != header.index(status_name) and not cell:
                errors.append(f"{path}:{number} has an empty {header[index]!r} cell")
        status = cells[header.index(status_name)]
        expected = "✅ implemented" if section == "Test Case Summary" else "✅ covered"
        if status != expected:
            errors.append(
                f"{path}:{number} status {status!r} does not equal required {expected!r}"
            )
    return errors


def matrix_sections(path: Path) -> dict[str, list[list[str]]]:
    sections: dict[str, list[list[str]]] = {}
    section = ""
    for line in path.read_text(encoding="utf-8").splitlines():
        if line.startswith("## "):
            section = line[3:].strip()
            sections.setdefault(section, [])
        elif line.startswith("|") and section:
            cells = [cell.strip() for cell in line.strip().strip("|").split("|")]
            if not all(set(cell) <= {"-", ":"} for cell in cells):
                sections[section].append(cells)
    return sections


def validate_matrix_mappings(path: Path, requirements: Path) -> list[str]:
    sections = matrix_sections(path)
    errors: list[str] = []
    summary = sections.get("Test Case Summary", [])
    if len(summary) < 2:
        return ["test matrix has no Test Case Summary rows"]
    test_rows = {row[0]: row for row in summary[1:] if row}
    declared_tests = set(test_rows)
    requirement_ids = {
        prefix: {
            re.match(r"^((?:FR|NFR|StR)-[0-9]+)", item.name).group(1)
            for item in requirements.glob(f"{prefix}-*.md")
        }
        for prefix in ("FR", "NFR", "StR")
    }
    configurations = {
        "Functional Requirement Coverage": ("FR", 1, 2),
        "Non-Functional Requirement Coverage": ("NFR", None, 2),
        "Stakeholder Requirement Coverage": ("StR", None, 2),
    }
    for section, (prefix, criteria_index, tests_index) in configurations.items():
        rows = sections.get(section, [])
        if len(rows) < 2:
            errors.append(f"test matrix has no rows for {section}")
            continue
        authored = {row[0] for row in rows[1:] if row}
        if authored != requirement_ids[prefix]:
            errors.append(
                f"{section} requirement census drift: "
                f"missing={sorted(requirement_ids[prefix] - authored)}, "
                f"extra={sorted(authored - requirement_ids[prefix])}"
            )
        for row in rows[1:]:
            if len(row) <= tests_index:
                continue
            identity = row[0]
            listed_tests = set(re.findall(r"\bTC-[0-9]{3}\b", row[tests_index]))
            unknown = listed_tests - declared_tests
            if unknown:
                errors.append(f"{identity} coverage names nonexistent tests: {sorted(unknown)}")
            if prefix in {"FR", "NFR"}:
                expected_tests = {
                    test_id for test_id, test_row in test_rows.items()
                    if len(test_row) > 4 and re.search(rf"\b{re.escape(identity)}-AC-[0-9]+\b", test_row[4])
                }
                if listed_tests != expected_tests:
                    errors.append(
                        f"{identity} coverage test mapping drift: "
                        f"expected={sorted(expected_tests)}, observed={sorted(listed_tests)}"
                    )
            if criteria_index is not None:
                requirement_file = next(requirements.glob(f"{identity}-*.md"), None)
                expected_criteria = set()
                if requirement_file is not None:
                    expected_criteria = set(
                        re.findall(
                            rf"\b{re.escape(identity)}-AC-[0-9]+\b",
                            requirement_file.read_text(encoding="utf-8"),
                        )
                    )
                listed_criteria = set(re.findall(rf"\b{re.escape(identity)}-AC-[0-9]+\b", row[criteria_index]))
                if listed_criteria != expected_criteria:
                    errors.append(
                        f"{identity} acceptance-criteria mapping drift: "
                        f"expected={sorted(expected_criteria)}, observed={sorted(listed_criteria)}"
                    )
    return errors


def validate_verification_references(root: Path = ROOT) -> list[str]:
    """Validate AC/VC verification cells independently of Quire obligations."""
    matrix = (root / "spec" / "test-matrix.md").read_text(encoding="utf-8")
    suites = (root / "spec" / "evidence" / "suites.md").read_text(encoding="utf-8")
    declared = set(REFERENCE.findall(matrix)) | set(REFERENCE.findall(suites))
    errors: list[str] = []
    for path in sorted((root / "spec" / "requirements").glob("*.md")):
        for number, line in enumerate(path.read_text(encoding="utf-8").splitlines(), start=1):
            if not line.startswith("|") or not re.search(r"-(?:AC|VC)-[0-9]+\s*\|", line):
                continue
            cells = [cell.strip() for cell in line.strip().strip("|").split("|")]
            criterion_match = re.search(r"((?:FR|NFR|StR)-[0-9]+-(?:AC|VC)-[0-9]+)", line)
            criterion = criterion_match.group(1) if criterion_match is not None else "<unknown>"
            verification = cells[-1] if cells else ""
            targets = REFERENCE.findall(verification)
            inspection = INSPECTION_METHOD.fullmatch(verification)
            if inspection is not None:
                review = root / inspection.group(1)
                try:
                    review_text = review.read_text(encoding="utf-8")
                except OSError as error:
                    errors.append(f"{path}:{number} names an unreadable inspection: {error}")
                    continue
                if re.search(r"^Source subject: `[0-9a-f]{40}`$", review_text, re.MULTILINE) is None:
                    errors.append(f"{path}:{number} names an inspection without an exact source subject")
                    continue
                if re.search(rf"^\|\s*{re.escape(criterion)}\s*\|", review_text, re.MULTILINE) is None:
                    errors.append(
                        f"{path}:{number} names an inspection that does not cover {criterion}"
                    )
                    continue
                subject = re.search(
                    r"^Source subject: `([0-9a-f]{40})`$", review_text, re.MULTILINE
                ).group(1)
                ancestor = subprocess.run(
                    ["/usr/bin/git", "merge-base", "--is-ancestor", subject, "HEAD"],
                    cwd=root,
                    check=False,
                    capture_output=True,
                )
                distance = None
                if ancestor.returncode == 0:
                    distance = int(subprocess.run(
                        ["/usr/bin/git", "rev-list", "--count", f"{subject}..HEAD"],
                        cwd=root, check=True, capture_output=True, text=True,
                    ).stdout.strip())
                content_changed = subprocess.run(
                    ["/usr/bin/git", "diff", "--quiet", subject, "HEAD", "--", *INSPECTED_PATHS],
                    cwd=root, check=False,
                ).returncode != 0
                if (ancestor.returncode != 0 or distance > MAX_INSPECTION_DISTANCE) and content_changed:
                    errors.append(
                        f"{path}:{number} names a stale or content-divergent inspection subject"
                    )
            elif TEST_METHOD.fullmatch(verification) is None:
                errors.append(f"{path}:{number} declares an empty or uncatalogued verification method")
            for target in targets:
                if target not in declared:
                    errors.append(
                        f"{path}:{number} references nonexistent verification target {target}"
                    )
    return errors


def load_report(path: Path | None) -> tuple[int, dict[str, Any] | None]:
    if path is not None:
        try:
            return 0, json.loads(path.read_text(encoding="utf-8"))
        except FileNotFoundError as error:
            print(f"coverage report unavailable: {error}", file=sys.stderr)
            return 125, None
        except (OSError, json.JSONDecodeError) as error:
            print(f"cannot read coverage report {path}: {error}", file=sys.stderr)
            return 2, None

    try:
        result = subprocess.run(
            ["quire", "coverage", "--scope", ".", "--strict", "--json"],
            check=False,
            capture_output=True,
            text=True,
        )
    except FileNotFoundError as error:
        print(f"traceability coverage unavailable: {error}", file=sys.stderr)
        return 125, None
    sys.stderr.write(result.stderr)
    if result.returncode != 0:
        return result.returncode, None
    try:
        return 0, json.loads(result.stdout)
    except json.JSONDecodeError as error:
        print(f"quire emitted invalid coverage JSON: {error}", file=sys.stderr)
        return 2, None


def main() -> int:
    if len(sys.argv) == 1:
        report_path = None
    elif len(sys.argv) == 3 and sys.argv[1] == "--report":
        report_path = Path(sys.argv[2])
    else:
        print(
            "usage: check_traceability_coverage.py [--report REPORT.json]",
            file=sys.stderr,
        )
        return 2

    status, report = load_report(report_path)
    if status != 0 or report is None:
        return status
    errors = validate_report(report)
    errors.extend(validate_matrix_statuses(ROOT / "spec" / "test-matrix.md"))
    errors.extend(
        validate_matrix_mappings(
            ROOT / "spec" / "test-matrix.md", ROOT / "spec" / "requirements"
        )
    )
    errors.extend(validate_verification_references())
    for error in errors:
        print(error, file=sys.stderr)
    if errors:
        return 1
    totals = report["totals"]
    print(f"strict traceability coverage is complete: {totals['backed']}/{totals['total']}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
