---
id: SR-116
title: TL-63 source-binding risk and complexity specification review
type: SpecReview
analysis: risk-complexity
scope: "FR-006-AC-9, AP-002"
review_set: all
---

## Summary

The material risks are an unsealed path substitution, scope promotion and
false export authority. Each has a one-axis adverse case; the more complex
snapshot and race boundary remains allocated to FR-015.

## Findings

| ID | Severity | Summary | Refs | Escape Cause |
| --- | --- | --- | --- | --- |
| FND-8006 | low | Acceptance of caller-supplied JSON alone would not establish fresh source export; the record must keep that limitation explicit until TL-22. | FR-015-AC-5, AP-002 | correct-requirement-no-evidence |
