---
id: FR-291
title: "Keep infinite-trace liveness evidence downstream and ordered"
type: FR
relationships:
  - target: ix://agent-ix/tl-syntax/MRS-002
    type: implements
  - target: ix://agent-ix/tl-syntax/FR-010
    type: depends_on
  - target: ix://agent-ix/tl-syntax/FR-289
    type: depends_on
  - target: ix://agent-ix/tl-syntax/FR-290
    type: depends_on
---

# FR-291: Keep infinite-trace liveness evidence downstream and ordered

## Description

Where a downstream component registers for the FR-290 `tl-syntax.liveness/v1`
capability, it shall operate only on the FR-289 `tl-syntax.formula-unbounded/v1`
canonical graph and shall route its evidence and implementation order the same
way FR-010 already routes bounded evidence, without acquiring an independent
grammar.

## Inputs

- A `tl-syntax.formula-unbounded/v1` document and its TL profile identity.
- A registered backend's proved/refuted/inconclusive/failed disposition, or
  the FR-290 `unsupported` absence settlement.

## Outputs

- Evidence attributable to the same canonical graph and profile identity the
  registered backend consumed.
- Routed implementation tickets with one owner and dependency per concern.

## Behavior

A registered backend consumes only the `tl-syntax.formula-unbounded/v1`
canonical graph FR-289 admits; it acquires no independent unbounded-interval
grammar, matching the FR-010 rule that a downstream component operates only on
the canonical graph tl-syntax produces. No component other than tl-syntax
mints or amends `tl-syntax.liveness/v1` or the interval grammar it names.

Dependency order:

1. tl-syntax [TL-15](https://linear.app/agent-ix/issue/TL-15)
   owns the `UnboundedInterval` value, the `tl-syntax.formula-unbounded/v1`
   document, and the `tl-syntax.liveness/v1` registration boundary FR-289 and
   FR-290 specify.
2. [STD-13](https://linear.app/agent-ix/issue/STD-13)
   mints the corresponding QSL member and owns the native-side
   settlement-basis mapping; it does not mint the TL profile.
3. [TL-13](https://linear.app/agent-ix/issue/TL-13) and
   [TL-7](https://linear.app/agent-ix/issue/TL-7) register and qualify the first
   `tl-syntax.liveness/v1` backend, demonstrating the FR-161-equivalent
   always/eventually/until/release and past-dual inductive semantics,
   fairness, lasso witnesses, and non-conclusive liveness monitoring against
   the FR-289 graph; they add no interval grammar of their own.

Each issue names its predecessor as a hard dependency; these links route work
and do not authorize implementation before the acceptance gates in the FR-289
and FR-290 files above are satisfied.

## Acceptance Criteria

| ID | Criteria | Verification |
|---|---|---|
| FR-291-AC-1 | A registered backend's evidence is attributable to the exact `tl-syntax.formula-unbounded/v1` canonical graph and TL profile identity it consumed, under every FR-341 disposition and the FR-290 absence path. | Test (TC-147) |
| FR-291-AC-2 | No component other than tl-syntax mints or amends `tl-syntax.liveness/v1` or the FR-289 interval grammar; a registered backend acquires no independent grammar. | Test (TC-147) |
| FR-291-AC-3 | TL-15, STD-13, TL-13, and TL-7 retain their stated dependency order without authorizing implementation ahead of the accepted V1 specification. | Inspection (TC-147) |

## Dependencies

Depends on FR-010's downstream-evidence pattern, FR-289's admitted grammar, and
FR-290's capability and absence contract.
