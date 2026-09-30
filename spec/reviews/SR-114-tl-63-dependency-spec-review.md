---
id: SR-114
title: TL-63 source-binding dependency specification review
type: SpecReview
analysis: dependency
scope: "FR-006-AC-9, FR-015, PLAN-007 Task-018"
review_set: all
---

## Summary

The source path projection consumes Quire's released assurance export and
Quoin's released change-assurance record. It does not replace TL-24 admission,
TL-23 executable-path parity or TL-22 fresh producer and candidate binding.

## Findings

| ID | Severity | Summary | Refs | Escape Cause |
| --- | --- | --- | --- | --- |
| FND-8004 | medium | A projected record cannot promote dependent work while retention, decision authority, package release and LR08 dispositions remain unselected. | PLAN-007, Task-018 | correct-requirement-no-evidence |
