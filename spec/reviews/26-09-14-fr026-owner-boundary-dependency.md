---
id: SR-078
title: "Dependency review of the corrected FR-026 owner boundary"
type: SpecReview
analysis: dependency
scope: "runtime and requirement DAG from QSL/QObs/QProtocol through QCI and TL-MLTL"
review_set: all
---
## Summary

The revised graph removes the false Protocol-formula dependency and adds one
acyclic QSL owner prerequisite. Existing predicate projection and temporal
projection work remains usable; only formula-result joining waits for FR-052.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | Resolved — QProtocol FR-042 was incorrectly treated as the native formula-result prerequisite; the real order is QSL FR-051/FR-043/FR-044 → QSL FR-052 → QCI FR-026 result join. | QSL FR-043/044/051/052; QCI FR-026 |
| FND-002 | high | Resolved — placing the bridge-owned mapping inside QSL would recreate the cycle broken by Task-012; QSL owns neutral native request/result contracts and QCI alone binds them to QObs/TL identities. | ADR-003; IF-008/009; QCI FR-028 |
| FND-003 | medium | Resolved — QProtocol FR-042 remains a completed prerequisite through FR-025 valuation rather than being reopened or version-reinterpreted. | QCI FR-025; QProtocol FR-042; Task-006/010 |
| FND-004 | medium | Resolved — Task-007 can retain completed projection modules while Task-010 implements FR-052, then repin QSL and finish join/readback before Task-011. | PLAN-010 Tasks 007/010/011 |
| FND-005 | low | Topological order: existing QSL evaluator and FR-051 handoff; FR-052 owner contracts; QCI sibling request construction and formula join; end-to-end TC-056/model export. No production cycle detected. | ADR-003; PLAN-010 |
