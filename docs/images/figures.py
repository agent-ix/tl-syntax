# SPDX-License-Identifier: MIT OR Apache-2.0
# Copyright (C) 2026 Agent-IX
"""tl-syntax documentation figures, rendered by the docs-figures skill.

Regenerate from the repository root with the agent-ix/dev-tools skill:

    python3 -m pip install fonttools
    python3 <dev-tools>/skills/docs-figures/scripts/render.py docs/images/figures.py

Add --check to confirm the committed SVGs are current. Counts and identifiers
are read from this repository: the formula fixture, the signal-catalog fixture,
the semantic profiles in src/formula/profile.rs and the corpus manifests.

The block between the two SHARED markers is identical in every TL repository
(tl-syntax, tl-parse, tl-rewrite, tl-mltl, tl-oracle, quire-mltl). Copy it
verbatim and call tl_stack(here="<crate>") and stack_pipeline(here="<crate>").
"""

import json
import re
from pathlib import Path

from docs_figures import FONTS, SMALL, TITLE, Figure  # noqa: F401

STYLE = "ix-docs"

ROOT = Path(__file__).resolve().parents[2]

# One role per kind of thing, fixed across the TL and Quire repositories.
ROLES = [
    ("code", "authored source", "box"),
    ("logic", "processing step", "box"),
    ("model", "checked document / view", "box"),
    ("evidence", "outcome", "box"),
    ("ink", "input", "box"),
]


# ==== SHARED: copy verbatim into sibling figures.py (begin) ===================

TL_CRATES = [
    # (column, row, name, one-line role, dependency note)
    (0, 1, "tl-parse", "text to formula graph"),
    (1, 1, "tl-oracle", "independent reference"),
    (2, 1, "tl-mltl", "evaluates traces"),
    (3, 1, "quire-mltl", "the bridge to Quire"),
    (2, 2, "tl-rewrite", "equivalent rewrites"),
    (3, 2, "quire-observation", "Quire observations"),
]


def tl_stack(here="tl-syntax"):
    """The TL crate family as a dependency graph; `here` is drawn highlighted."""
    f = Figure(
        "tl-stack",
        980,
        356,
        "The TL crates. tl-syntax defines the formula graph, profiles and signal catalog, and "
        "tl-parse, tl-oracle, tl-mltl, tl-rewrite and quire-mltl all depend on it. tl-rewrite "
        "depends on tl-mltl. quire-mltl depends on tl-mltl and quire-observation, and is the "
        "single bridge to Quire. Dashed lines are development-only: tl-mltl and tl-rewrite "
        "test against tl-oracle, and tl-rewrite tests against tl-parse.",
    )
    w, h, step = 192, 56, 262
    rows = {0: 24, 1: 140, 2: 256}
    f.text(3 * step, 244, "QUIRE", "muted", "mono", SMALL)
    f.box(0, rows[0], 980, h, "logic" if here == "tl-syntax" else "ink", "tl-syntax",
          "formula graph · semantic profiles · signal catalog · strict documents")
    where = {}
    for col, row, name, sub in TL_CRATES:
        x, y = col * step, rows[row]
        where[name] = (x, y)
        role = "logic" if name == here else "ink"
        f.box(x, y, w, h, role, name, sub, dashed=(name == "quire-observation"))
    for name in ("tl-parse", "tl-oracle", "tl-mltl", "quire-mltl"):
        x, y = where[name]
        f.line([(x + w / 2, rows[0] + h), (x + w / 2, y - 3)], "model")
    f.note(where["tl-parse"][0] + w / 2 + 10, 104, "types", "model")
    mx, my = where["tl-mltl"]
    rx, ry = where["tl-rewrite"]
    f.line([(mx + w / 2, my + h), (rx + w / 2, ry - 3)], "model")
    f.note(mx + w / 2 + 10, my + h + 30, "documents", "model")
    qx, qy = where["quire-mltl"]
    f.line([(mx + w, my + h / 2), (qx - 3, qy + h / 2)], "model")
    f.note(mx + w + 8, my + h / 2 - 8, "results", "model")
    ox, oy = where["quire-observation"]
    f.line([(ox + w / 2, oy - 3), (ox + w / 2, qy + h)], "model")
    f.note(ox + w / 2 + 10, qy + h + 30, "observations", "model")
    # development-only edges: tl-oracle and tl-parse feed tests of tl-mltl and tl-rewrite
    px, py = where["tl-parse"]
    zx, zy = where["tl-oracle"]
    f.line([(zx + w, zy + h / 2), (mx - 3, my + h / 2)], "model", dashed=True)
    f.line([(zx + w / 2, zy + h), (zx + w / 2, ry + 18), (rx - 3, ry + 18)], "model", dashed=True)
    f.line([(px + w / 2, py + h), (px + w / 2, ry + 40), (rx - 3, ry + 40)], "model", dashed=True)
    f.legend(0, 350, [("logic", "this repository", "box"), ("ink", "other crate", "box"),
                      ("model", "depends on", "solid"), ("model", "development only", "dash")])
    return f


PIPELINE_STAGES = [
    ("MLTL text", "your formula", "code", None),
    ("tl-parse", "parse", "logic", "tl-parse"),
    ("tl-rewrite", "rewrite", "logic", "tl-rewrite"),
    ("tl-mltl", "evaluate", "logic", "tl-mltl"),
    ("tl-oracle", "tests only", "logic", "tl-oracle"),
    ("quire-mltl", "map to Quire", "logic", "quire-mltl"),
]


def stack_pipeline(here="tl-syntax"):
    """End-to-end flow: text, parse, rewrite, evaluate with an oracle cross-check, Quire view."""
    f = Figure(
        "pipeline",
        980,
        392,
        "A formula moves from text to a Quire outcome. tl-parse turns MLTL text into a formula "
        "document, tl-rewrite returns a rewritten document, and tl-mltl evaluates it against a "
        "trace. In tests, tl-oracle evaluates the same formula and trace independently and the "
        "verdicts are compared. quire-mltl maps the result to a Boolean Value or a typed "
        "NonValue reason. All documents use the vocabulary of tl-syntax.",
    )
    w, h = 140, 56
    step = (980 - w) / (len(PIPELINE_STAGES) - 1)
    cx = [round(i * step + w / 2) for i in range(len(PIPELINE_STAGES))]
    for i, (name, sub, role, crate) in enumerate(PIPELINE_STAGES):
        if crate is not None:
            role = "logic" if crate == here else "ink"
        f.box(cx[i] - w // 2, 0, w, h, role, name, sub)
        f.line([(cx[i], h + 2), (cx[i], 312)], "faint", dashed=True, arrow=False, weight=1.0)
    msgs = [
        (0, 1, 96, "MLTL text", "code", False),
        (1, 2, 140, "formula document", "model", False),
        (2, 3, 184, "rewritten formula", "model", False),
        (3, 4, 228, "formula + trace", "model", True),
        (4, 3, 262, "reference verdict", "evidence", True),
        (3, 5, 306, "result", "model", False),
    ]
    for a, b, y, label, role, dev in msgs:
        sign = 1 if b > a else -1
        f.line([(cx[a] + 3 * sign, y), (cx[b] - 3 * sign, y)], role, dashed=dev,
               weight=2.4 if not dev else 1.5)
        left = min(cx[a], cx[b]) + 10
        f.note(left, y - 8, label, role)
    f.box(980 - 190, 332, 190, 50, "evidence", "Value or NonValue", "true / false or reason")
    f.line([(cx[5], 312), (cx[5], 329)], "evidence")
    f.box(0, 332, 520, 50, "logic" if here == "tl-syntax" else "ink", "tl-syntax",
          "the formula, profile, signal and document types every step uses")
    return f


# ==== SHARED (end) ============================================================


# ==== SHARED SCENARIO: copy verbatim into sibling figures.py (begin) ==========

SCENARIO_LANES = [
    # (name, one-word role, step number whose look-here callout sits on this lane)
    ("engineer", "you", None),
    ("tl-parse", "text", 1),
    ("tl-syntax", "signals", 2),
    ("tl-rewrite", "normalize", 3),
    ("tl-mltl", "evaluate", 4),
    ("R2U2", "outside", None),
    ("tl-oracle", "reference", 5),
    ("quire-mltl", "assurance", 6),
]

SCENARIO_LOOK = [
    # (step, crate, what to look at) for the case r2u2-future-deadline-v1
    (1, "tl-parse", "the tree is one F node, interval [0,3], over p2"),
    (2, "tl-syntax", "p2 is bound to signal s2, the signal the requirement waits for"),
    (3, "tl-rewrite", "status Unchanged under closed-trace: no rule applies, no step recorded; online-prefix has no rule set"),
    (4, "tl-mltl", "s2 is absent at instants 0 to 3, so false at time 0; R2U2 agrees"),
    (5, "tl-oracle", "a second evaluator reads the same formula and trace and returns false"),
    (6, "quire-mltl", "false becomes Value false only if the observation views allow a value"),
]


def scenario_sequence(here=None):
    """The debugging walkthrough as a sequence diagram; the lane named `here` is highlighted."""
    f = Figure(
        "scenario-sequence",
        980,
        684,
        "A deadline requirement fails on a recorded trace, and the engineer asks why. The engineer "
        "sends F[0,3] p2 to tl-parse and gets a formula document, binds p2 to signal s2 with tl-syntax, "
        "and sends the document to tl-rewrite, which reports Unchanged under the closed-trace profile. tl-mltl evaluates the formula "
        "and trace and returns false at time 0 with a buffer of 4 instants, and R2U2, which runs outside "
        "the crates, returns false at time 0 for the C2PO expression F[0,3](p2). tl-oracle evaluates the "
        "same formula and trace independently and returns false. quire-mltl maps the result to Value false. "
        "Numbered rings mark where to look at each step.",
    )
    w, h = 112, 50
    step = (980 - w) / (len(SCENARIO_LANES) - 1)
    cx = [round(i * step + w / 2) for i in range(len(SCENARIO_LANES))]
    lane = {name: i for i, (name, _, _) in enumerate(SCENARIO_LANES)}
    bottom = 492
    for i, (name, sub, _) in enumerate(SCENARIO_LANES):
        if name == "engineer":
            role = "code"
        elif name == here:
            role = "logic"
        else:
            role = "ink"
        f.box(cx[i] - w // 2, 0, w, h, role, name, sub, dashed=(name == "R2U2"))
        f.line([(cx[i], h + 2), (cx[i], bottom)], "faint", dashed=True, arrow=False, weight=1.0)
    y = 82
    pairs = [
        # (from, to, go label, go role, back label, back role, dashed, ring step)
        ("engineer", "tl-parse", "F[0,3] p2", "code", "formula doc", "model", False, 1),
        ("engineer", "tl-syntax", "signal catalog", "ink", "p2 bound to s2", "model", False, 2),
        ("engineer", "tl-rewrite", "formula doc", "model", "Unchanged", "evidence", False, 3),
        ("engineer", "tl-mltl", "formula + trace", "model", "false at 0, buffer 4", "evidence", False, 4),
        ("tl-mltl", "R2U2", "F[0,3](p2)", "model", "false at 0", "evidence", True, None),
        ("engineer", "tl-oracle", "formula + trace", "model", "false", "evidence", True, 5),
        ("engineer", "quire-mltl", "result", "model", "Value false", "evidence", False, 6),
    ]
    for a, b, go, go_role, back, back_role, dev, ring in pairs:
        ia, ib = lane[a], lane[b]
        f.line([(cx[ia] + 3, y), (cx[ib] - 3, y)], go_role, dashed=dev, weight=1.5 if dev else 2.4)
        f.text(cx[ia] + 12, y - 7, go, go_role, "mono", SMALL)
        yb = y + 26
        f.line([(cx[ib] - 3, yb), (cx[ia] + 3, yb)], back_role, dashed=dev, weight=1.5 if dev else 2.4)
        f.text(cx[ia] + 26, yb - 7, back, back_role, "mono", SMALL)
        if ring:
            rx, ry = cx[ib] + 22, y + 13
            f.dot(rx, ry, 9, "evidence", filled=False)
            f.text(rx, ry + 4, str(ring), "evidence", "mono", SMALL, "middle")
        y += 62
    f.text(0, bottom + 32, "Look here", "muted", "title", TITLE)
    ly = bottom + 56
    for n, crate, what in SCENARIO_LOOK:
        f.dot(9, ly - 4, 9, "evidence", filled=False)
        f.text(9, ly, str(n), "evidence", "mono", SMALL, "middle")
        f.text(30, ly, crate, "logic" if crate == here else "muted", "mono", SMALL)
        f.text(140, ly, what, "ink", "mono", SMALL)
        ly += 22
    return f


# ==== SHARED SCENARIO (end) ===================================================


# ---- repository data ---------------------------------------------------------


def load(path):
    return json.loads((ROOT / path).read_text(encoding="utf-8"))


def profiles():
    """Semantic profile identifiers declared in src/formula/profile.rs, in source order."""
    text = (ROOT / "src/formula/profile.rs").read_text(encoding="utf-8")
    arms = re.findall(r'Self::\w+ => "(mltl\.[a-z-]+/v\d+)"', text)
    return arms


def domain_text(domain):
    kind = domain["kind"]
    if kind == "boolean":
        return "boolean"
    if kind == "integer":
        return f"integer {domain['minimum']}..{domain['maximum']}"
    return (f"fixed decimal {domain['minimum_coefficient']}..{domain['maximum_coefficient']}"
            f" scale {domain['scale']}")


def node_text(index, node):
    kind = node["kind"]
    if kind == "proposition":
        return f"n{index}  proposition {node['proposition']}"
    iv = node["interval"]
    return f"n{index}  {kind} [{iv['start']},{iv['end']}] of n{node['operand']}"


def corpus_counts():
    shared = len(load("corpus/manifest.json")["fixtures"])
    future = len(load("corpus/future-operators/cases.json")["cases"])
    past = load("corpus/past-history/cases.json")["evaluations"]
    infinite = len(load("corpus/infinite-trace/cases.json")["cases"])
    return [("tl-syntax-corpus/v1", shared), ("future-operators", future),
            ("past-history evaluations", len(past)), ("infinite-trace", infinite)]


# ---- figures -----------------------------------------------------------------


def hero():
    """README hero: a checked formula graph and a checked signal catalog meet in a binding."""
    formula = load("corpus/formulas/short-trace-future.json")
    catalog = load("tests/fixtures/valid-signal-catalog.json")
    nodes = formula["nodes"]
    signals = catalog["signals"]
    n_bind = len(catalog["bindings"])
    n_prof = len(profiles())
    f = Figure(
        "hero",
        980,
        426,
        f"A node table of {len(nodes)} nodes and a semantic profile, {formula['semantic_profile']}, "
        f"go into Formula::new, which returns a borrowed Formula or the first FormulaError. "
        f"A signal catalog of {len(signals)} signals and {n_bind} binding goes into SignalCatalog::new, "
        f"which returns a SignalCatalog or the first SignalCatalogError. bind_formula joins the "
        f"two into a BoundFormula when every proposition has a binding to a Boolean signal. "
        f"The crate defines {n_prof} semantic profiles.",
    )
    f.text(0, 12, "FORMULA GRAPH", "muted", "mono", SMALL)
    f.box(0, 24, 380, 68, "code", "node table", *[node_text(i, n) for i, n in enumerate(nodes)])
    f.box(0, 112, 380, 56, "ink", "semantic profile", formula["semantic_profile"])
    f.box(450, 56, 190, 62, "logic", "Formula::new", "order · profile fit")
    f.box(690, 56, 130, 62, "model", "Formula", "borrowed view")
    f.box(450, 150, 190, 56, "evidence", "FormulaError", "first bad node")
    f.line([(380, 58), (410, 58), (410, 80), (447, 80)], "code")
    f.note(386, 52, "nodes", "code")
    f.line([(380, 140), (410, 140), (410, 96), (447, 96)], "ink")
    f.note(386, 160, "profile", "ink")
    f.line([(640, 87), (687, 87)], "logic", weight=2.4)
    f.line([(545, 118), (545, 147)], "evidence")
    f.text(0, 236, "SIGNALS", "muted", "mono", SMALL)
    f.box(0, 248, 380, 78, "code", "signal catalog",
          *[f"{s['id']}  {s['name']}  {domain_text(s['domain'])}" for s in signals])
    f.box(0, 346, 380, 56, "code", "bindings", f"{n_bind} proposition to Boolean signal")
    f.box(450, 282, 190, 62, "logic", "SignalCatalog::new", "ids · names · targets")
    f.box(690, 282, 130, 62, "model", "SignalCatalog", "borrowed view")
    f.box(450, 366, 190, 36, "evidence", "SignalCatalogError")
    f.line([(380, 287), (410, 287), (410, 306), (447, 306)], "code")
    f.line([(380, 374), (410, 374), (410, 324), (447, 324)], "code")
    f.line([(640, 313), (687, 313)], "logic", weight=2.4)
    f.line([(545, 344), (545, 363)], "evidence")
    f.box(840, 160, 140, 62, "logic", "bind_formula", "all bound")
    f.box(840, 282, 140, 62, "model", "BoundFormula", "formula + catalog")
    f.line([(820, 87), (830, 87), (830, 180), (837, 180)], "model")
    f.line([(820, 313), (830, 313), (830, 204), (837, 204)], "model")
    f.line([(910, 222), (910, 279)], "logic", weight=2.4)
    f.legend(0, 422, [ROLES[0], ROLES[1], ROLES[2], ROLES[3], ROLES[4]])
    return f


def tl_stack_here():
    return tl_stack(here="tl-syntax")


def pipeline_here():
    return stack_pipeline(here="tl-syntax")


def scenario_sequence_here():
    return scenario_sequence(here="tl-syntax")


def corpus():
    """Cases in each checked-in corpus, counted from the corpus files."""
    counts = corpus_counts()
    total = sum(n for _, n in counts)
    f = Figure(
        "corpus",
        980,
        236,
        f"The checked-in corpora hold {total} counted cases: "
        + ", ".join(f"{name} {n}" for name, n in counts)
        + ".",
    )
    f.text(0, 16, "CASES PER CORPUS", "muted", "mono", SMALL)
    f.columns(0, 30, 980, 202, [n for _, n in counts], labels=[name for name, _ in counts], role="evidence")
    return f


FIGURES = [hero, tl_stack_here, pipeline_here, scenario_sequence_here, corpus]
