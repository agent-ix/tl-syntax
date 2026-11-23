#!/usr/bin/env python3
"""Independent generator for tl-syntax.numeric-corpus/v1 (FR-305).

This script shares no code with the Rust crate. Expected expression types are
derived by its own interval arithmetic; expected predicate truth is computed
with exact rational arithmetic (fractions.Fraction) over declared valuations.
Run with `python3 -I corpus/numeric/provenance/generate.py` from the
repository root; it rewrites corpus/numeric/ deterministically. The corpus is
immutable by version identity: any change to a case needs numeric-corpus/v2.
"""

import itertools
import json
import os
import sys
from fractions import Fraction

ROOT = os.path.join(os.path.dirname(os.path.abspath(__file__)), "..")
I64_MIN = -(2**63)
I64_MAX = 2**63 - 1


def canonical(value):
    return json.dumps(value, separators=(",", ":"), ensure_ascii=False)


def write(relative, text):
    path = os.path.join(ROOT, relative)
    os.makedirs(os.path.dirname(path), exist_ok=True)
    with open(path, "w", encoding="utf-8", newline="") as handle:
        handle.write(text)


# ---------------------------------------------------------------- catalogs --

CATALOG = {
    "schema_version": "tl-syntax.signal-catalog/v1",
    "signals": [
        {"id": 1, "name": "flag", "domain": {"kind": "boolean"}},
        {"id": 2, "name": "count", "domain": {"kind": "integer", "minimum": -5, "maximum": 7}},
        {"id": 3, "name": "temp", "domain": {"kind": "fixed_decimal", "minimum_coefficient": -400, "maximum_coefficient": 1250, "scale": 1}},
        {"id": 4, "name": "gain", "domain": {"kind": "fixed_decimal", "minimum_coefficient": -3, "maximum_coefficient": 2, "scale": 2}},
        {"id": 5, "name": "big", "domain": {"kind": "fixed_decimal", "minimum_coefficient": 1, "maximum_coefficient": 3, "scale": 9}},
        {"id": 6, "name": "drop", "domain": {"kind": "integer", "minimum": -9, "maximum": -2}},
    ],
    "bindings": [{"proposition": 1, "signal": 1}],
}

DOMAINS = {}
for signal in CATALOG["signals"]:
    domain = signal["domain"]
    if domain["kind"] == "integer":
        DOMAINS[signal["id"]] = (0, domain["minimum"], domain["maximum"])
    elif domain["kind"] == "fixed_decimal":
        DOMAINS[signal["id"]] = (domain["scale"], domain["minimum_coefficient"], domain["maximum_coefficient"])


# ------------------------------------------------------------------ typing --

class Refused(Exception):
    def __init__(self, cls):
        super().__init__(cls)
        self.cls = cls


def fits(value):
    return I64_MIN <= value <= I64_MAX


def derive(expressions):
    types = []
    for node in expressions:
        kind = node["kind"]
        if kind == "signal":
            if node["signal"] not in DOMAINS:
                raise Refused("UnknownSignal" if node["signal"] not in [s["id"] for s in CATALOG["signals"]] else "NonNumericSignal")
            t = DOMAINS[node["signal"]]
        elif kind == "integer":
            t = (0, node["value"], node["value"])
        elif kind == "decimal":
            if node["scale"] > 18:
                raise Refused("ScaleOverflow")
            t = (node["scale"], node["coefficient"], node["coefficient"])
        elif kind == "slot_signal":
            raise Refused("SlotOutsideTemplate")
        else:
            operands = [node[k] for k in ("operand", "left", "right") if k in node]
            if any(o >= len(types) for o in operands):
                raise Refused("OperandNotPreceding")
            if kind == "neg":
                s, lo, hi = types[node["operand"]]
                t = (s, -hi, -lo)
            elif kind == "abs":
                s, lo, hi = types[node["operand"]]
                upper = max(abs(lo), abs(hi))
                lower = 0 if lo <= 0 <= hi else min(abs(lo), abs(hi))
                t = (s, lower, upper)
            elif kind in ("add", "sub"):
                (s1, l1, h1), (s2, l2, h2) = types[node["left"]], types[node["right"]]
                if s1 != s2:
                    raise Refused("ScaleMismatch")
                t = (s1, l1 + l2, h1 + h2) if kind == "add" else (s1, l1 - h2, h1 - l2)
            elif kind == "mul":
                (s1, l1, h1), (s2, l2, h2) = types[node["left"]], types[node["right"]]
                if s1 + s2 > 18:
                    raise Refused("ScaleOverflow")
                corners = [l1 * l2, l1 * h2, h1 * l2, h1 * h2]
                t = (s1 + s2, min(corners), max(corners))
            elif kind == "rescale":
                s, lo, hi = types[node["operand"]]
                if node["scale"] > 18:
                    raise Refused("ScaleOverflow")
                if node["scale"] < s:
                    raise Refused("RescaleNarrowing")
                factor = 10 ** (node["scale"] - s)
                t = (node["scale"], lo * factor, hi * factor)
            else:
                raise AssertionError(kind)
        if not (fits(t[1]) and fits(t[2])):
            raise Refused("RangeOverflow")
        types.append(t)
    return types


def check_table(table):
    types = derive(table["expressions"])
    depth = []
    for node in table["expressions"]:
        operands = [node[k] for k in ("operand", "left", "right") if k in node]
        depth.append(1 + max([depth[o] for o in operands], default=0))
        if depth[-1] > 256:
            raise Refused("ExpressionLimitExceeded")
    previous = None
    for predicate in table["predicates"]:
        if previous is not None and previous >= predicate["proposition"]:
            raise Refused("PropositionNotIncreasing")
        previous = predicate["proposition"]
        for root in (predicate["left"], predicate["right"]):
            if root >= len(types):
                raise Refused("RootOutOfRange")
        if types[predicate["left"]][0] != types[predicate["right"]][0]:
            raise Refused("PredicateScaleMismatch")
    return types


# --------------------------------------------------------------- truth ------

def evaluate(expressions, index, valuation):
    node = expressions[index]
    kind = node["kind"]
    if kind == "signal":
        scale = DOMAINS[node["signal"]][0]
        return Fraction(valuation[node["signal"]], 10**scale)
    if kind == "integer":
        return Fraction(node["value"])
    if kind == "decimal":
        return Fraction(node["coefficient"], 10 ** node["scale"])
    if kind == "neg":
        return -evaluate(expressions, node["operand"], valuation)
    if kind == "abs":
        return abs(evaluate(expressions, node["operand"], valuation))
    if kind == "rescale":
        return evaluate(expressions, node["operand"], valuation)
    left = evaluate(expressions, node["left"], valuation)
    right = evaluate(expressions, node["right"], valuation)
    return {"add": left + right, "sub": left - right, "mul": left * right}[kind]


COMPARE = {
    "lt": lambda a, b: a < b,
    "le": lambda a, b: a <= b,
    "gt": lambda a, b: a > b,
    "ge": lambda a, b: a >= b,
    "eq": lambda a, b: a == b,
    "ne": lambda a, b: a != b,
}


def signals_read(expressions):
    return sorted({node["signal"] for node in expressions if node["kind"] == "signal"})


def sample_values(signal):
    _, lo, hi = DOMAINS[signal]
    values = {lo, hi}
    if lo <= 0 <= hi:
        values.add(0)
    return sorted(values)


def truth_table(table):
    signals = signals_read(table["expressions"])
    rows = []
    for combination in itertools.product(*[sample_values(s) for s in signals]):
        valuation = dict(zip(signals, combination))
        truth = []
        for predicate in table["predicates"]:
            left = evaluate(table["expressions"], predicate["left"], valuation)
            right = evaluate(table["expressions"], predicate["right"], valuation)
            truth.append({"proposition": predicate["proposition"], "value": COMPARE[predicate["operator"]](left, right)})
        rows.append({"valuation": [{"signal": s, "coefficient": valuation[s]} for s in signals], "truth": truth})
    return rows


# ------------------------------------------------------------------ cases ---

def sig(s):
    return {"kind": "signal", "signal": s}


def integer(v):
    return {"kind": "integer", "value": v}


def decimal(c, s):
    return {"kind": "decimal", "coefficient": c, "scale": s}


def unary(kind, o):
    return {"kind": kind, "operand": o}


def binary(kind, l, r):
    return {"kind": kind, "left": l, "right": r}


def rescale(o, s):
    return {"kind": "rescale", "operand": o, "scale": s}


def pred(p, op, l, r):
    return {"proposition": p, "operator": op, "left": l, "right": r}


def table(expressions, predicates):
    return {"schema_version": "tl-syntax.predicate-table/v1", "expressions": expressions, "predicates": predicates}


VALID = {
    "every-node-and-operator": table(
        [
            sig(2), integer(3), binary("add", 0, 1), binary("sub", 0, 1), unary("neg", 0),
            unary("abs", 0), binary("mul", 0, 1), sig(3), rescale(0, 1), decimal(255, 1),
            sig(4), binary("mul", 10, 7), unary("abs", 10), decimal(0, 3), sig(6), unary("abs", 14),
        ],
        [
            pred(10, "lt", 0, 1), pred(11, "le", 2, 1), pred(12, "gt", 7, 9), pred(13, "ge", 8, 9),
            pred(14, "eq", 4, 5), pred(15, "ne", 6, 3), pred(16, "lt", 11, 13), pred(17, "le", 12, 10),
            pred(18, "gt", 15, 1),
        ],
    ),
    "scales-0-1-2-9-18": table(
        [sig(5), binary("mul", 0, 0), decimal(4, 18), rescale(0, 18), sig(2), decimal(5, 1), sig(4), decimal(1, 9)],
        [pred(20, "gt", 1, 2), pred(21, "lt", 3, 1), pred(22, "ge", 4, 4), pred(23, "ne", 6, 6), pred(24, "le", 0, 7)],
    ),
}

chain = [integer(0)] + [unary("neg", i) for i in range(256)]
REFUSED = {
    "range-overflow": (table([integer(I64_MAX), integer(1), binary("add", 0, 1)], []), "RangeOverflow"),
    "scale-mismatch": (table([sig(2), sig(3), binary("add", 0, 1)], []), "ScaleMismatch"),
    "scale-overflow": (table([decimal(1, 18), decimal(1, 1), binary("mul", 0, 1)], []), "ScaleOverflow"),
    "rescale-narrowing": (table([sig(3), rescale(0, 0)], []), "RescaleNarrowing"),
    "non-numeric-signal": (table([sig(1)], []), "NonNumericSignal"),
    "unknown-signal": (table([sig(99)], []), "UnknownSignal"),
    "expression-depth-limit": (table(chain, []), "ExpressionLimitExceeded"),
    "operand-not-preceding": (table([unary("neg", 0)], []), "OperandNotPreceding"),
    "slot-outside-template": (table([{"kind": "slot_signal", "depth": 0}], []), "SlotOutsideTemplate"),
    "predicate-scale-mismatch": (table([sig(2), sig(3)], [pred(1, "lt", 0, 1)]), "PredicateScaleMismatch"),
    "proposition-not-increasing": (table([sig(2)], [pred(9, "lt", 0, 0), pred(9, "gt", 0, 0)]), "PropositionNotIncreasing"),
    "root-out-of-range": (table([sig(2)], [pred(9, "lt", 0, 5)]), "RootOutOfRange"),
}

BASE_WIRE = canonical(table([sig(2), integer(3)], [pred(9, "lt", 0, 1)]))
WIRE = {
    "float": (BASE_WIRE.replace('"value":3', '"value":3.0'), "InvalidDocument"),
    "exponent": (BASE_WIRE.replace('"value":3', '"value":3e0'), "InvalidDocument"),
    "negative-zero": (BASE_WIRE.replace('"value":3', '"value":-0'), "NonCanonicalDocument"),
    "leading-zero": (BASE_WIRE.replace('"value":3', '"value":03'), "InvalidDocument"),
    "duplicate-member": (BASE_WIRE.replace('"value":3', '"value":3,"value":3'), "InvalidDocument"),
    "unknown-member": (BASE_WIRE.replace('"value":3', '"value":3,"unit":"m"'), "InvalidDocument"),
    "trailing-data": (BASE_WIRE + " {}", "InvalidDocument"),
}


# ------------------------------------------------------------ aggregation ---
# FR-308: Boolean members, trace length 1, k in 0..=5, n in 0..=6. The
# hand-written forms below are built independently of the crate's expansion
# (left-leaning chains and a disjunction of all n-subsets).

BOOLEAN_CATALOG = {
    "schema_version": "tl-syntax.signal-catalog/v1",
    "signals": [{"id": i, "name": f"b{i}", "domain": {"kind": "boolean"}} for i in range(1, 6)],
    "bindings": [{"proposition": i, "signal": i} for i in range(1, 6)],
}


def chain(nodes, leaves, kind):
    current = leaves[0]
    for leaf in leaves[1:]:
        nodes.append({"kind": kind, "left": current, "right": leaf})
        current = len(nodes) - 1
    return current


def handwritten(quantifier, k, n):
    nodes = []

    def constant(value):
        nodes.append({"kind": "true" if value else "false"})
        return len(nodes) - 1

    def atoms(members):
        ids = []
        for member in members:
            nodes.append({"kind": "proposition", "proposition": member})
            ids.append(len(nodes) - 1)
        return ids

    members = list(range(1, k + 1))
    if quantifier == "for_each":
        root = constant(True) if k == 0 else chain(nodes, atoms(members), "and")
    elif quantifier == "for_some":
        root = constant(False) if k == 0 else chain(nodes, atoms(members), "or")
    elif n == 0:
        root = constant(True)
    elif n > k:
        root = constant(False)
    else:
        terms = [chain(nodes, atoms(subset), "and") for subset in itertools.combinations(members, n)]
        root = chain(nodes, terms, "or")
    return {"schema_version": "tl-syntax.formula/v1", "semantic_profile": "mltl.closed-trace/v1", "root": root, "nodes": nodes}


def aggregation_document(quantifier, k, n):
    q = {"kind": quantifier} if quantifier != "at_least" else {"kind": "at_least", "n": n}
    return {
        "schema_version": "tl-syntax.set-aggregation/v1",
        "sets": [{"name": "members", "members": list(range(1, k + 1))}],
        "first_proposition": 100,
        "aggregations": [{
            "set": "members",
            "quantifier": q,
            "template": {"nodes": [{"kind": "slot_proposition", "depth": 0}], "root": 0, "expressions": [], "predicates": []},
        }],
    }


def aggregation_cases():
    grid = [("for_each", k, 0) for k in range(6)] + [("for_some", k, 0) for k in range(6)]
    grid += [("at_least", k, n) for k in range(6) for n in range(7)]
    for quantifier, k, n in grid:
        name = f"{quantifier}-k{k}" + (f"-n{n}" if quantifier == "at_least" else "")
        yield name, quantifier, k, n


# --------------------------------------------------------------- contracts --
# FR-309: one formula/v2 graph and contract sets bound to it by the
# CONTENT_IDENTITY_DOMAIN_V1 identity (ADR-019 register item 3, retained).

import hashlib

CONTRACT_FORMULA = {
    "schema_version": "tl-syntax.formula/v2",
    "semantic_profile": "mltl.closed-trace/v1",
    "root": 4,
    "nodes": [
        {"kind": "proposition", "proposition": 1},
        {"kind": "proposition", "proposition": 2},
        {"kind": "future", "interval": {"start": 0, "end": 2}, "operand": 0},
        {"kind": "globally", "interval": {"start": 0, "end": 1}, "operand": 1},
        {"kind": "and", "left": 2, "right": 3},
    ],
}


def content_identity(contract, text):
    digest = hashlib.sha256()
    digest.update(b"tl-syntax.content-identity/v1\0")
    digest.update(contract.encode())
    digest.update(b"\0")
    digest.update(text.encode())
    return digest.hexdigest()


def contract_set(identity, contracts, profile="mltl.closed-trace/v1"):
    return {
        "schema_version": "tl-syntax.contract-set/v1",
        "semantic_profile": profile,
        "graph_identity": identity,
        "contracts": [{"id": i, "name": n, "assumption": a, "guarantee": g} for i, n, a, g in contracts],
    }


def contract_cases():
    identity = content_identity("tl-syntax.formula/v2", canonical(CONTRACT_FORMULA))
    good = [(1, "door-alarm", 2, 3), (2, "same-root", 2, 2)]
    yield "valid-shared-roots", contract_set(identity, good), "valid"
    yield "valid-empty", contract_set(identity, []), "valid"
    yield "root-out-of-range", contract_set(identity, [(1, "a", 9, 3)]), "RootOutOfRange"
    yield "duplicate-name", contract_set(identity, [(1, "a", 2, 3), (2, "a", 2, 3)]), "DuplicateName"
    yield "id-not-increasing", contract_set(identity, [(2, "a", 2, 3), (2, "b", 2, 3)]), "ContractIdNotIncreasing"
    yield "empty-name", contract_set(identity, [(1, "", 2, 3)]), "EmptyName"
    yield "graph-identity-mismatch", contract_set("0" * 64, good), "GraphIdentityMismatch"
    yield "profile-mismatch", contract_set(identity, good, "mltl.online-prefix/v1"), "ProfileMismatch"


def main():
    cases = []
    write("catalogs/catalog-1.json", canonical(CATALOG) + "\n")
    for name, value in VALID.items():
        types = check_table(value)
        write(f"valid/{name}.json", canonical(value))
        write(f"truth/{name}.json", canonical(truth_table(value)) + "\n")
        cases.append({
            "id": name, "class": "valid", "catalog": "catalogs/catalog-1.json",
            "table": f"valid/{name}.json",
            "expected_types": [list(t) for t in types],
            "truth": f"truth/{name}.json",
        })
    for name, (value, refusal) in REFUSED.items():
        try:
            check_table(value)
        except Refused as refused:
            assert refused.cls == refusal, (name, refused.cls, refusal)
        else:
            raise AssertionError(f"{name} was not refused")
        write(f"refused/{name}.json", canonical(value))
        cases.append({"id": name, "class": "refused-table", "catalog": "catalogs/catalog-1.json", "table": f"refused/{name}.json", "refusal": refusal})
    for name, (text, refusal) in WIRE.items():
        write(f"wire/{name}.json", text)
        cases.append({"id": f"wire-{name}", "class": "refused-wire", "catalog": "catalogs/catalog-1.json", "table": f"wire/{name}.json", "refusal": refusal})
    write("catalogs/boolean-members.json", canonical(BOOLEAN_CATALOG) + "\n")
    compared = 0
    for name, quantifier, k, n in aggregation_cases():
        write(f"aggregation/{name}.json", canonical(aggregation_document(quantifier, k, n)))
        write(f"handwritten/{name}.json", canonical(handwritten(quantifier, k, n)) + "\n")
        traces = 2**k
        compared += traces
        cases.append({
            "id": f"aggregation-{name}", "class": "aggregation", "catalog": "catalogs/boolean-members.json",
            "document": f"aggregation/{name}.json", "handwritten": f"handwritten/{name}.json",
            "population": {"members": k, "valuations": [False, True], "trace_length": 1},
            "traces": traces,
        })
    assert compared == 567, compared
    write("contracts/formula.json", canonical(CONTRACT_FORMULA))
    for name, document, outcome in contract_cases():
        write(f"contracts/{name}.json", canonical(document))
        cases.append({
            "id": f"contract-{name}", "class": "contract-set", "formula": "contracts/formula.json",
            "document": f"contracts/{name}.json", "outcome": outcome,
        })
    counts = {}
    for case in cases:
        counts[case["class"]] = counts.get(case["class"], 0) + 1
    manifest = {
        "corpus": "tl-syntax.numeric-corpus/v1",
        "role": "evidence-input",
        "predicate_table": "tl-syntax.predicate-table/v1",
        "generator": "provenance/generate.py",
        "class_counts": counts,
        "aggregation_compared_traces": compared,
        "cases": cases,
    }
    write("manifest.json", json.dumps(manifest, indent=2) + "\n")


if __name__ == "__main__":
    sys.exit(main())
