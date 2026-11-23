---
id: ADR-015
title: "Evaluate numerics exactly and export to C2PO only under a proven float-matching rule"
type: ADR
status: proposed
owner: tl-syntax-maintainer
relationships:
  - target: ix://agent-ix/tl-syntax/ADR-014
    type: depends_on
  - target: ix://agent-ix/tl-syntax/FR-302
    type: realizes_requirement
  - target: ix://agent-ix/tl-mltl/FR-056
    type: realizes_requirement
  - target: ix://agent-ix/tl-mltl/FR-057
    type: realizes_requirement
---

# ADR-015: Evaluate numerics exactly and export to C2PO only under a proven float-matching rule

## Status

Proposed (TL-419). Ready for human acceptance. The numeric target profile
values it needs are DEFERRED to a retained R2U2/C2PO observation (see Open
points). Every statement below about how R2U2 or C2PO behave is UNGROUNDED: the
retained corpus (`corpus/r2u2-v4.2` in tl-mltl) contains only Boolean `INPUT`
declarations and `FTSPEC`/`PTSPEC` sections, and no numeric type, tolerance, `abs`,
contract or aggregation construct. The rule is a design to be validated against a
future observation, not a description of the target.

## Context

Integer and fixed-Decimal signals are exact: a Decimal value is a signed `i64`
coefficient times 10 to the negative scale (scale 0 through 18). R2U2's C2PO
language computes comparisons in the monitor's own numeric types, including
floating point with tolerance. Decimal fractions such as 0.1 are not exactly
representable in binary floating point, so naive export can flip a verdict at a
boundary ("x > 0.3" with x exactly 0.3). A silent flip would make the R2U2
differential disagree for the wrong reason, or worse agree by luck.

## Decision

1. **Exact semantics are the definition.** Evaluation is on `i64` coefficients
   with `i128` intermediates, at identical scale, with the static range check
   of ADR-014 guaranteeing no overflow. Comparison is integer comparison of
   coefficients. No float exists in any tl-* evaluation path.
2. **Export is admitted only when the target cannot change any verdict.** Let
   the *target numeric profile* be a recorded tuple
   `(integer_bits, float_bits, tolerance_model, tolerance)` pinned from a retained
   R2U2 observation (UNGROUNDED until then). `tolerance_model` is `none`,
   `equality_only` or `all_comparisons`; an unpinned or unknown model is treated as
   `all_comparisons`, the worst case. For a predicate, let `g = 10^-scale` be the
   grid spacing shared by every value and constant in it and `u` the unit roundoff
   of `float_bits` (`2^-24` binary32, `2^-53` binary64; assumes round-to-nearest-even,
   each operation rounded once at `float_bits`, no extended precision and no fused
   multiply-add: UNGROUNDED). `R` is the exact-rational forward error bound of the
   exported expression over the declared domain, the sum of
   - **input conversion**: each signal read and each literal is a decimal fraction
     that is generally not representable; it contributes `u * |v|max`, where `v` is
     its largest magnitude (the observed values are converted by the target, also
     UNGROUNDED);
   - **operation rounding**: each `add`, `sub`, `mul` and `rescale` contributes
     `u * |result|max`, with errors propagated by standard forward analysis (`add` and
     `sub`: sum of operand errors; `mul`: `|a|max * err_b + |b|max * err_a + err_a *
     err_b`; `neg` and `abs`: operand error unchanged; `rescale` multiplies by the
     literal `10^k`, itself converted);
   - **threshold conversion**: the rendered midpoint literal `c +- g/2` is also not
     generally representable and contributes `u * |c +- g/2|max`.
   `T` is the profile tolerance when `tolerance_model` is `all_comparisons` (the
   tolerance may move any exported comparison, including the strict inequalities
   below) and when it is unknown; it is zero for `none`, and for `equality_only`
   because the export never emits a float `==` or `!=` (see the pair rendering
   below). The predicate is admitted iff
   - every domain bound and literal lies within `integer_bits` (Integer) or is
     representable on the grid within the target range (Decimal), and
   - `R + T < g / 2`.
   Admitted comparisons are exported with midpoint thresholds so that equality
   points do not depend on rounding: `x < c` is `x < c - g/2`, `x <= c` is
   `x < c + g/2`, `x > c` is `x > c + g/2`, `x >= c` is `x > c - g/2`,
   `x == c` is the pair `x > c - g/2 && x < c + g/2`, and `x != c` its negation.
   A predicate that fails the test is refused with a typed loss record that
   names the failed clause and the numbers; it is never exported approximately.
3. **No pinned profile means no numeric export.** Until the target profile is
   pinned from retained evidence, every numeric predicate is refused with the
   typed loss `TargetNumericProfileUnpinned`. Pure Boolean export is unchanged.
4. **Unsupported constructs are refused with a typed loss.** Constructs C2PO
   cannot express exactly (division is not in the grammar; `rescale` beyond the
   target type; multiplication whose range exceeds `integer_bits`; signals
   whose catalog bounds exceed the target type) yield a `NumericLossRecord`
   (FR-056 in tl-mltl) and no expression and no manifest.

## Alternatives rejected

- Export decimals as float literals and compare directly: rejected; verdict
  flips at grid points are the failure this ADR exists to prevent.
- Treat the target's tolerance as the semantics: rejected; it would make
  results depend on an unpinned constant and break reproducibility.
- Ignore input and threshold conversion error and count only operation rounding:
  rejected; a bound that omits a term cannot prove that no verdict changes.
- Scale everything to integers inside C2PO: viable and strictly safer, but it
  moves the scale into every exported expression; kept as the documented
  fallback if the midpoint rule cannot be satisfied on a real corpus.

## Consequences

Numeric export is conservative: some realistic Decimal domains (large
magnitude, high scale) will be refused. Refusal is evidence-bearing and
countable, as the past-profile export already is (`corpus/README.md`: 42 of
1833 admitted; refusals never count as agreement).

## Open points

- The numeric target profile (integer width, float width, tolerance model and
  value, integer-division and modulo behaviour, whether `abs` exists) is not
  grounded in this repository: no R2U2 or C2PO source is retained here and nothing
  executes either. DEFERRED to the retained exchange of TL-423, which no specified
  work produces (see the matrix "Open and blocked acceptance criteria").
- UNGROUNDED, each to be settled only by that exchange: the float type of the
  target; its rounding mode and use of extended precision or fused operations; how
  it converts decimal literals and trace values to float; whether and how a
  tolerance applies to `<`, `<=`, `>` and `>=` and not only to `==`; whether `abs`
  and typed `INPUT` declarations exist at all.
- What the specified work can verify is the threshold algebra and the admission
  test in exact rational arithmetic. Whether the target's floating-point evaluation
  of an exported form really agrees with the exact semantics is not verifiable by
  any tl-* crate (they use no float) and is established, if at all, only by the
  retained R2U2 differential, which does not exist.
