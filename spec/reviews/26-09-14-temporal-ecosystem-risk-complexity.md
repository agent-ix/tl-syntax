---
id: SR-071
title: "Risk and complexity review of the complete temporal ecosystem"
type: SpecReview
analysis: risk-complexity
scope: "implementation risk of PLAN-010 Tasks 006..012 across the nine-repository ecosystem"
review_set: all
---
## Summary

The highest risks are low-volatility architecture and compatibility risks:
splitting a foundational crate, preserving canonical identities during module
moves, and joining independently versioned owner results. Each now has an
early dependency position and executable compatibility/mutation gate.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | Resolved in plan — the model-package split touches a widely imported API; Task-012 lands first and TC-041 requires full QCI/QSL corpus and byte/API compatibility before owner work. | quire-contract-ir FR-028; Task-012; TC-041 |
| FND-002 | high | Resolved in plan — nine independently versioned strict contracts create cross-pin drift risk; Wave 3 admits exact revisions/schema digests and rejects copied or moving selections end to end. | VO-001; Task-011; TC-056/TC-040 |
| FND-003 | high | Resolved in requirements — progress, closure, completeness, activation, truth and settlement can collapse combinatorially; owner FRs require independent axes and table/mutation tests before Boolean projection. | FR-025/FR-026; QObs FR-004; QProtocol FR-042; tl-mltl FR-018 |
| FND-004 | medium | Resolved in architecture — large mixed modules make reorganization regression-prone; Task-009 permits only boundary-justified moves with compatibility re-exports and full corpus replay. | ADR-003 topology; Task-009; existing qci expression.rs/conformance.rs and tl-mltl past.rs |
| FND-005 | medium | Resolved in authority boundary — future self-analysis could become circular certification; FR-027 and IF-006 prohibit model output from any acceptance or owner-admission input. | IF-006; quire-contract-ir FR-027 Authority boundary |
