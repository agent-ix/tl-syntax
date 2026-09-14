---
id: SR-069
title: "Dependency review of the complete temporal ecosystem"
type: SpecReview
analysis: dependency
scope: "ADR-003 runtime/normative graph, PLAN-010 Tasks 001..012, Cargo manifests of all eight executable repositories"
review_set: all
---
## Summary

The review compared the specified graph with real Cargo metadata and produced
an acyclic owner-to-consumer order. The only latent production cycle is removed
by the specified model-package split before any owner pin advances.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | Resolved — current QSL depends heavily on the Contract IR substrate, so adding the required QCI-to-QSL owner edge would form a Cargo cycle. ADR-003, IF-008, FR-028 and Task-012 split the substrate before owner implementation. | QSL Cargo.toml/src; ADR-003; IF-008; quire-contract-ir FR-028; Task-012 |
| FND-002 | high | Resolved — Task-010 previously could start before the cycle break; PLAN-010 now orders Task-008 → Task-012 → Task-010 and bridge Tasks 006/007 consume Task-010. | PLAN-010 Dependency Graph; Task-010; Task-012 |
| FND-003 | medium | Resolved — the model exporter had no component allocation and therefore no place in the runtime DAG; it is now owned by Contract IR FR-027 and implemented only after both bridges and exact revisions exist. | ADR-003; IF-006; quire-contract-ir FR-027; Task-011 |
| FND-004 | medium | Resolved — QSL-to-Protocol’s real production edge was absent from the umbrella diagram; ADR-003 now records model → QSL → Protocol plus all owner-to-bridge edges. | QSL/Protocol Cargo.toml; ADR-003 |
| FND-005 | low | Resolved — TL rewrite’s production dependence on both tl-syntax and tl-mltl and its historical test-only pins are explicitly preserved and isolated. | tl-rewrite Cargo.toml; ADR-003; tl-rewrite FR-010; Task-009 |
