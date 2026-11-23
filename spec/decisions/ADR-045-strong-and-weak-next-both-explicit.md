---
id: ADR-045
title: "Spell strong next and weak next explicitly and give plain next no default"
type: ADR
status: proposed
owner: tl-syntax-maintainer
relationships:
  - target: ix://agent-ix/tl-syntax/FR-333
    type: realizes_requirement
  - target: ix://agent-ix/tl-syntax/ADR-044
    type: depends_on
---

# ADR-045: Spell strong next and weak next explicitly and give plain next no default

## Status

Proposed (TL-427). Ready for human acceptance; not accepted.

## Context

On finite traces the two next operators differ at the last position: strong next
is false there, weak next is true there. R2U2 documentation and the 2026
FRET-to-R2U2 paper (Aurandt et al., NFM 2026) define the bare next differently,
so any default would silently misread one community's formulas.

## Decision

The LTLf edition has two distinct primitive nodes, `StrongNext` and `WeakNext`,
with canonical internal text `Xs` and `Xw`. There is no node, constructor,
serde alias or text spelling for a bare `Next`/`X` in the LTLf edition; a
consumer holding one must choose. Under `mltl.infinite-trace/v1` there is no end
of trace, the two coincide, and the existing `F[1,1]` spelling is kept; tl-parse
may accept a bare `X` there only as a lowering to it (tl-parse ADR-041).
Duality `Xw p = not Xs not p` is a tested law, not a second truth table.

## Alternatives rejected

- Default weak (textbook LTLf `X` is strong in De Giacomo-Vardi; R2U2 differs).
- A single node with a flag: a defaulted flag is a default.
- Only strong next with `Xw` derived: loses the one-node round trip for
  weak-next formulas imported from outside tools.

## Consequences

Importers (tl-mltl) that meet a bare next in a foreign format emit a typed
loss/refusal rather than choosing. `last` is expressible as `Xw false`.
