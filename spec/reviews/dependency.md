---
id: SR-1504
title: "dependency review of TL-150 numeric specs (TL-150-spec-tl-syntax)"
type: SpecReview
analysis: dependency
scope: "tl-syntax ADR-014..018, StR-015/016, FR-302..311, NFR-018/019, TM-017, PLAN-021; cross-checked against tl-parse, tl-mltl, tl-solve numeric specs"
review_set: all
---

## Summary

Ordering and enablement across tickets and repos.

Method note: independent fresh-context review, no build tools run. Skill lenses were applied manually by the reviewer in one pass; the criterion-strength lens normally uses the external Jev service, which was not used. All specs are unbuilt and ADRs are proposed.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | Build order is constrained but not stated in one place: TL-420 (tl-syntax) before TL-421/422/424/425; TL-426 needs tl-mltl numeric trace reader (TL-422) for witness re-check; TL-423 and TL-425 differential ACs need an R2U2 run nobody has scheduled; FR-311-AC-2 needs Peter. Plans per repo exist but cross-repo gating is only in the matrix. | PLAN-021; tl-solve PLAN-200; FR-311-AC-2 |
| FND-002 | medium | TL-424 requires tl-syntax FR-307/308, tl-parse FR-032 and tl-mltl FR-054-AC-4 in three repos with a shared 567-trace claim; any one repo lagging leaves the AC unverifiable. | FR-308; tl-mltl FR-054-AC-4; tl-parse FR-032 |
| FND-003 | low | 149 depends on tl-syntax catalog v2 (FR-034) for input/output direction; 150 contract-set binds the v1 catalog only and does not mention v2. | tl-mltl-149 ADR-004 d4; FR-309 |
| FND-004 | low | No id collisions with sibling branches TL-147, TL-149, TL-151 (tl-syntax FR-332..335 and ADR-044/045 of 151; FR-034 of 149 are disjoint from FR-302..311, ADR-014..018). | sibling worktrees |
