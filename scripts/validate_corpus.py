#!/usr/bin/env python3
"""Validate corpus schemas, exact expected outcomes, and semantic rejection reasons."""

from __future__ import annotations

import json
from pathlib import Path
from typing import Any

from jsonschema import Draft7Validator


ROOT = Path(__file__).resolve().parent.parent
CORPUS = ROOT / "corpus"
EXPECTED_VALID = {
    "primitive-true-v1": (0, True),
    "nested-not-future-v1": (1, True),
    "boundary-singleton-globally-v1": (0, True),
    "short-trace-future-v1": (2, False),
    "large-bound-future-v1": (4294967295, False),
}


def load_json(path: Path) -> Any:
    return json.loads(path.read_text(encoding="utf-8"))


def operand_ids(node: dict[str, Any]) -> list[int]:
    kind = node.get("kind")
    if kind in {"not", "future", "globally"}:
        return [node.get("operand")]
    if kind in {"and", "or", "implies", "equivalent", "until", "release"}:
        return [node.get("left"), node.get("right")]
    return []


def semantic_formula_error(document: Any, schema: Draft7Validator) -> str | None:
    if not isinstance(document, dict):
        return "schema_error"
    if document.get("semantic_profile") not in {
        "mltl.closed-trace/v1",
        "mltl.online-prefix/v1",
    }:
        return "unsupported_semantic_profile"
    schema_errors = list(schema.iter_errors(document))
    if schema_errors:
        return "schema_error"
    nodes = document["nodes"]
    for node in nodes:
        interval = node.get("interval")
        if interval is not None and interval["start"] > interval["end"]:
            return "interval_inverted"
        span = node.get("span")
        if span is not None and span["start"] > span["end"]:
            return "span_inverted"
    if document["root"] >= len(nodes):
        return "root_out_of_range"
    for index, node in enumerate(nodes):
        if any(not isinstance(operand, int) or operand >= index for operand in operand_ids(node)):
            return "operand_not_preceding"
    return None


def validate_proposition_map(value: Any, schema: Draft7Validator) -> None:
    errors = list(schema.iter_errors(value))
    if errors:
        raise AssertionError(f"proposition map violates schema: {errors[0].message}")
    entries = value["propositions"]
    ids = [entry["id"] for entry in entries]
    names = [entry["name"] for entry in entries]
    if ids != sorted(set(ids)):
        raise AssertionError("proposition identities are not strictly increasing")
    if len(names) != len(set(names)):
        raise AssertionError("proposition names are not unique")


def main() -> int:
    formula_schema_value = load_json(CORPUS / "schema" / "formula-v1.schema.json")
    proposition_schema_value = load_json(
        CORPUS / "schema" / "proposition-map-v1.schema.json"
    )
    Draft7Validator.check_schema(formula_schema_value)
    Draft7Validator.check_schema(proposition_schema_value)
    formula_schema = Draft7Validator(formula_schema_value)
    proposition_schema = Draft7Validator(proposition_schema_value)

    validate_proposition_map(load_json(CORPUS / "propositions.json"), proposition_schema)
    manifest = load_json(CORPUS / "manifest.json")
    observed_valid: set[str] = set()
    for fixture in manifest["fixtures"]:
        identity = fixture["id"]
        document = load_json(CORPUS / fixture["formula"])
        observed_error = semantic_formula_error(document, formula_schema)
        if fixture["expected_validation"] == "valid":
            if observed_error is not None:
                raise AssertionError(f"{identity} unexpectedly failed: {observed_error}")
            expected = EXPECTED_VALID.get(identity)
            actual = (fixture.get("expected_horizon"), fixture.get("expected_closed_trace"))
            if expected != actual:
                raise AssertionError(
                    f"{identity} expected-result drift: expected {expected}, observed {actual}"
                )
            observed_valid.add(identity)
        elif fixture["expected_validation"] == "invalid":
            expected_error = fixture.get("expected_error")
            if observed_error != expected_error:
                raise AssertionError(
                    f"{identity} rejection drift: expected {expected_error}, observed {observed_error}"
                )
        else:
            raise AssertionError(f"{identity} has unknown expected_validation")
    if observed_valid != set(EXPECTED_VALID):
        raise AssertionError("valid fixture population differs from the reviewed expectation set")

    probes = [
        {
            "schema_version": "tl-syntax.formula/v1",
            "semantic_profile": "mltl.closed-trace/v1",
            "root": 0,
            "nodes": [{"kind": "true", "FABRICATED": "x", "operand": 99}],
        },
        {
            "schema_version": "tl-syntax.formula/v1",
            "semantic_profile": "mltl.closed-trace/v1",
            "root": 0,
            "nodes": [{"kind": "until"}],
        },
    ]
    if any(not list(formula_schema.iter_errors(probe)) for probe in probes):
        raise AssertionError("formula node schema accepted an unknown-field or arity probe")
    print("corpus schemas, rejection reasons, and reviewed expectations are valid")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
