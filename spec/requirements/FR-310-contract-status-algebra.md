---
id: FR-310
title: Define the contract status vocabulary and truth table once
type: FR
relationships:
  - target: ix://agent-ix/tl-syntax/StR-016
    type: implements
  - target: ix://agent-ix/tl-syntax/FR-309
    type: depends_on
---

# FR-310: Define the contract status vocabulary and truth table once

## Description

The tl-syntax crate shall define the contract status vocabulary and its truth
table as a total, allocation-free function so that the evaluator, the exporter
and the solver consume one definition.

## Behavior

`ContractStatus` is `Inactive | Invalid | Verified`; `ContractOutcome` is
`Status(ContractStatus) | Pending`. Given the assumption result `a` and the
guarantee result `g`, each `True | False | Pending`:

| a | g | outcome |
|---|---|---|
| False | any | Inactive |
| True | False | Invalid |
| True | True | Verified |
| True | Pending | Pending |
| Pending | any | Pending |

Stable wire spellings `inactive`, `invalid`, `verified`, `pending`.
The function is monotone in information: refining a `Pending` input never
changes a decided outcome.

Obligation lowering: `lower_contract_obligation` appends one `Implies` node
whose operands are the assumption and guarantee roots, returning its id, using
one node of the node budget, and refusing at the document node limit. The
obligation is `Implies(a, g)`; it is true exactly when the outcome is `Inactive`
or `Verified` and false exactly when `Invalid`.

## Acceptance Criteria

| ID | Criteria | Verification |
|---|---|---|
| FR-310-AC-1 | All nine combinations of `(a, g)` produce the tabulated outcome and spelling. | unit-testing (TC-1525) |
| FR-310-AC-2 | Monotonicity: for every pair of inputs where one refines the other, a decided outcome is preserved. | property-based-testing (TC-1526) |
| FR-310-AC-3 | Obligation lowering appends exactly one `Implies` node, preserves existing nodes and ids, and refuses at the node limit. | unit-testing (TC-1527) |

## Dependencies

FR-309 supplies the roots.
