---
id: FR-289
title: "Admit unbounded intervals under the infinite-trace facet"
type: FR
relationships:
  - target: ix://agent-ix/tl-syntax/MRS-002
    type: implements
  - target: ix://agent-ix/tl-syntax/FR-001
    type: depends_on
  - target: ix://agent-ix/tl-syntax/FR-008
    type: depends_on
  - target: ix://agent-ix/tl-syntax/FR-009
    type: depends_on
  - target: ix://agent-ix/quire-specification/FR-250
    type: depends_on
---

# FR-289: Admit unbounded intervals under the infinite-trace facet

## Description

When a future-time primitive (`F`, `G`, `U`, `R`) or, by FR-008 lowering, a
derived operator (`W`, `M`) is bound to the `quire.temporal.infinite-trace/v1`
facet member (minted in
[quire-specification#112](https://github.com/agent-ix/quire-specification/issues/112)),
tl-syntax shall accept an interval whose upper bound is absent (unbounded) in
addition to the existing closed `[a,b]` form from FR-001.

## Inputs

- A checked lower discrete bound `a` with `0 <= a <= u32::MAX`, and no upper
  bound token.
- The containing document's facet identity, which must equal exactly
  `quire.temporal.infinite-trace/v1`.

## Outputs

- A validated `UnboundedInterval` value carrying only `a`, distinct from the
  closed `Interval` type FR-001 defines, or a typed refusal when the facet
  identity is absent, unknown, or mismatched.
- A `tl-syntax.formula-unbounded/v1` document: a new, co-existing formula
  edition, structurally identical to `tl-syntax.formula/v1` in node vocabulary
  (the same closed `NodeKind` set, including the FR-008 W/M lowering targets)
  and differing only in that an `F`, `G`, `U`, or `R` node's interval field may
  hold either a closed `Interval` or an `UnboundedInterval`.

## Behavior

The admitted grammar is `[a,)`: a checked lower bound followed by a comma and
an explicit open upper position, with no numeric upper token. `[a,)` and
`[a,b]` are the only two interval forms an infinite-trace-facet document
admits; every other spelling, including a bare `[a,` without the closing
paren, an omitted lower bound, or an upper bound spelled as a sentinel value,
is refused as malformed rather than accepted as unbounded.

`F[a,)` and `G[a,)` are primitive: "eventually from `a`" and "always from `a`"
over the infinite suffix beginning at offset `a`. `U[a,)` and `R[a,)` retain
their FR-008 dependency-order and false-extension meaning with the closing
bound removed rather than substituted; interval semantics beyond that removed
bound are FR-291's downstream-evaluator concern, not this admission boundary's.
`W[a,) = p U[a,) q OR G[a,) p` and `M[a,) = (p R[a,) q) AND F[a,) p` apply the
existing FR-008 equations unchanged: lowering an unbounded interval requires no
new equation, only that the request/report interval field admit the
`UnboundedInterval` variant.

`tl-syntax.formula-unbounded/v1` is a new edition, not a successor of
`tl-syntax.formula/v1`, matching quire-specification#112's ruling that the
unbounded operator grammar is minted as a new exact edition rather than an
amendment of the bounded one. This keeps every existing `tl-syntax.formula/v1`
consumer provably bounded: it can never receive a document carrying an
`UnboundedInterval` it has no representation for, and an unknown schema string
fails deserialization under the existing FR-004 rule rather than silently
downgrading.

A document bound to `quire.temporal.infinite-trace/v1` but built under
`tl-syntax.formula/v1`, or one using `UnboundedInterval` without that facet
identity, is refused before construction. Admitting this grammar makes no
evaluation, liveness, lasso, or fairness claim; FR-290 governs what a consumer
without a registered liveness backend does with an admitted unbounded formula.

## Acceptance Criteria

| ID | Criteria | Verification |
|---|---|---|
| FR-289-AC-1 | For every valid lower bound, `[a,)` constructs an `UnboundedInterval` distinct from any closed `Interval`, and `[a,b]` continues to construct the existing closed form, under the `quire.temporal.infinite-trace/v1` facet. | Test (TC-144) |
| FR-289-AC-2 | `F`, `G`, `U`, `R` and, by unchanged FR-008 lowering, `W`, `M` each accept an `UnboundedInterval` operand under `tl-syntax.formula-unbounded/v1`; no other `NodeKind` accepts one. | Test (TC-144, TC-145) |
| FR-289-AC-3 | `tl-syntax.formula-unbounded/v1` round-trips and is rejected as an unknown schema by an unmodified `tl-syntax.formula/v1` decoder; a `tl-syntax.formula/v1` document never carries an `UnboundedInterval`. | Test (TC-145) |
| FR-289-AC-4 | A missing, unknown, or mismatched facet identity, and every malformed unbounded-interval spelling, are refused before document construction and identify the mismatched axis. | Test (TC-144) |

## Dependencies

FR-001 supplies the closed interval this requirement extends alongside. FR-008
supplies the W/M lowering equations this requirement reuses unchanged. FR-009
governs where this edition sits in the compatibility axis table. FR-290
governs the liveness-capability absence path for a document this requirement
admits.
