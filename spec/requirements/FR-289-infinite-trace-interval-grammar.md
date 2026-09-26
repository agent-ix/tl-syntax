---
id: FR-289
title: "Admit unbounded intervals under the TL infinite-trace profile"
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

# FR-289: Admit unbounded intervals under the TL infinite-trace profile

## Description

When a temporal primitive (`F`, `G`, `U`, `R`, `O`, `H`, `S`, `T`) or, by FR-008
lowering, a derived operator (`W`, `M`) is bound to the TL-native
`mltl.infinite-trace/v1` profile, tl-syntax shall accept an interval whose
upper bound is absent in addition to the existing closed `[a,b]` form from
FR-001. The QSL `quire.temporal.infinite-trace/v1` member is a separately
owned correspondence, not a TL profile identity or a lowering target.

## Inputs

- A checked lower discrete bound `a` with `0 <= a <= u32::MAX`, and no upper
  bound token.
- The containing document's semantic profile, which must equal exactly
  `mltl.infinite-trace/v1`.

## Outputs

- A validated `UnboundedInterval` value carrying only `a`, distinct from the
  closed `Interval` type FR-001 defines, or a typed refusal when the profile
  identity is absent, unknown, or mismatched.
- A `tl-syntax.formula-unbounded/v1` document: a new, co-existing formula
  edition whose node vocabulary includes the bounded future and past
  primitives already present in formula v1/v2. The interval field on
  `F/G/U/R/O/H/S/T` admits a closed or unbounded interval; `Y` remains the
  existing one-position past operator. Future and past nodes may occur in one
  infinite-trace graph. W/M lower into the future primitives before wire
  construction.

## Behavior

The admitted grammar is `[a,)`: a checked lower bound followed by a comma and
an explicit open upper position, with no numeric upper token. `[a,)` and
`[a,b]` are the two interval forms an infinite-trace-facet document admits,
and that admission is closed: a spelling outside those two forms is refused
before construction and identifies the mismatched axis.

`F[a,)` and `G[a,)` are primitive: "eventually from `a`" and "always from `a`"
over the infinite suffix beginning at offset `a`. `U[a,)` and `R[a,)` retain
their FR-008 dependency order with the closing bound removed rather than
substituted. The past duals `O/H/S/T[a,)` use the same checked lower-bound
representation. Their evaluation, including the origin boundary, is the
downstream provider's concern rather than this admission boundary's.
`W[a,) = p U[a,) q OR G[a,) p` and `M[a,) = (p R[a,) q) AND F[a,) p` apply the
existing FR-008 equations unchanged: lowering an unbounded interval requires no
new equation, only that the request/report interval field admit the
`UnboundedInterval` variant.

`tl-syntax.formula-unbounded/v1` is a sibling edition, not a successor of
formula v1 or v2. It carries past nodes without changing the bounded v2
edition. Existing v1/v2 consumers remain provably bounded: neither decoder
can receive an `UnboundedInterval` it has no representation for, and the new
schema string fails their FR-004 unknown-schema rule rather than downgrading.

A document bound to `mltl.infinite-trace/v1` but built under
`tl-syntax.formula/v1` or `/v2`, or one using `UnboundedInterval` without that
profile identity, is refused before construction. Admitting this grammar makes no
evaluation, liveness, lasso, or fairness claim; FR-290 governs what a consumer
without a registered liveness backend does with an admitted unbounded formula.

## Acceptance Criteria

| ID | Criteria | Verification |
|---|---|---|
| FR-289-AC-1 | For every valid lower bound, `[a,)` constructs an `UnboundedInterval` distinct from any closed `Interval`, and `[a,b]` continues to construct the existing closed form, under `mltl.infinite-trace/v1`. | Test (TC-144) |
| FR-289-AC-2 | `F/G/U/R/O/H/S/T` and, by FR-008 lowering, `W/M` accept the unbounded interval under `tl-syntax.formula-unbounded/v1`; `Y`, atomic, and Boolean nodes do not. Mixed future/past graphs are admitted only in that edition. | Test (TC-144, TC-145) |
| FR-289-AC-3 | `tl-syntax.formula-unbounded/v1` round-trips, including past nodes, and unmodified formula v1/v2 decoders reject its schema; formula v1/v2 bytes and boundedness remain unchanged. | Test (TC-145) |
| FR-289-AC-4 | A missing, unknown, or mismatched profile identity, and every malformed unbounded-interval spelling, are refused before document construction with the mismatched axis identified. | Test (TC-144) |

## Dependencies

FR-001 supplies the closed interval this requirement extends alongside. FR-008
supplies the W/M lowering equations this requirement reuses unchanged. FR-009
governs where this edition sits in the compatibility axis table. FR-290
governs the liveness-capability absence path for a document this requirement
admits.
