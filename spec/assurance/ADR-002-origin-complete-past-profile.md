---
id: ADR-002
title: "Add past time as an origin-complete separate profile"
type: ADR
status: accepted
owner: tl-syntax-maintainer
relationships:
  - target: ix://agent-ix/tl-syntax/MRS-003
    type: depends_on
  - target: ix://agent-ix/tl-syntax/FR-011
    type: depends_on
  - target: ix://agent-ix/tl-syntax/FR-012
    type: depends_on
  - target: ix://agent-ix/tl-syntax/FR-013
    type: depends_on
---

# ADR-002: Add past time as an origin-complete separate profile

## Context

Past operators need an evaluation anchor, a declared history origin, reverse
offsets, and history-completeness semantics that existing future closed/prefix
profiles do not carry. Reusing a current profile would silently reinterpret old
formulas; creating a second unrelated AST/evaluator architecture would split
the ecosystem.

## Decision

Extend the common canonical node graph with O/H/Y/S/T only under new
`tl-syntax.formula/v2` and `mltl.origin-complete-history/v1` identities. Keep
formula-v1 and both future profiles closed and unchanged. Reject mixed
future/past graphs in the first past profile.

Past evaluation is anchored in nonempty, gap-free history from origin zero.
Positions before origin use proposition-false extension. Results are final for
their exact anchor, while a later anchor or late data creates a new immutable
identity. Physical closure after the anchor is irrelevant. Required history is
reported as maximum reverse offset, not minimum physical trace length.

## Alternatives rejected

- Reusing a future semantic profile: rejected because anchor/history/progress
  meaning would change under an existing identity.
- A separate unrelated past AST: rejected because it would duplicate common
  Boolean graph validation, identity, and wire infrastructure.
- Mixed future/past v1: rejected until combined progress, lookback/lookahead,
  closure, and monitor correspondence are separately specified.
- Weak Previous as a dual of strong Previous: rejected because the authoritative
  native v1 contract specifies only strong Previous. Strong Previous is the
  primitive `Y` node and has exactly the `Once[1,1]` truth relation selected by
  `ix://agent-ix/quire-specification/FR-092`; no physical-position-existence
  condition or weak boundary value is inferred.
- Foreign-runtime differential qualification: rejected by the native-language
  ruling and because none has an accepted exact profile correspondence.

## Consequences

Downstream exhaustive Rust matches must add explicit past/profile handling, but
old wire bytes and semantics do not change. A new evaluator path is legitimate
past semantics inside the same architecture; derived rewrites may not replace
it with future nodes. Native clocks/captures/predicates are resolved before TL.

The owner-approved implementation epic `agent-ix/tl-syntax#52` authorizes the
routed Rust implementation after this profile lands on `main`. It does not
authorize native Quire grammar, production deployment, external qualification,
or release.
