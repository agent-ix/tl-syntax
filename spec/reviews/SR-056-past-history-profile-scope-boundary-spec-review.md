---
id: SR-056
title: "Scope-boundary review of the past/history profile"
type: SpecReview
analysis: scope-boundary
scope: "MRS-003, FR-011 through FR-013, ADR-002"
review_set: all
---

## Summary

**PASS after remediation.** TL owns internal canonical syntax and evaluation;
tl-parse owns only an internal dialect; native Quire remains the sole editable
formal-clause authority. Capture/predicate resolution and target mappings stay
in their named downstream boundaries.

## Findings

| ID | Severity | Summary | Refs |
|---|---|---|---|
| FND-5601 | high | Adding O/H/S/T text could create a second user-authored Quire language. Fixed: `tl-parse.clean-ascii/v3` and formula-v2 are internal interchange only and the native bridge does not parse them as source clauses. | MRS-003, FR-013 |
| FND-5602 | high | TL could accidentally own capture absence or partial predicate truth. Fixed: the native bridge must resolve captures, clocks, anchors, and total Booleans before TL history construction. | FR-012-AC-5 |
| FND-5603 | medium | External monitor acceptance could become a qualification or semantic-authority claim. Fixed with explicit output-only/unavailable/unsupported dispositions and no foreign-runtime dependency. | FR-013-AC-4 |
| FND-5604 | low | Mixed future/past progress and closure are outside the pure-past contract. Fixed by profile validation refusal and a separate future-v1 preservation rule. | FR-011-AC-4, FR-013-AC-1 |
