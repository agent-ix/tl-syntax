#!/usr/bin/env python3
"""Validate corpus schemas, derive horizons, and check reviewed result oracles."""

from __future__ import annotations

import json
import sys
from pathlib import Path
from typing import Any

from jsonschema import Draft7Validator


ROOT = Path(__file__).resolve().parent.parent
CORPUS = ROOT / "corpus"
EXPECTED_CLOSED_TRACE = {
    "primitive-true-v1": True,
    "nested-not-future-v1": True,
    "boundary-singleton-globally-v1": True,
    "short-trace-future-v1": False,
    "large-bound-future-v1": False,
}
OPERATOR_FIELDS = {
    "false": (),
    "true": (),
    "proposition": (),
    "not": ("operand",),
    "and": ("left", "right"),
    "or": ("left", "right"),
    "implies": ("left", "right"),
    "equivalent": ("left", "right"),
    "future": ("operand",),
    "globally": ("operand",),
    "until": ("left", "right"),
    "release": ("left", "right"),
}
TEMPORAL_KINDS = {"future", "globally", "until", "release"}


def load_json(path: Path) -> Any:
    return json.loads(path.read_text(encoding="utf-8"))


def operand_ids(node: dict[str, Any]) -> list[int]:
    kind = node.get("kind")
    if kind not in OPERATOR_FIELDS:
        raise AssertionError(f"validator does not recognize formula operator {kind!r}")
    return [node.get(field) for field in OPERATOR_FIELDS[kind]]


def formula_horizon(document: dict[str, Any]) -> int:
    """Derive the standard bounded-MLTL worst-case lookahead at the root."""
    values: list[int] = []
    for index, node in enumerate(document["nodes"]):
        kind = node["kind"]
        operands = operand_ids(node)
        try:
            children = [values[operand] for operand in operands]
        except (IndexError, TypeError) as error:
            raise AssertionError(
                f"node {index} has an invalid operand while deriving its horizon"
            ) from error
        if kind in {"false", "true", "proposition"}:
            value = 0
        elif kind == "not":
            value = children[0]
        elif kind in {"and", "or", "implies", "equivalent"}:
            value = max(children)
        elif kind in TEMPORAL_KINDS:
            value = node["interval"]["end"] + max(children)
        else:  # operand_ids rejects this first; retain a defensive diagnostic.
            raise AssertionError(f"cannot derive a horizon for operator {kind!r}")
        if value > (1 << 64) - 1:
            raise AssertionError(f"node {index} horizon exceeds the shared u64 model")
        values.append(value)
    try:
        return values[document["root"]]
    except (IndexError, TypeError) as error:
        raise AssertionError("formula root is invalid while deriving its horizon") from error


def schema_operator_kinds(schema: dict[str, Any]) -> set[str]:
    kinds: set[str] = set()
    for variant in schema["$defs"]["node"]["oneOf"]:
        declaration = variant["properties"]["kind"]
        if "const" in declaration:
            kinds.add(declaration["const"])
        else:
            kinds.update(declaration["enum"])
    return kinds


def formula_document(node: dict[str, Any]) -> dict[str, Any]:
    return {
        "schema_version": "tl-syntax.formula/v1",
        "semantic_profile": "mltl.closed-trace/v1",
        "root": 0,
        "nodes": [node],
    }


def representative_node(kind: str) -> dict[str, Any]:
    node: dict[str, Any] = {"kind": kind}
    if kind == "proposition":
        node["proposition"] = 0
    for field in OPERATOR_FIELDS[kind]:
        node[field] = 0 if field == "operand" else int(field == "right")
    if kind in TEMPORAL_KINDS:
        node["interval"] = {"start": 0, "end": 1}
    return node


def validate_formula_schema_contract(
    schema_value: dict[str, Any], schema: Draft7Validator
) -> None:
    expected_kinds = set(OPERATOR_FIELDS)
    observed_kinds = schema_operator_kinds(schema_value)
    if observed_kinds != expected_kinds:
        raise AssertionError(
            "formula schema operator vocabulary drift: "
            f"expected {sorted(expected_kinds)}, observed {sorted(observed_kinds)}"
        )

    for kind in sorted(expected_kinds):
        errors = list(schema.iter_errors(formula_document(representative_node(kind))))
        if errors:
            raise AssertionError(
                f"formula schema rejects supported operator {kind}: {errors[0].message}"
            )

    invalid_documents = [
        {
            "schema_version": "tl-syntax.formula/v1",
            "semantic_profile": "mltl.closed-trace/v1",
            "root": 0,
            "nodes": [],
        },
        {
            "schema_version": "tl-syntax.formula/v1",
            "semantic_profile": "totally.made.up/v9",
            "root": 0,
            "nodes": [{"kind": "true"}],
        },
        {
            "schema_version": "tl-syntax.formula/v1",
            "semantic_profile": "mltl.closed-trace/v1",
            "root": -1,
            "nodes": [{"kind": "true"}],
        },
        formula_document({"kind": "true", "FABRICATED": "x", "operand": 99}),
        formula_document({"kind": "until"}),
        formula_document(
            {
                "kind": "future",
                "interval": {"start": 0, "end": 1, "FABRICATED": 1},
                "operand": 0,
            }
        ),
        formula_document({"kind": "true", "span": {"start": 0, "end": 1, "JUNK": 1}}),
    ]
    if any(not list(schema.iter_errors(document)) for document in invalid_documents):
        raise AssertionError("formula schema accepted a required constraint probe")

    inverted = formula_document(
        {"kind": "future", "interval": {"start": 3, "end": 2}, "operand": 0}
    )
    if semantic_formula_error(inverted, schema) != "interval_inverted":
        raise AssertionError("semantic corpus gate accepted an inverted interval")


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


def validate() -> None:
    formula_schema_value = load_json(CORPUS / "schema" / "formula-v1.schema.json")
    proposition_schema_value = load_json(
        CORPUS / "schema" / "proposition-map-v1.schema.json"
    )
    Draft7Validator.check_schema(formula_schema_value)
    Draft7Validator.check_schema(proposition_schema_value)
    formula_schema = Draft7Validator(formula_schema_value)
    proposition_schema = Draft7Validator(proposition_schema_value)
    validate_formula_schema_contract(formula_schema_value, formula_schema)

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
            derived_horizon = formula_horizon(document)
            declared_horizon = fixture.get("expected_horizon")
            if declared_horizon != derived_horizon:
                raise AssertionError(
                    f"{identity} horizon drift: derived {derived_horizon}, "
                    f"declared {declared_horizon}"
                )
            expected_closed = EXPECTED_CLOSED_TRACE.get(identity)
            declared_closed = fixture.get("expected_closed_trace")
            if expected_closed is None or declared_closed != expected_closed:
                raise AssertionError(
                    f"{identity} closed-trace oracle drift: reviewed {expected_closed}, "
                    f"declared {declared_closed}"
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
    if observed_valid != set(EXPECTED_CLOSED_TRACE):
        raise AssertionError("valid fixture population differs from the reviewed expectation set")

    print("corpus schemas, derived horizons, rejection reasons, and reviewed oracles are valid")


def main() -> int:
    try:
        validate()
    except (AssertionError, KeyError, TypeError, ValueError, OSError, json.JSONDecodeError) as error:
        print(f"corpus validation failed: {error}", file=sys.stderr)
        return 1
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
