---
id: FR-333
title: "Represent weak next and strong next as distinct explicit nodes with no default"
type: FR
relationships:
  - target: ix://agent-ix/tl-syntax/StR-045
    type: implements
  - target: ix://agent-ix/tl-syntax/ADR-045
    type: depends_on
  - target: ix://agent-ix/tl-syntax/FR-332
    type: depends_on
---

# FR-333: Represent weak next and strong next as distinct explicit nodes with no default

## Description

When a formula in `tl-syntax.formula-ltlf/v1` needs a next operator,
tl-syntax shall provide `StrongNext` (`Xs`) and `WeakNext` (`Xw`) as separate
primitive nodes with no bare next node, constructor, alias or default.

## Behavior

`StrongNext` and `WeakNext` are unary, carry no interval, have distinct
canonical node identities and distinct wire tags in the existing snake_case tag convention (`strong_next`,
`weak_next`), and
are admitted only in the LTLf edition. The wire decoder refuses `next`, `Next`, `X`, the CamelCase spellings
`StrongNext` and `WeakNext`, and any unknown tag. Their meaning (strong false at the last position, weak true at
the last position) is stated in ADR-044 and evaluated in tl-mltl FR-083.
`Xw p` and `not Xs not p` remain structurally distinct graphs; their equality of
meaning is an evaluator law (tl-mltl FR-083-AC-3), never a rewrite here.

## Acceptance Criteria

| ID | Criteria | Verification |
|---|---|---|
| FR-333-AC-1 | `Xs p` and `Xw p` over the same operand yield different canonical graph identities, and both round-trip through the wire form with tags `strong_next` and `weak_next`. | Test (TC-223) |
| FR-333-AC-2 | A wire document naming `Next`, `X`, or an unknown next tag is refused; no constructor accepts a missing strength. | Test (TC-224) |
| FR-333-AC-3 | `StrongNext`, `WeakNext` and the four LTLf temporal kinds are refused under every existing edition and profile. | Test (TC-225) |

## Dependencies

FR-332 for the edition and profile.
