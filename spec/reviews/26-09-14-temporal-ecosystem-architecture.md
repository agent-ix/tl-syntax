---
id: SR-075
title: "Architecture evaluation of the complete temporal ecosystem"
type: SpecReview
analysis: architecture-evaluation
scope: "current code/module/Cargo topology and reviewed target DOM-001/ADR-003/PLAN-010 across all nine repositories"
review_set: all
---
## Summary

Success, refusal, correction, dependency-change and self-model scenarios were
walked from native definition through owner readers, TL evaluation and result
join. The reviewed architecture resolves the two blocking design defects—a
latent Cargo cycle and an ownerless model exporter—and allocates existing
module-coupling repairs to compatibility-gated implementation tasks.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | Resolved in architecture — QSL’s real production dependency on QCI conflicts with the required bridge-to-QSL edge; the two-package Contract IR boundary breaks the cycle without copying types or changing QSL imports. | QSL Cargo.toml/src; ADR-003; IF-008; QCI FR-028; Task-012 |
| FND-002 | high | Resolved in architecture — IF-006 had no runtime owner, so the self-model path ended at an abstraction; Contract IR FR-027 now owns strict manifest/model production and Task-011 owns integration. | IF-006; QCI FR-027; Task-011 |
| FND-003 | high | Resolved in architecture — the observation success/failure path authenticated only part of the required authority set and crossed into Protocol activation; IF-002/QObs FR-004 now define nine readers and a clean activation boundary. | QObs src/lib.rs/replay.rs/handoff.rs; IF-002; QObs FR-004 |
| FND-004 | medium | Resolved in implementation architecture — current large modules mix policy, traversal, wire and decisions; ADR-003 specifies subsystem targets, stable re-exports and byte/result compatibility, and Task-009/010/012 own the moves before bridge integration. | QCI expression.rs/conformance.rs; QSL state/evaluation.rs; tl-mltl past.rs; tl-rewrite rewrite.rs; ADR-003; Tasks 009/010/012 |
| FND-005 | medium | Resolved in process architecture — model-generated proposals could feed the source loop without an explicit authority break; PROC-001 and FR-027 require a distinct external owner review/merge/selection before any proposal is effective. | PROC-001; IF-006; QCI FR-027 |
