---
id: FR-011
title: "Define bounded past-time operator semantics"
type: FR
relationships:
  - target: ix://agent-ix/tl-syntax/MRS-003
    type: implements
  - target: ix://agent-ix/tl-syntax/FR-001
    type: depends_on
  - target: ix://agent-ix/tl-syntax/FR-002
    type: depends_on
---

# FR-011: Define bounded past-time operator semantics

## Description

When a validated past-profile formula is evaluated at an anchored discrete
position, the evaluator shall apply the closed v1 past operator vocabulary and
its exact inclusive-offset satisfaction relation.

## Inputs

- Operator profile `tl-syntax.past-operators/v1` and a formula tagged
  `mltl.origin-complete-history/v1`.
- A nonempty, gap-free Boolean proposition history with origin position zero
  and a validated anchor `t` in that history.
- Checked inclusive intervals `[a,b]` with `0 <= a <= b <= u32::MAX`.

## Outputs

- A Boolean result at anchor `t`, or a typed refusal owned by FR-012.
- No pending future-time result and no mutation of an earlier result.

## Behavior

The closed `tl-syntax.past-operators/v1` vocabulary contains the existing atoms
and Boolean operators plus:

| Node | Meaning at anchor `t` |
|---|---|
| `Once[a,b] p` / `O[a,b] p` | true iff some offset `j` in `[a,b]` makes `p` true at `t-j` |
| `Historically[a,b] p` / `H[a,b] p` | true iff every offset `j` in `[a,b]` makes `p` true at `t-j` |
| `p Since[a,b] q` / `p S[a,b] q` | true iff some witness offset `j` in `[a,b]` makes `q` true at `t-j` and `p` is true at every offset `k` in `[a,j)` |
| `p Triggered[a,b] q` / `p T[a,b] q` | exactly the Boolean dual `not((not p) S[a,b] (not q))` |

Offsets before `a` are irrelevant to Since, matching the future U lower-bound
convention in reverse. Triggered is a primitive canonical node whose semantics
must equal its stated dual; it is not an independent alternative truth table.

For a negative absolute position before origin, every proposition is false;
constants retain their Boolean values and Boolean/past nodes evaluate
recursively from those values. Subtraction uses checked offset comparison, not
unsigned wraparound. This is pre-origin false extension, not missing history.

The v1 profile refuses strong and weak Previous. As with future Next, the
canonical proposition-false extension does not encode physical-position
existence for the complete Boolean formula domain. It also refuses every future
operator and every graph containing both future and past nodes.

## Acceptance Criteria

| ID | Criteria | Verification |
|---|---|---|
| FR-011-AC-1 | For all valid histories, anchors, intervals, propositions, constants, and Boolean nests, O and H equal the stated existential/universal reverse-offset definitions including `[0,0]`, singleton history, and pre-origin extension. | Test (TC-048, TC-052) |
| FR-011-AC-2 | For all valid inputs, S uses a witness in `[a,b]`, requires its left operand only at reverse offsets `[a,j)`, ignores offsets before `a`, and handles witnesses at both inclusive endpoints. | Test (TC-049, TC-052) |
| FR-011-AC-3 | For all valid inputs, T is structurally distinct but evaluates exactly as the Boolean dual of S under every boundary and pre-origin case. | Test (TC-050, TC-052) |
| FR-011-AC-4 | Previous, every future or mixed-time node, every unknown past operator, and every unknown operator-profile identity are rejected before evaluation under the past profile. | Test (TC-053) |

## Dependencies

FR-001 supplies intervals and FR-002 supplies the common topological graph
discipline. FR-012 supplies valid histories/anchors; FR-013 supplies the new
profile and wire compatibility contract.
