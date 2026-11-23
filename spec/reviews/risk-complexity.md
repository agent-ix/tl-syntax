---
id: SR-1505
title: "risk-complexity review of TL-150 numeric specs (TL-150-spec-tl-syntax)"
type: SpecReview
analysis: risk-complexity
scope: "tl-syntax ADR-014..018, StR-015/016, FR-302..311, NFR-018/019, TM-017, PLAN-021; cross-checked against tl-parse, tl-mltl, tl-solve numeric specs"
review_set: all
---

## Summary

Technical risk and volatility.

Method note: independent fresh-context review, no build tools run. Skill lenses were applied manually by the reviewer in one pass; the criterion-strength lens normally uses the external Jev service, which was not used. All specs are unbuilt and ADRs are proposed.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | ADR-015 float-matching rule is the top risk and rests on unretained facts: the R bound counts one half-ulp per operation but not the decimal-to-float conversion of each observed value or of the midpoint literals c-g/2, and does not say how a tolerance applies to inequalities. A rule that proves "cannot change any verdict" with a missing error term is unsound. | ADR-015 d2; tl-mltl FR-057 |
| FND-002 | medium | Set aggregation bound counts nodes and predicates but not expression nodes created by per-member template instantiation, and nested quantifiers (tl-parse FR-032) are bounded by "product of member counts" with no tl-syntax counterpart (FR-306 has one quantifier). | FR-307:59; tl-parse FR-032 |
| FND-003 | medium | tl-solve array multipliers at width up to 64 are allowed by derived ranges while signal width cap is 32; combined with 1.3M-case N-C partition runtime is unmeasured. | tl-solve FR-201/FR-202/FR-204 |
| FND-004 | low | Rational i128 arithmetic for R+T maps overflow to ResourceExhausted; acceptable. | tl-mltl FR-057 |
