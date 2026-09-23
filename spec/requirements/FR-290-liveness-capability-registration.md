---
id: FR-290
title: "Register the liveness capability and settle its absence"
type: FR
relationships:
  - target: ix://agent-ix/tl-syntax/MRS-002
    type: implements
  - target: ix://agent-ix/tl-syntax/FR-289
    type: depends_on
  - target: ix://agent-ix/quire-specification/FR-161
    type: references
---

# FR-290: Register the liveness capability and settle its absence

## Description

When a `tl-syntax.formula-unbounded/v1` document is handed to a downstream
consumer for settlement, tl-syntax shall expose one named liveness capability
identity that a downstream evaluator registers against, and an absent
registration shall settle `unsupported` with a warning naming that capability
rather than a hold, block, or refusal error.

## Inputs

- A `tl-syntax.formula-unbounded/v1` document admitted under FR-289.
- The set of liveness backends registered, at the calling boundary, for the
  `tl-syntax.liveness/v1` capability identity.

## Outputs

- Either the registered backend's settlement (`proved`, `refuted`,
  `inconclusive`, `unsupported`, or `failed` per quire-specification FR-341), or,
  absent a registration, an `unsupported` settlement carrying a warning that
  names `tl-syntax.liveness/v1`.
- No mutation of a prior settlement and no partial or silently-downgraded
  result.

## Behavior

`tl-syntax.liveness/v1` is the one capability identity a downstream evaluator
registers against to declare itself able to settle a
`tl-syntax.formula-unbounded/v1` document. Registration is a non-wire Rust API
contract owned by tl-syntax, structurally the same kind of contract as the
FR-008 lowering request/report/refusal identities: a registered backend
implements the typed settlement interface this capability names; an
unregistered caller never receives a silently-downgraded or partial result in
its place.

Only the registered infinite-trace provider may emit `proved`. A bounded or
finite-prefix evaluator cannot construct that claim for an unbounded liveness
formula. The provider retains `resource-incomplete` separately from other
failure reasons even when both map to the external `failed` label.

Absence of a registered backend is never a hold, never a blocker, and never a
refusal error: this is the solver-absence path QSL establishes, and tl-syntax's
liveness capability follows the same contract. A `tl-syntax.formula-unbounded/v1`
document handed to settlement with no backend registered for
`tl-syntax.liveness/v1` settles `unsupported`, carrying a warning that names
`tl-syntax.liveness/v1`, before any evaluation is attempted. This is the
absence path tl-mltl's registration (tl-mltl#68, tl-mltl#72) resolves once it
exists: a registered tl-mltl backend replaces the `unsupported` settlement with
its own proved/refuted/inconclusive/failed disposition; it never replaces the
capability identity or the registration contract itself.

A bounded `tl-syntax.formula/v1` document never consults this capability: it is
evaluated exactly as FR-010 already specifies, and this requirement adds
nothing to that path.

## Acceptance Criteria

| ID | Criteria | Verification |
|---|---|---|
| FR-290-AC-1 | With no backend registered for `tl-syntax.liveness/v1`, every `tl-syntax.formula-unbounded/v1` document settles `unsupported` with a warning naming `tl-syntax.liveness/v1`, before evaluation is attempted. | Test (TC-146) |
| FR-290-AC-2 | With a backend registered for `tl-syntax.liveness/v1`, settlement is routed to that backend's disposition and the `unsupported` warning is not produced. | Test (TC-146) |
| FR-290-AC-3 | The absence settlement is never a hold, block, or refusal error; bounded documents never consult this capability; only a registered infinite-trace provider may emit `proved`. | Test (TC-146) |

## Dependencies

FR-289 supplies the `tl-syntax.formula-unbounded/v1` document this capability
settles. FR-291 supplies the downstream dependency order and evidence a
registered backend must demonstrate.
