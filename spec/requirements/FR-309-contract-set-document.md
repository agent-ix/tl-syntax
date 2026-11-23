---
id: FR-309
title: Carry assume-guarantee contracts in a versioned document
type: FR
relationships:
  - target: ix://agent-ix/tl-syntax/StR-016
    type: implements
  - target: ix://agent-ix/tl-syntax/ADR-017
    type: depends_on
  - target: ix://agent-ix/tl-syntax/FR-021
    type: depends_on
---

# FR-309: Carry assume-guarantee contracts in a versioned document

## Description

When a caller declares contracts, tl-syntax shall construct a
`tl-syntax.contract-set/v1` document that names assumption and guarantee roots in
one validated formula graph, with a strict reader, without changing the graph.

## Inputs

- A validated formula (any existing edition), its content identity, and an
  ordered list of contracts `{ id: u32, name: string, assumption: NodeId,
  guarantee: NodeId }`.

## Outputs

- An owned `ContractSetDocument`; typed `ContractSetError`.

## Behavior

- Pattern: the same as `tl-syntax.fairness-premises/v1` (the document is derived
  interchange with no FSL source-context fields, per ADR-014 decision 6): it carries
  `graph_identity` (the `CONTENT_IDENTITY_DOMAIN_V1` digest of the canonical
  formula bytes), the semantic profile and, where the edition has one, the clock.
- Contract ids are strictly increasing; names are 1..=255 UTF-8 bytes and unique
  by exact bytes; at most 10,000 contracts.
- Both roots shall exist in the graph (`RootOutOfRange`). They need not be
  distinct: `assumption == guarantee` is legal and means the contract is verified
  exactly when the assumption holds.
- A mismatched graph identity, profile or clock is refused
  (`GraphIdentityMismatch`, `ProfileMismatch`, `ClockMismatch`).
- Roots are references; the same node may serve several contracts.
- Wire rules follow FR-303 (strict reader, embedded schema bytes and identity
  string, no new digest constant). The `graph_identity` above is the one retained
  content digest of this document, justified in the ADR-019 register.

## Acceptance Criteria

| ID | Criteria | Verification |
|---|---|---|
| FR-309-AC-1 | An empty set and a set with shared roots round-trip byte-identically with exact graph, profile and clock identities, over the v1, v2 and unbounded editions. | integration-testing (TC-1522) |
| FR-309-AC-2 | Out-of-range roots, duplicate or non-increasing ids, duplicate or empty or oversized names, over-limit populations and identity mismatches each yield a distinct error and no document. | unit-testing (TC-1523) |
| FR-309-AC-3 | The formula graph and schema digests are unchanged by the presence of a contract set. | inspection (TC-1524) |

## Dependencies

[FR-021](./FR-021-fairness-premise-document.md) is the binding precedent.
