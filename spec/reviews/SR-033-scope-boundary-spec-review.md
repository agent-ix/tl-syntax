---
id: SR-033
title: "Scope-boundary review of the complete tl-syntax specification corpus"
type: SpecReview
analysis: scope-boundary
scope: "spec/"
review_set: all
---

## Summary

tl-syntax owns syntax values, serialization, corpus conformance, and the local
producer boundary. Engineering Assurance classifies compatibility, Quire
exports static facts, Quoin retains declared inputs, and human reviewers retain
release authority; none is a local test runner.

## Findings

| ID | Severity | Summary | Refs | Escape Cause |
| --- | --- | --- | --- | --- |
| FND-3301 | medium | The authoritative boundary for top-level declaration metadata and subject scope is intentionally unresolved until the shared source-grounded export is released and adopted; current local prose must not be treated as verified scope. | tl-syntax#16, FR-006, AA-001 | correct-requirement-no-evidence |
