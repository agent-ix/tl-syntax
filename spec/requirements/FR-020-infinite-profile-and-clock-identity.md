---
id: FR-020
title: "Bind the TL infinite-trace profile and exact clock identity"
type: FR
relationships:
  - target: ix://agent-ix/tl-syntax/FR-003
    type: depends_on
  - target: ix://agent-ix/tl-syntax/FR-012
    type: depends_on
  - target: ix://agent-ix/tl-syntax/FR-289
    type: depends_on
  - target: ix://agent-ix/quire-specification/FR-250
    type: references
---

# FR-020: Bind the TL infinite-trace profile and exact clock identity

## Description

When an infinite-trace formula or trace crosses a TL boundary, tl-syntax shall
identify it with `mltl.infinite-trace/v1` and preserve its exact position-clock
binding. QSL's `quire.temporal.infinite-trace/v1` is a correspondence used for
result comparison; QSL infinite-trace clauses do not lower into a TL target.

## Inputs

- A formula-unbounded document and an optional lasso or partial-valuation trace.
- The exact `event_position` clock identity. The fixed-sample clock admitted
  for finite histories by [FR-012](./FR-012-history-anchor-progress.md) remains
  a distinct, unsupported selection for this infinite-trace edition.

## Outputs

- The distinct TL profile identity and the unchanged clock binding, or a typed
  profile or clock refusal before a combined input is constructed.

## Behavior

`mltl.infinite-trace/v1` is distinct from `mltl.closed-trace/v1`,
`mltl.online-prefix/v1`, and `mltl.origin-complete-history/v1`. A caller
selects it explicitly; neither an unbounded spelling nor a QSL member name
selects it implicitly. Formula v1/v2 and their existing profile identities
retain their serialized bytes and meaning.

Intervals measure discrete event positions. An infinite lasso cannot claim an
exact fixed-sample timestamp at every unbounded repetition within a bounded
integer domain, so V1 admits only `event_position`. Fixed-sample, dense, and
timestamped clocks refuse on the clock axis; no duration conversion, implicit
resampling, rounding, or wall-clock inference is admitted. Formula, trace,
fairness premises, and result attribution must name the same clock identity.

## Acceptance Criteria

| ID | Criteria | Verification |
|---|---|---|
| FR-020-AC-1 | The four TL profile identities remain distinct; selecting the QSL member name or omitting the TL profile refuses instead of silently choosing `mltl.infinite-trace/v1`. | Test (TC-148) |
| FR-020-AC-2 | `event_position` is preserved across formula, trace, fairness, and result attribution; fixed-sample, dense, timestamped, or mismatched clock selections refuse on the clock axis before evaluation. | Test (TC-149) |
| FR-020-AC-3 | Existing formula v1/v2 golden bytes remain identical and QSL-to-TL infinite-trace correspondence is recorded as result comparison, never lowering. | Test (TC-150), Inspection |

## Dependencies

FR-003 owns identity preservation, FR-012 supplies the distinct finite-history
clock precedent, and FR-289 owns unbounded formula admission.
