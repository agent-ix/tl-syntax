---
id: SR-081
title: "Scope and boundary review of the corrected FR-026 owner boundary"
type: SpecReview
analysis: scope-boundary
scope: "semantic and runtime responsibility allocation for native definition, observation, predicate result, native evaluation, TL evaluation and bridge join"
review_set: all
---
## Summary

Every value in the corrected path has one executable owner. The bridge owns
construction/correspondence only; QSL and TL own independent formula evaluation,
QProtocol owns predicate results, and QObs owns external authority artifacts.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | Resolved — formula-wide native truth is allocated to QSL FR-052 rather than inferred by QCI or borrowed from a QProtocol leaf result. | IF-003/005/009; VO-005/009 |
| FND-002 | high | Resolved — QSL request fields carry QObs references but do not authenticate them; QCI cross-checks each against constructor-private QObs views, preserving QObs as sole authority without a QSL→QObs cycle. | QSL FR-052; QObs FR-004; QCI FR-026 |
| FND-003 | medium | Resolved — QCI constructs both owner inputs but invokes neither evaluator; callers obtain results through QSL and TL owner APIs and return only constructor-private validated views. | IF-005; QCI FR-026 |
| FND-004 | medium | Resolved — canonical Protocol assessment and canonical native temporal result are distinct objects with distinct purposes; VO-005 supplies predicate valuations and VO-009 supplies formula truth. | VO-005; VO-009; FR-025/026 |
| FND-005 | low | Out of scope remains transport, scheduling, ambient clocks, persistence, evidence retention, qualification and automatic acceptance; the correction adds none of them. | DOM-001; ADR-003 |
