---
id: SR-1508
title: "criterion-strength review of TL-150 numeric specs (TL-150-spec-tl-syntax)"
type: SpecReview
analysis: criterion-strength
scope: "tl-syntax ADR-014..018, StR-015/016, FR-302..311, NFR-018/019, TM-017, PLAN-021; cross-checked against tl-parse, tl-mltl, tl-solve numeric specs"
review_set: all
---

## Summary

Whether each AC can fail. Manual pass; Jev service not used.

Method note: independent fresh-context review, no build tools run. Skill lenses were applied manually by the reviewer in one pass; the criterion-strength lens normally uses the external Jev service, which was not used. All specs are unbuilt and ADRs are proposed.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | FR-308-AC-1 can pass with a wrong claim: it checks the computed trace count equals a recorded number, not that hand-written forms are right; equivalence is only proved in tl-mltl on Boolean members at length 1, so temporal templates and numeric members are never compared. | FR-308; tl-mltl FR-054-AC-4 |
| FND-002 | medium | FR-202-AC-3 "No test can make any refusal produce a verdict" is an unfalsifiable universal; replace with an enumerated property. | tl-solve FR-202-AC-3 |
| FND-003 | low | TC-1533, StR VC demonstrations, FR-058-AC-4 and FR-311-AC-2 have no failing adverse case. | numeric-predicates-test-matrix.md |
