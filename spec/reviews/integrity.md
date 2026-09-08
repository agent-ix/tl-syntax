---
id: SR-029
title: "Integrity review of the complete tl-syntax specification corpus"
type: SpecReview
analysis: integrity
scope: "spec/"
review_set: all
---

## Summary

Requirement, matrix, and symbol traces were inspected alongside Quire's
coverage export. The corpus has explicit owners and verification methods, but
the shared traceability module still skips one status-column classifier because
its configured header does not match the authored matrix header.

## Findings

| ID | Severity | Summary | Refs | Escape Cause |
| --- | --- | --- | --- | --- |
| FND-2901 | low | The shared coverage status-column configuration expects `Status` while tl-syntax authors `Coverage Status`, so that advisory classifier cannot independently classify matrix completion. | spec/test-matrix.md, SR-003, quire-contract-ir#21 | correct-requirement-no-evidence |
