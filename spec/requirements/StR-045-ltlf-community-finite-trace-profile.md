---
id: StR-045
title: LTLf users need a textbook finite-trace profile that cannot be confused with existing profiles
type: StR
relationships:
  - target: ix://agent-ix/tl-syntax/FR-332
    type: satisfied_by
  - target: ix://agent-ix/tl-syntax/FR-333
    type: satisfied_by
  - target: ix://agent-ix/tl-syntax/FR-334
    type: satisfied_by
  - target: ix://agent-ix/tl-syntax/FR-335
    type: satisfied_by
---

# StR-045: LTLf users need a textbook finite-trace profile that cannot be confused with existing profiles

## Stakeholder Need

Temporal-logic researchers and regulated-requirements authors who work with LTLf
need to state unbounded `G`, `F`, `U`, `R` and both next operators over finite
traces with the end-of-trace behavior defined in the literature, and need
existing MLTL documents to keep exactly the meaning they have today.

## Rationale

Plain LTL is already expressible as `G[0,) p`. LTLf is not: the closed-trace
profile pads with false and has no unbounded interval. Without a distinct,
explicit profile a formula authored for one reading could be evaluated under
another and return a different verdict with no diagnostic. R2U2 and recent
FRET-to-R2U2 work define bare next differently from De Giacomo and Vardi, so
the need includes refusing an implicit choice.

## Validation Criteria

| ID | Criteria | Validation |
|---|---|---|
| StR-045-VC-1 | An LTLf formula is accepted only under its own profile identity, and every existing profile, edition and byte stream is unchanged. | Test (TC-221, TC-226) |
| StR-045-VC-2 | Strong and weak next are both writable and neither is a default. | Test (TC-223, TC-224) |

## Stakeholders

LTLf and FRET users, tl-parse, tl-mltl, tl-solve and release-gate maintainers.

## Context and Assumptions

Sources: De Giacomo and Vardi, IJCAI 2013; De Giacomo, De Masellis and Montali,
AAAI 2014; Aurandt et al., NFM 2026. Assumption: LTLf traces are nonempty
(recorded in [ADR-044](../decisions/ADR-044-ltlf-finite-trace-profile.md)).

## Traceability

Realized by FR-332 through FR-335; constrained by NFR-048.
