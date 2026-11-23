---
id: ADR-044
title: "Add LTLf as a separate unbounded finite-trace profile with explicit end-of-trace rules"
type: ADR
status: proposed
owner: tl-syntax-maintainer
relationships:
  - target: ix://agent-ix/tl-syntax/StR-045
    type: realizes_requirement
  - target: ix://agent-ix/tl-syntax/FR-332
    type: realizes_requirement
  - target: ix://agent-ix/tl-syntax/FR-334
    type: realizes_requirement
  - target: ix://agent-ix/tl-syntax/ADR-002
    type: relates_to
---

# ADR-044: Add LTLf as a separate unbounded finite-trace profile with explicit end-of-trace rules

## Status

Proposed (TL-427, project P-TL-151). Ready for human acceptance; not accepted.

## Context

Plain LTL over infinite traces is already representable as `G[0,) p` under
`mltl.infinite-trace/v1`. LTLf, as defined by De Giacomo and Vardi (IJCAI 2013)
and used by the LTLf community, interprets unbounded operators over a nonempty
finite trace and has no padding. The existing `mltl.closed-trace/v1` profile
evaluates a bounded formula over a complete trace and treats every position
after the trace end as a position at which every proposition is false. That
padding is a deliberate tl-syntax choice and is not textbook LTLf: under
padding `G[0,2] p` is false on the two-position trace `p, p`, while the LTLf
reading of "always p until the trace ends" is true.

Reusing either existing profile identity for LTLf would silently reinterpret
documents already exchanged. This is the same hazard ADR-002 resolved for past
time.

## Decision

1. Add exactly one new semantic profile identity, `mltl.ltlf-finite-trace/v1`
   (Rust variant `SemanticProfile::LtlfFiniteTraceV1`), a new operator catalog
   `tl-syntax.ltlf-operators/v1`, and a new sibling formula edition
   `tl-syntax.formula-ltlf/v1`. The four existing profiles, three existing
   editions and their wire bytes are unchanged.
2. LTLf traces are **nonempty**. Positions are `0..n-1`, `n >= 1`. An empty
   trace is a typed refusal owned by the evaluator, never `false`.
3. End-of-trace rules (the evaluator contract; `i` is the current position,
   `last` is `n-1`):
   - atoms and constants are read at `i`; Boolean operators are pointwise;
   - `Xs p` (strong next) is true iff `i < last` and `p` holds at `i+1`;
   - `Xw p` (weak next) is true iff `i = last` or `p` holds at `i+1`;
   - `F p` is true iff some `j` in `[i, last]` satisfies `p`;
   - `G p` is true iff every `j` in `[i, last]` satisfies `p`;
   - `p U q` is true iff some `j` in `[i, last]` satisfies `q` and every `k` in
     `[i, j)` satisfies `p`;
   - `p R q` is the Boolean dual `not(not p U not q)`: true iff every `j` in
     `[i, last]` satisfies `q` or some `k` in `[i, j)` satisfies `p`.
   A formula is true on a trace iff true at position 0. There is no padding.
4. The v1 vocabulary is constants, propositions, Boolean operators, `F`, `G`,
   `U`, `R`, `Xs`, `Xw`. Temporal nodes carry **no interval**. Bounded
   intervals, `[a,)` with `a > 0`, past operators, `Y`, and the derived `W`/`M`
   are refused in v1, so that no bounded spelling can mean padding in one
   profile and truncation in another.
5. **Relation to closed-trace.** The profiles are unrelated identities with no
   implicit conversion. They agree on no admitted LTLf formula by construction
   (closed-trace admits only bounded intervals; LTLf admits none). Any
   translation from LTLf to a bounded profile is the mission-bound closure
   of tl-mltl ADR-045 and is never implicit.

## Alternatives rejected

- Redefine closed-trace padding as truncation: changes the meaning of existing
  documents under an existing identity.
- Admit bounded intervals under LTLf (truncating `G[a,b]`): two meanings for one
  spelling across profiles; deferred to a later profile version if a user needs it.
- Reuse `tl-syntax.formula-unbounded/v1` with a profile restriction: that
  edition requires `mltl.infinite-trace/v1` (FR-289) and carries an interval
  field that LTLf must not have.
- Allow the empty trace as vacuously true/false: LTLf in the cited literature is
  over nonempty traces; either default would be a silent choice.

## Consequences

`SemanticProfile::ALL` grows from four to five and downstream exhaustive
matches must name the new variant (source-breaking in Rust, wire-compatible).
Release-gate corpus decoders and the cross-crate corpus gain one edition.
tl-mltl owns evaluation (tl-mltl FR-083); tl-parse owns text (tl-parse FR-058).
