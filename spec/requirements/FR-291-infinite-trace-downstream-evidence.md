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

- A `tl-syntax.formula-unbounded/v1` document and its facet identity.
- A registered backend's proved/refuted/inconclusive/failed disposition, or
  the FR-290 `unsupported` absence settlement.

## Outputs

- Evidence attributable to the same canonical graph and facet identity the
  registered backend consumed.
- Routed implementation tickets with one owner and dependency per concern.

## Behavior

A registered backend consumes only the `tl-syntax.formula-unbounded/v1`
canonical graph FR-289 admits; it acquires no independent unbounded-interval
grammar, matching the FR-010 rule that a downstream component operates only on
the canonical graph tl-syntax produces. No component other than tl-syntax
mints or amends `tl-syntax.liveness/v1` or the interval grammar it names.

Dependency order:

1. tl-syntax issue [#73](https://github.com/agent-ix/tl-syntax/issues/73)
   owns the `UnboundedInterval` value, the `tl-syntax.formula-unbounded/v1`
   document, and the `tl-syntax.liveness/v1` registration boundary FR-289 and
   FR-290 specify.
2. [quire-specification#112](https://github.com/agent-ix/quire-specification/issues/112)
   mints the `quire.temporal.infinite-trace/v1` facet member this profile
   admits under and owns the native-side settlement-basis mapping.
3. tl-mltl issues [#68](https://github.com/agent-ix/tl-mltl/issues/68) and
   [#72](https://github.com/agent-ix/tl-mltl/issues/72) register the first
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
| FR-291-AC-1 | A registered backend's evidence is attributable to the exact `tl-syntax.formula-unbounded/v1` canonical graph and facet identity it consumed, under both the proved/refuted/inconclusive/failed and the FR-290 `unsupported` dispositions. | Test (TC-147) |
| FR-291-AC-2 | No component other than tl-syntax mints or amends `tl-syntax.liveness/v1` or the FR-289 interval grammar; a registered backend acquires no independent grammar. | Test (TC-147) |
| FR-291-AC-3 | The stated dependency order routes tl-syntax#73, quire-specification#112, and tl-mltl#68/#72 without authorizing implementation ahead of the FR-289/FR-290 acceptance gates. | Test (TC-147) |

## Dependencies

Depends on FR-010's downstream-evidence pattern, FR-289's admitted grammar, and
FR-290's capability and absence contract.
