---
id: FR-023
title: "Preserve missing and conflicting proposition values"
type: FR
relationships:
  - target: ix://agent-ix/tl-syntax/FR-007
    type: depends_on
  - target: ix://agent-ix/tl-syntax/FR-020
    type: depends_on
---

# FR-023: Preserve missing and conflicting proposition values

## Description

When a trace position has incomplete or conflicting proposition evidence,
tl-syntax shall preserve each declared proposition's value as one of `true`,
`false`, `missing`, or `conflicting` in
`tl-syntax.partial-valuation/v1`.

## Inputs

- A proposition-map identity and one state per declared proposition at a
  position under `mltl.infinite-trace/v1`.

## Outputs

- A canonical, ordered partial valuation or a typed refusal before a trace
  document is constructed.

## Behavior

The four states are distinct on the wire and in the Rust API. `missing` means
no value was supplied; `conflicting` means incompatible values were supplied.
Neither is coerced to a Boolean value or to the other state. Every declared
proposition occurs once; unknown, duplicate, omitted, or unordered entries
refuse. FR-160 possibility-set evaluation belongs to the downstream provider,
not this syntax document.

## Acceptance Criteria

| ID | Criteria | Verification |
|---|---|---|
| FR-023-AC-1 | All four states round-trip distinctly and canonical ordering is stable across runs. | Test (TC-157) |
| FR-023-AC-2 | Unknown, duplicate, omitted, or unordered proposition entries refuse before a valuation or containing trace is produced. | Test (TC-158) |
| FR-023-AC-3 | Changing only `missing` to `conflicting` changes wire and semantic identity; no syntax path infers a Boolean verdict from either state. | Test (TC-159) |

## Dependencies

FR-007 owns proposition-map identity; FR-020 owns the profile binding.
