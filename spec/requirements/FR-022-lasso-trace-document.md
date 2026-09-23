---
id: FR-022
title: "Publish a versioned lasso-trace input"
type: FR
relationships:
  - target: ix://agent-ix/tl-syntax/FR-020
    type: depends_on
  - target: ix://agent-ix/tl-syntax/FR-023
    type: depends_on
---

# FR-022: Publish a versioned lasso-trace input

## Description

When an infinite trace is represented by a finite lasso, tl-syntax shall
encode it as `tl-syntax.lasso-trace/v1`, with a finite prefix `u` and a
nonempty loop `v` whose observations repeat forever.

## Inputs

- Ordered prefix and loop observations over a declared proposition map.
- `mltl.infinite-trace/v1` and one exact clock binding.

## Outputs

- A validated lasso carrying schema, profile, proposition-map, and clock
identities, or a typed refusal.

## Behavior

Positions are contiguous from zero across the materialized `u·v` and wrap to
the first loop position after the last loop position. The loop is nonempty;
the prefix may be empty. Each observation uses FR-023's valuation vocabulary.
The lasso document does not contain an expected verdict or evaluate a formula.
An empty loop, gap, duplicate/out-of-order position, unknown proposition,
unsupported clock, or identity mismatch refuses before construction.

## Acceptance Criteria

| ID | Criteria | Verification |
|---|---|---|
| FR-022-AC-1 | Empty-prefix and nonempty-prefix lassos with nonempty loops round-trip with their exact loop entry, proposition map, profile, clock, and partial valuations. | Test (TC-154) |
| FR-022-AC-2 | Empty loops, invalid positions, foreign propositions, and mismatched identities produce typed refusals without a partial lasso. | Test (TC-155) |
| FR-022-AC-3 | Repeating the materialized loop preserves each position's valuation and clock mapping without inventing an end-of-trace position. | Test (TC-156) |

## Dependencies

FR-020 supplies identity and clock rules; FR-023 supplies valuation states.
