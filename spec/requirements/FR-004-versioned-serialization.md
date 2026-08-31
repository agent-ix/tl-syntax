---
id: FR-004
title: Serialize and validate versioned syntax documents
type: FR
relationships:
  - target: ix://agent-ix/tl-syntax/FR-002
    type: depends_on
  - target: ix://agent-ix/tl-syntax/FR-003
    type: depends_on
---

# FR-004: Serialize and validate versioned syntax documents

## Description

Where the serde feature is enabled, the library shall encode and decode owned
formula and proposition-map documents whose schema and semantic profile are
explicit and versioned.

## Inputs

- Owned formula nodes or proposition-map entries and supported version values.

## Outputs

- Deterministically structured documents or explicit version, mapping, or
  formula-validation errors.

## Behavior

- Formula documents shall carry the formula schema version and semantic profile.
- Proposition-map documents shall carry the proposition-map schema version.
- Proposition maps shall reject duplicate or non-increasing identities plus
  empty or duplicate names.
- Unknown schema or semantic-profile strings shall fail deserialization.

## Acceptance Criteria

| ID | Criteria | Verification |
|---|---|---|
| FR-004-AC-1 | Supported formula and proposition-map documents round-trip without loss, and `FormulaDocument::from_formula` preserves the borrowed formula's profile, root, and nodes. | Test (TC-009, TC-017) |
| FR-004-AC-2 | Unknown schema and profile versions fail to deserialize. | Test (TC-010) |
| FR-004-AC-3 | Malformed formula graphs and proposition maps fail validation. | Test (TC-011) |

## Dependencies

Depends on [FR-002](./FR-002-validated-formula.md) and
[FR-003](./FR-003-identities-and-profiles.md).
