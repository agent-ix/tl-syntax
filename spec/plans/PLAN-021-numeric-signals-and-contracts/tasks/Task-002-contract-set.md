---
id: Task-002
title: "Contract set and status algebra (TL-425 syntax half)"
type: Task
status: todo
track: B
priority: P0
relationships:
  - target: ix://agent-ix/tl-syntax/FR-309
    type: references
  - target: ix://agent-ix/tl-syntax/FR-310
    type: references
  - target: ix://agent-ix/tl-syntax/FR-311
    type: references
  - target: ix://agent-ix/tl-syntax/TC-1522
    type: verifies
  - target: ix://agent-ix/tl-syntax/TC-1525
    type: verifies
---
# Task-002: Contract set and status algebra (TL-425 syntax half)

## Scope

FR-309, FR-310, FR-311: contract-set document, status truth table, obligation lowering, read-only ContractView. FR-311 stays provisional until the inter-project decision.

## Subtasks

- [ ] ContractSetDocument bound by graph identity.
- [ ] ContractStatus/ContractOutcome total function.
- [ ] lower_contract_obligation; ContractView.

## Deliverables

Document, function, tests TC-1522..TC-1528.

## Notes

Specification-only session: no code was written or run. Layer 1 of P-TL-150.
