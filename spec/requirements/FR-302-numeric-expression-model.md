---
id: FR-302
title: Model exact numeric expressions with static typing
type: FR
relationships:
  - target: ix://agent-ix/tl-syntax/StR-015
    type: implements
  - target: ix://agent-ix/tl-syntax/FR-007
    type: depends_on
  - target: ix://agent-ix/tl-syntax/ADR-014
    type: depends_on
  - target: ix://agent-ix/tl-syntax/ADR-015
    type: depends_on
---

# FR-302: Model exact numeric expressions with static typing

## Description

When a numeric expression over catalog signals is constructed, tl-syntax shall
derive its exact type from the catalog's declared domains and shall refuse the
expression at construction if it cannot be evaluated exactly without overflow.

## Inputs

- A validated signal catalog (FR-007) and an expression arena: an ordered list
  of expression nodes in which every operand precedes its user.
- Expression node kinds: `signal(SignalId)`, `integer(i64)`,
  `decimal(coefficient i64, scale u8)`, `neg`, `abs`, `add`, `sub`, `mul`,
  `rescale(operand, scale u8)`.

## Outputs

- A borrowed validated expression view and, per node, a derived type
  `ExprType { scale: u8, minimum: i64, maximum: i64 }` where the bounds are the
  inclusive coefficient range. An Integer is scale 0.
- Under `alloc` and `serde`, the owned form used by FR-303.
- A typed `ExpressionError`, never a partial view.

## Typing rules

- `signal` takes the catalog domain: Integer gives `(0, min, max)`, FixedDecimal
  gives `(scale, minimum_coefficient, maximum_coefficient)`. A Boolean signal is
  `NonNumericSignal`; an absent signal is `UnknownSignal`.
- `integer(v)` is `(0, v, v)`; `decimal(c, s)` is `(s, c, c)` with `s <= 18`.
- `neg`: `(s, -hi, -lo)`. `abs`: `(s, lo', hi')` where `hi' = max(|lo|, |hi|)`
  and `lo' = 0` if `lo <= 0 <= hi` else `min(|lo|, |hi|)`.
- `add` and `sub` require equal scale (`ScaleMismatch` otherwise) and use
  interval sum or difference.
- `mul` yields scale `s1 + s2` (`ScaleOverflow` above 18) and the interval
  hull of the four corner products.
- `rescale(e, s')` requires `s' >= s` and `s' <= 18`; it multiplies the range by
  `10^(s'-s)`. Narrowing is `RescaleNarrowing`; there is no implicit rescale
  anywhere.
- Every bound is computed in `i128` and must fit `i64`; otherwise
  `RangeOverflow` names the node. Literals that do not fit are unrepresentable.
- Limits: at most 100,000 expression nodes per table and expression depth at
  most 256; both are typed refusals (`ExpressionLimitExceeded`).

## Constraints

| ID | Constraint | Type | Validation |
|---|---|---|---|
| FR-302-CON-1 | No floating-point type shall appear in the model, its wire form or its errors. | Design | Inspection |
| FR-302-CON-2 | Borrowed validation shall use caller-owned scratch of one `ExprType` slot per node without allocating. | Portability | Test |

## Acceptance Criteria

| ID | Criteria | Verification |
|---|---|---|
| FR-302-AC-1 | Every node kind types as specified, including `abs` over a range straddling zero and `mul` hulls with negative corners, and the derived type equals an independently computed interval on the numeric corpus. | property-based-testing (TC-1500, TC-1501) |
| FR-302-AC-2 | `RangeOverflow` at the first overflowing node, `ScaleMismatch`, `ScaleOverflow`, `RescaleNarrowing`, `NonNumericSignal`, `UnknownSignal` and `ExpressionLimitExceeded` are each produced by a distinct minimal input, in node order, with no view returned. | unit-testing (TC-1502) |
| FR-302-AC-3 | A validated expression evaluated by a test-only exact reference (tl-syntax exposes no evaluator; evaluation is owned by tl-mltl) at every corner of its inputs' declared domains never exceeds its derived range and never overflows `i64`. | property-based-testing (TC-1503) |

## Dependencies

[FR-007](./FR-007-typed-signal-context.md) supplies the catalog and domains.
