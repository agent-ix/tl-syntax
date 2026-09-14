---
id: SR-080
title: "Risk and complexity review of the corrected FR-026 owner boundary"
type: SpecReview
analysis: risk-complexity
scope: "implementation risk for QSL FR-052 and QCI FR-026 under PLAN-010"
review_set: all
---
## Summary

The top hazards are low-volatility correctness risks: wrapping an existing
native evaluator without semantic drift, canonicalizing all independent axes,
and pairing corrections across identity domains. Each has an early contract or
differential gate; no high-volatility product rule is introduced.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | Mitigated — FR-052 changes a mature evaluator boundary and could alter native meaning; implementation must delegate to FR-043 and replay every existing TC-122 discriminator through TC-140 before promotion. | QSL FR-043/052; TC-122/140 |
| FND-002 | high | Mitigated — canonical request/result identity and strict re-evaluation are new published contracts; independent schema digests, golden bytes, field mutations and exact resource boundaries are mandatory. | QSL FR-052-AC-1/4/7 |
| FND-003 | high | Mitigated — paired correction logic can confuse QSL, TL, mapping and correspondence identities; typed identity roles plus a strict-read prior join and cross-domain substitution controls own the risk. | QCI FR-026-AC-2/6; VO-007/009 |
| FND-004 | medium | Mitigated — retaining four axes around an evaluator that consumes closure plus watermark may accidentally couple state; table tests independently mutate all four retained references and completeness. | QSL FR-052-AC-5; QCI FR-026-AC-5 |
| FND-005 | medium | Mitigated — multi-profile fixture construction is large; one reusable owner corpus must cover future event/fixed-sample and pure-past O/H/Y/S/T without copying semantics into QCI. | TC-140; TC-039 |
