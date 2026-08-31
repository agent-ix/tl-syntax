#!/usr/bin/env python3
"""Validate corpus schemas and independently derive horizons and result oracles."""

from __future__ import annotations

import json
import sys
from pathlib import Path
from typing import Any

try:
    from jsonschema import Draft7Validator
except ImportError as error:
    print(f"corpus validation unavailable: {error}", file=sys.stderr)
    raise SystemExit(125) from error


ROOT = Path(__file__).resolve().parent.parent
CORPUS = ROOT / "corpus"
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


def evaluate_closed_trace(document: dict[str, Any], trace: list[list[int]]) -> bool:
    """Evaluate a valid topological formula under complete finite-trace semantics."""
    nodes = document["nodes"]
    memo: dict[tuple[int, int], bool] = {}

    def distinct_instants(start: int, end: int) -> range | tuple[int, ...]:
        """Represent the finite prefix plus one equivalent out-of-trace instant."""
        if start > end:
            return ()
        prefix_end = min(end, len(trace) - 1)
        prefix = tuple(range(start, prefix_end + 1)) if start <= prefix_end else ()
        outside = max(start, len(trace))
        return prefix + ((outside,) if outside <= end else ())

    def evaluate(node_id: int, instant: int) -> bool:
        key = (node_id, min(instant, len(trace)))
        if key in memo:
            return memo[key]
        node = nodes[node_id]
        kind = node["kind"]
        if kind == "false":
            value = False
        elif kind == "true":
            value = True
        elif kind == "proposition":
            value = instant < len(trace) and node["proposition"] in trace[instant]
        elif kind == "not":
            value = not evaluate(node["operand"], instant)
        elif kind == "and":
            value = evaluate(node["left"], instant) and evaluate(node["right"], instant)
        elif kind == "or":
            value = evaluate(node["left"], instant) or evaluate(node["right"], instant)
        elif kind == "implies":
            value = not evaluate(node["left"], instant) or evaluate(node["right"], instant)
        elif kind == "equivalent":
            value = evaluate(node["left"], instant) == evaluate(node["right"], instant)
        else:
            interval = node["interval"]
            start = instant + interval["start"]
            end = instant + interval["end"]
            candidates = distinct_instants(start, end)
            if kind == "future":
                value = any(evaluate(node["operand"], item) for item in candidates)
            elif kind == "globally":
                value = all(evaluate(node["operand"], item) for item in candidates)
            elif kind == "until":
                value = any(
                    evaluate(node["right"], witness)
                    and all(
                        evaluate(node["left"], item)
                        for item in distinct_instants(instant, witness - 1)
                    )
                    for witness in candidates
                )
            elif kind == "release":
                value = all(
                    evaluate(node["right"], witness)
                    or any(
                        evaluate(node["left"], item)
                        for item in distinct_instants(instant, witness - 1)
                    )
                    for witness in candidates
                )
            else:
                raise AssertionError(f"cannot evaluate operator {kind!r}")
        memo[key] = value
        return value

    return evaluate(document["root"], 0)


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
    expected_semantics = [
        "interval start must not exceed end",
        "span start must not exceed end",
        "root must index an existing node",
        "every operand must reference a preceding node",
    ]
    if schema_value.get("x-tl-syntax-semantic-validator") != "python3 scripts/validate_corpus.py":
        raise AssertionError("formula schema does not declare its mandatory semantic validator")
    if schema_value.get("x-tl-syntax-semantic-constraints") != expected_semantics:
        raise AssertionError("formula schema semantic constraint declaration drifted")
    if schema_value["properties"]["nodes"].get("maxItems") != 100_000:
        raise AssertionError("formula schema wire node limit drifted")
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
            expected_closed = evaluate_closed_trace(document, fixture["trace"])
            declared_closed = fixture.get("expected_closed_trace")
            if declared_closed != expected_closed:
                raise AssertionError(
                    f"{identity} closed-trace oracle drift: derived {expected_closed}, "
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
    print("corpus schemas, derived horizons, rejection reasons, and derived oracles are valid")


def main() -> int:
    try:
        validate()
    except (AssertionError, OSError, json.JSONDecodeError) as error:
        print(f"corpus validation failed: {error}", file=sys.stderr)
        return 1
    except (KeyError, TypeError, ValueError) as error:
        print(f"corpus validator error: {type(error).__name__}: {error}", file=sys.stderr)
        return 2
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
