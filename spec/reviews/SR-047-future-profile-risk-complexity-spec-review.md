---
id: SR-047
title: "Risk-complexity review of the future operator profile"
type: SpecReview
analysis: risk-complexity
scope: "FR-008 through FR-010 and ADR-001"
review_set: all
---

## Summary

**PASS after remediation.** The largest risks are trace-boundary semantic
mislabeling, expansion at the node limit, and version drift across repositories.
The accepted design removes the first and makes the others explicit preflight
and identity obligations.

## Findings

| ID | Severity | Summary | Refs |
|---|---|---|---|
| FND-4701 | high | A one-node X lowering looked cheap but hid a trace-closure mismatch. Fixed by removing X; a future next operator requires a separate closure-aware profile decision. | ADR-001, FR-009 |
| FND-4702 | medium | Nested input can triple graph growth near the limit and expose partial mutation or identity overflow. Fixed with a base count, preflighted count/NodeId bounds, fixed-size output, and no caller mutation. | FR-008-AC-3, TC-044 |
| FND-4703 | medium | Independent dialect, wire, evaluator, and report changes could reuse one vague profile label. Fixed with four orthogonal axes and explicit successor-identity triggers. | FR-009-AC-2, FR-009-AC-3 |
