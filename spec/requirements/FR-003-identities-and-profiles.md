---
id: FR-003
title: Preserve source, proposition, and semantic-profile identities
type: FR
relationships:
  - target: ix://agent-ix/tl-syntax/StR-002
    type: implements
---

# FR-003: Preserve source, proposition, and semantic-profile identities

## Description

When syntax crosses a component boundary, the library shall preserve stable
proposition identities, optional half-open source spans, and an explicit closed-
trace or online-prefix semantic-profile identity without parser-specific state.

## Inputs

- Unsigned proposition identities, optional byte offsets, and a supported
  semantic-profile identity.

## Outputs

- Ordered identity values and profile-tagged documents.

## Behavior

- A source span shall reject an end offset smaller than its start offset.
- Closed Trace v1 shall identify Boolean evaluation over a complete finite trace.
- Online Prefix v1 shall identify pending-until-decidable prefix semantics.

## Acceptance Criteria

| ID | Criteria | Verification |
|---|---|---|
| FR-003-AC-1 | Proposition identities and valid source spans compare and order deterministically. | Test (TC-006) |
| FR-003-AC-2 | Both v1 semantic profiles have distinct stable wire names. | Test (TC-007) |
| FR-003-AC-3 | Every serialized formula document requires a semantic profile. | Test (TC-008) |

## Dependencies

The versioned documents in [FR-004](./FR-004-versioned-serialization.md) use
these identities.
