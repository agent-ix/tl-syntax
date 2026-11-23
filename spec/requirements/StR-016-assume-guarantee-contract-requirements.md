---
id: StR-016
title: Assessors need assume-guarantee contracts with distinct inactive, invalid and verified results
type: StR
relationships:
  - target: ix://agent-ix/tl-syntax/FR-309
    type: satisfied_by
  - target: ix://agent-ix/tl-syntax/FR-310
    type: satisfied_by
  - target: ix://agent-ix/tl-syntax/FR-311
    type: satisfied_by
---

# StR-016: Assessors need assume-guarantee contracts with distinct inactive, invalid and verified results

## Stakeholder Need

Certification assessors require that a contract (an assumption about the
environment paired with a guarantee from the system) shall report whether it was
inactive, invalid or verified, so that a never-exercised contract is not
reported as a satisfied one.

## Rationale

Certification evidence is usually structured as assume-guarantee contracts, and
R2U2 and the Lunar Gateway work already use them. A plain implication is true
when the assumption never held, which hides exactly the case an assessor must
see.

## Validation Criteria

| ID | Criteria | Validation |
|---|---|---|
| StR-016-VC-1 | For a contract whose assumption fails, whose assumption holds and guarantee fails, and whose assumption and guarantee both hold, a reviewer sees inactive, invalid and verified respectively. | Demonstration |
| StR-016-VC-2 | The realizability project can reference a contract's assumption and guarantee without this crate deciding what an environment assumption is. | Inspection |

## Stakeholders

Assessors, R2U2/C2PO users, the realizability project (P-TL-149).

## Context and Assumptions

Status is per evaluation position. The inter-project decision about
environment assumptions is recorded in ADR-017 and FR-311 and is not made here.

## Dependencies

FR-309 through FR-311.
