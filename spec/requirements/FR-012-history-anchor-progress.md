---
id: FR-012
title: "Bind past evaluation to complete history and anchor"
type: FR
relationships:
  - target: ix://agent-ix/tl-syntax/MRS-003
    type: implements
  - target: ix://agent-ix/tl-syntax/FR-003
    type: depends_on
  - target: ix://agent-ix/tl-syntax/FR-011
    type: depends_on
---

# FR-012: Bind past evaluation to complete history and anchor

## Description

When a past formula is evaluated, the evaluator shall require an immutable,
origin-complete history identity and an in-range anchor and shall distinguish a
result at that anchor from progress at later anchors or superseding late data.

## Inputs

- A `tl-mltl.position-history/v1` history identity, revision, digest, origin
  position zero, through-position, and ordered observation positions.
- Evaluation anchor, formula/profile identity, proposition-map identity, and
  evaluator identity/revision.
- A declared event-position clock or an exactly mapped fixed-sample clock.

## Outputs

- A `tl-mltl.past-evaluation/v1` result reference binding every input identity,
  anchor, and `mltl.origin-complete-history/v1`.
- `tl-mltl.history-requirement/v1` analysis in positions and bounded evaluation
  statistics.
- A typed refusal for invalid, incomplete, stale, or unsupported input.

## Behavior

History is complete only when it is nonempty, starts at origin zero, contains
exactly one observation for every position through its declared end, matches
its digest/revision, and includes the anchor. Empty, gapped, duplicate,
out-of-order, stale-digest, or out-of-range-anchor inputs refuse. Pre-origin
offsets generated while evaluating an otherwise complete history use FR-011
false extension and are not classified as gaps.

Required history is a maximum reverse offset, not a minimum physical trace
length:

- atoms and constants require zero;
- Boolean unary nodes retain their child's value and Boolean binary nodes take
  the maximum of their children;
- O/H add the interval upper bound to the child's required history; and
- S/T add the interval upper bound to the maximum child requirement.

All additions produce `u64` and refuse overflow. Evaluation separately enforces
declared recursion, temporal-span, step, and input-position limits. An interval
cardinality over the temporal-span limit refuses before iteration.

A pure-past result is final for its exact anchor once history through that
anchor is complete; it never uses the future-profile Pending value. Advancing
to a later explicit position creates a new result identity. Silence advances
nothing by itself. A fixed-sample clock may advance only through its exact
epoch, period, unit, and position mapping. Formula interval bounds remain
integer position offsets under that mapping; they never become elapsed-time
durations. Dense or timestamped events, implicit resampling, rounding, clock
drift correction, and wall-clock inference are unsupported.

Physical end-of-stream closure after the anchor cannot change the anchored
result. Late data at or before an evaluated anchor creates a new history
revision/digest and superseding result identity; it never edits the earlier
result. A history declared complete through an anchor cannot simultaneously
claim an unresolved gap there.

Capture and typed-predicate evaluation remain outside TL. A native Quire bridge
resolves captures, anchors, clocks, and total Boolean propositions before
constructing the history; absent capture or predicate data is unsupported at
that bridge and cannot become a false proposition or fabricated history row.

## Acceptance Criteria

| ID | Criteria | Verification |
|---|---|---|
| FR-012-AC-1 | Every accepted history is nonempty, gap-free, duplicate-free, ordered from zero through its declared end, digest/revision-consistent, and contains the anchor; each violated dimension returns a distinct refusal. | Test (TC-051, TC-053) |
| FR-012-AC-2 | Required-history analysis follows the exact recursive equations, uses checked u64 arithmetic, and reports the same value for structurally equal formulas independent of source spans. | Test (TC-052) |
| FR-012-AC-3 | A result binds formula/profile, history revision/digest, anchor, clock mapping, proposition map, evaluator revision, and limits; mutating any dimension invalidates attribution even when the Boolean stays equal. | Test (TC-051, TC-056) |
| FR-012-AC-4 | Later positions and late data create new identities, closure after an anchor does not change its result, silence does not synthesize a position, and no pure-past result is Pending. | Test (TC-052, TC-056) |
| FR-012-AC-5 | Event-position and exactly mapped fixed-sample histories are accepted with bounds interpreted only as position offsets; timestamped/dense time, duration reinterpretation, rounding, implicit resampling, missing captures, and partial/error-valued predicates are unsupported without Boolean coercion. | Test (TC-053, TC-056) |

## Dependencies

FR-003 supplies source/profile identity and FR-011 supplies operator semantics.
The native clock/capture/predicate correspondence remains owned by
quire-contract-ir #64 and its prerequisites.
