---
id: SR-061
title: "Dependency review of progressive tl-syntax source readiness"
type: SpecReview
analysis: dependency
scope: "MRS-004, FR-014..FR-018, NFR-004..NFR-005, AD-002, IT-001..IT-002"
review_set: all
---

## Summary

The M6 dependency graph was checked for ownership and cycles at specification
commit `767dc92a97f1b3d9ffbb76467bdda9ecf2261e40`. Local work orders executable-
path classification before source binding, stage preservation and human
decision admission; package publication follows all of them. External
enablement remains separate from feature implementation.

## Findings

| ID | Severity | Summary | Refs | Escape Cause |
| --- | --- | --- | --- | --- |
| FND-6101 | medium | IT-001 remains blocked until tl-syntax#16 has a compatible released Engineering Assurance matrix accepting the source-grounded Quire release; branch pins and repository-local mappings are inadmissible. | tl-syntax#16, FR-014, IT-001 | correct-requirement-no-evidence |
| FND-6102 | medium | Decision admission, durable retention and integrator packaging remain blocked on an immutable reviewer/decision policy, a selected retention backend/operator, and accepted shared contracts from Engineering Assurance and Quoin. | FR-015, FR-016, FR-017, engineering-assurance#11, engineering-assurance#34 | correct-requirement-no-evidence |
