---
id: ADR-001
title: "Lower derived future operators into the canonical v1 graph"
type: ADR
status: accepted
owner: tl-syntax-maintainer
relationships:
  - target: ix://agent-ix/tl-syntax/MRS-002
    type: depends_on
  - target: ix://agent-ix/tl-syntax/FR-008
    type: depends_on
  - target: ix://agent-ix/tl-syntax/FR-009
    type: depends_on
  - target: ix://agent-ix/tl-syntax/FR-010
    type: depends_on
---

# ADR-001: Lower derived future operators into the canonical v1 graph

## Context

The ecosystem already exchanges and evaluates one bounded future-time graph
with F/G/U/R primitives and two explicit finite-trace semantic profiles. Adding
surface conveniences as new graph nodes would require duplicate evaluator,
horizon, rewrite, schema, corpus, and interoperability semantics and would make
old formula-v1 consumers face new wire variants.

Native Quire is the sole editable formal-clause language. TL notation is an
internal interchange convenience and may not become a second clause authority.

## Decision

Adopt `tl-syntax.future-operators/v1` as a closed operator-profile contract.
Admit bounded weak until `W` and bounded strong release `M` only as the exact
structural lowerings in FR-008. Lower them before formula
validation and exchange. Keep `NodeKind`, `tl-syntax.formula/v1`,
`mltl.closed-trace/v1`, and `mltl.online-prefix/v1` unchanged.

Canonical formatting emits primitive syntax and never reconstructs sugar.
Generated nodes retain the full derived-expression span while a non-wire
lowering report retains the distinct operator-token span; semantic identity
continues to ignore spans. Unknown or
unsupported profiles and spellings refuse before producing a document.

## Alternatives rejected

- **New derived `NodeKind` variants:** rejected because every downstream
  semantic component would acquire a parallel implementation and old wire
  consumers would need a schema change.
- **Formatter re-sugaring:** rejected because multiple equivalent source forms
  would make canonical output depend on provenance that semantic formula
  identity intentionally excludes.
- **Strong or weak next:** deferred because the current false-extension graph
  has no end-of-trace existence atom; treating `F[1,1]` as strong next or
  `G[1,1]` as weak next gives the wrong closed-boundary result for part of the
  Boolean formula domain.
- **One mixed future/past profile:** rejected because #33 owns a separately
  versioned past/history contract and its origin/history behavior is not a
  future-profile detail.
- **External-tool acceptance as semantic evidence:** rejected because target
  parser acceptance is not equivalence and foreign runtimes are excluded from
  production and qualification paths.

## Consequences

The parser and adapters require bounded graph-building APIs but the evaluator
and rewriter gain no operator cases. Derived input normalizes to text that old
v1 parsers can consume. Formula-v1 documents stay compatible. Node budgets
become observable at the lowering boundary, so the three-node expansion charge and
all-or-nothing failure are normative. Target-specific loss remains visible.

This decision authorizes specifications and routed implementation tickets only.
It does not authorize a native Quire grammar change, a monitor runtime, a
release, or an equivalence claim for an external tool.
