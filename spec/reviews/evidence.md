---
id: SR-031
title: "Evidence-method review of the complete tl-syntax specification corpus"
type: SpecReview
analysis: evidence
scope: "spec/"
review_set: all
---

## Summary

Quoin's catalog-derived advisor was run for every obligation. It found four
advisory mismatches between authored verification methods and recommendations;
they require an owner decision, not an automatic rewrite of the specification.

## Findings

| ID | Severity | Summary | Refs | Escape Cause |
| --- | --- | --- | --- | --- |
| FND-3101 | low | Four quantified NFR measurements are authored as Inspection or compile-time checking while the catalog recommends performance benchmarking; confirm the existing methods are intentional or revise them with corresponding evidence. | NFR-001-M-1, NFR-001-M-2, NFR-002-M-2, NFR-003-M-6 | correct-requirement-no-evidence |
