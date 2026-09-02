---
id: FR-002
title: Validate the complete bounded MLTL formula graph
type: FR
relationships:
  - target: ix://agent-ix/tl-syntax/FR-001
    type: depends_on
---

# FR-002: Validate the complete bounded MLTL formula graph

## Description

When a formula view is constructed, the library shall validate a root and node
table containing propositions, Boolean constants and operators, bounded Future,
Globally, Until, and Release nodes.

## Inputs

- A root node identity and a borrowed topologically ordered node table.

## Outputs

- A validated formula view or a structural validation error.

## Behavior

- The validator shall reject an absent root.
- The validator shall reject every operand that does not precede its owner.
- Temporal nodes shall accept only `Interval` values whose private bounds and
  checked deserialization reject inversion before formula validation.
- The public node, formula, and identity values shall implement deterministic
  equality and ordering.

## Acceptance Criteria

| ID | Criteria | Verification |
|---|---|---|
| FR-002-AC-1 | Valid primitive, Boolean, nested, and temporal formulas expose their root and nodes unchanged, including lookup of the root through `Formula::node`. | Test (TC-003) |
| FR-002-AC-2 | Missing roots, forward references, self references, and out-of-range references are rejected. | Test (TC-004) |
| FR-002-AC-3 | Formula values sort identically for repeated runs over the same inputs. | Test (TC-005) |

## Dependencies

Depends on [FR-001](./FR-001-inclusive-intervals.md). Serialization consumes
the validated view through [FR-004](./FR-004-versioned-serialization.md).
