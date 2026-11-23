---
id: FR-311
title: Contract interface required by realizability
type: FR
relationships:
  - target: ix://agent-ix/tl-syntax/StR-016
    type: implements
  - target: ix://agent-ix/tl-syntax/ADR-017
    type: depends_on
  - target: ix://agent-ix/tl-syntax/FR-310
    type: depends_on
---

# FR-311: Contract interface required by realizability

## Description

PROVISIONAL, PENDING AN INTER-PROJECT DECISION (see ADR-017). tl-syntax shall
expose a read-only contract interface that P-TL-149 (realizability) can consume
without this project deciding whether environment assumptions are contracts.

## Behavior

- A borrowed `ContractView` offers, per contract, its id, name, assumption root
  and guarantee root, and the obligation lowering of FR-310, and nothing that
  states a scope for the assumption.
- The interface is deliberately minimal and additive. If P-TL-149 selects option
  A of ADR-017 (invariant assumptions), the change is a new
  `tl-syntax.contract-set/v2` carrying an explicit assumption scope; v1 remains
  per evaluation position and unchanged. If it selects B or C, nothing changes
  here.
- Until the decision is recorded, no document in this project claims that a
  contract is an environment assumption.

## Acceptance Criteria

| ID | Criteria | Verification |
|---|---|---|
| FR-311-AC-1 | `ContractView` exposes ids, names, roots and the obligation lowering and no scope or realizability field. | unit-testing (TC-1528) |
| FR-311-AC-2 | The decision (A, B or C of ADR-017) is recorded by a human in ADR-017 before any realizability document cites a contract as an assumption. | inspection (TC-1529) |

## Dependencies

FR-310. Decision owner: Peter, between P-TL-150 and P-TL-149.
