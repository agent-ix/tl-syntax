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

Extend the common canonical node graph with O/H/S/T only under new
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
- Previous as O[1,1] or H[1,1]: rejected because physical-position existence is
  absent and constants expose the boundary mismatch.
- Foreign-runtime differential qualification: rejected by the native-language
  ruling and because none has an accepted exact profile correspondence.

## Consequences

Downstream exhaustive Rust matches must add explicit past/profile handling, but
old wire bytes and semantics do not change. A new evaluator path is legitimate
past semantics inside the same architecture; derived rewrites may not replace
it with future nodes. Native clocks/captures/predicates are resolved before TL.

This decision authorizes specification and routed planning only. It does not
authorize implementation, native Quire grammar, production monitoring,
qualification, or release.
