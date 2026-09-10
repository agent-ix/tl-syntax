---
id: SR-052
title: "Integrity review of the past/history profile"
type: SpecReview
analysis: integrity
scope: "MRS-003, FR-011 through FR-013, ADR-002, TM-003, tracked-source census"
review_set: all
---

## Summary

**PASS after remediation.** The artifacts use unique identities and explicit
relationships, and every criterion maps to planned evidence without claiming
implementation. Quire reports the complete corpus grammar-clean.

## Findings

| ID | Severity | Summary | Refs |
|---|---|---|---|
| FND-5201 | medium | Past nodes cannot be added under the closed formula-v1 identity. Fixed: formula-v2 is additive, v1 bytes/outcomes remain unchanged, and upgrade/down-conversion conditions are explicit. | FR-013-AC-1, ADR-002 |
| FND-5202 | low | Six new live specification artifacts change the exact tracked-source partition while these eight SpecReviews remain archival. Fixed by updating the expected array and spec-area count; a one-count mutation makes the census fail. | tests/shared_assurance.rs |
| FND-5203 | low | TC-048 through TC-058 are intentionally unimplemented. Retained as planned so coverage reports missing backing without any status lie. | TM-003 |
| FND-5204 | low | The post-v0.1 past profile was reachable only through its dependency on MRS-002. Fixed with a direct master-specification reference while leaving MRS-001's v0.1 scope unchanged. | MRS-001, MRS-003 |
