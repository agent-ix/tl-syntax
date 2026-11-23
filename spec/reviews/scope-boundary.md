---
id: SR-1503
title: "scope-boundary review of TL-150 numeric specs (TL-150-spec-tl-syntax)"
type: SpecReview
analysis: scope-boundary
scope: "tl-syntax ADR-014..018, StR-015/016, FR-302..311, NFR-018/019, TM-017, PLAN-021; cross-checked against tl-parse, tl-mltl, tl-solve numeric specs"
review_set: all
---

## Summary

Responsibility allocation across tl-syntax, tl-parse, tl-mltl, tl-solve and FSL.

Method note: independent fresh-context review, no build tools run. Skill lenses were applied manually by the reviewer in one pass; the criterion-strength lens normally uses the external Jev service, which was not used. All specs are unbuilt and ADRs are proposed.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | FSL versus tl-syntax boundary is moved without amending the owning requirement; see base FND-001. | StR-003:44; ADR-014 d6 |
| FND-002 | medium | FR-308 makes tl-syntax own the equivalence case and population, but the comparison executes in tl-mltl (FR-054-AC-4); tl-syntax AC-1 only checks count, so the equivalence claim is owned by no single repo's AC. | FR-308; tl-mltl FR-054-AC-4 |
| FND-003 | medium | tl-syntax owns the contract status truth table and obligation lowering but 149 needs a stronger interface (input-only assumption check, trace-level status); boundary with P-TL-149 is a human decision. | FR-311; ADR-017 |
| FND-004 | low | tl-oracle is the independent evaluator for FR-054 but evaluates Boolean traces only; numeric truth comes from a throwaway script retained in tl-syntax corpus provenance, so the "outside reference" is a script, not an outside tool. | tl-mltl FR-054; FR-305 |
