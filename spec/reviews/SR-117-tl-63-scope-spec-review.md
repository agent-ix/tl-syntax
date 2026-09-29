---
id: SR-117
title: TL-63 source-binding scope-boundary specification review
type: SpecReview
analysis: scope-boundary
scope: "FR-006-AC-9, FR-015, PLAN-007"
review_set: all
---

## Summary

Quire owns parsed specification facts and source locators; Quoin owns the sealed
record. tl-syntax owns only declaration projection and domain-specific checks.
The candidate's complete change footprint and source-release authority remain
outside this TL-63 slice.

## Findings

| ID | Severity | Summary | Refs | Escape Cause |
| --- | --- | --- | --- | --- |
| FND-8007 | low | No local parser, retention store or approval source is authorized by the delta. | FR-006-AC-9, FR-015-AC-4 | correct-requirement-no-evidence |
