---
id: SR-015
title: "Base specification review — semantic formula identity"
type: SpecReview
analysis: base
scope: "FR-003, TM-001, SR-014"
review_set: base
---

# Base specification review — semantic formula identity

## Summary

The owner selected the base review set for the span-independent formula
identity amendment. The review checked identifier resolution, criterion-to-test
matrix linkage, EARS grammar, trace tag coverage, and scope containment. The
amendment specifies the distinction between diagnostic wire provenance and
semantic identity without changing the v1 diagnostic wire contract.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-1501 | low | No base-review defect found: FR-003-AC-4 resolves to the non-retired TC-027 row, the implementation test carries that trace, and spans remain explicitly retained only in the diagnostic wire form. | FR-003-AC-4, TC-027, tests/integration.rs:143 |
