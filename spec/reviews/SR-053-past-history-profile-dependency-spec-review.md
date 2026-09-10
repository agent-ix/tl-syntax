---
id: SR-053
title: "Dependency review of the past/history profile"
type: SpecReview
analysis: dependency
scope: "FR-011 through FR-013 and routed TL ecosystem follow-ons"
review_set: all
---

## Summary

**PASS after remediation.** Specification work is separated from gated
implementation. M0 and accepted MRS-002/MRS-003 artifacts gate a repository-
owned order from graph/wire through parser, evaluator/rewrite, corpus, and
mapping consumers.

## Findings

| ID | Severity | Summary | Refs |
|---|---|---|---|
| FND-5301 | medium | The initial order named components without allocating every contract to a repository. Fixed with explicit ownership for syntax/wire, text, evaluator/history, rewrite, corpus replay, target adapters, and the native bridge. | FR-013 Dependencies |
| FND-5302 | medium | A prose-only implementation gate was easy to strand or bypass. Fixed with TC-058 as an automated dependency-manifest rejection gate over owner, predecessor, M0, and profile acceptance. | FR-013-AC-5, TC-058 |
| FND-5303 | low | quire-contract-ir #64 could be mistaken for the source of TL past semantics. Fixed: it is a downstream native correspondence consumer, independently blocked on clock/capture/predicate rules. | MRS-003, FR-012, FR-013 |
