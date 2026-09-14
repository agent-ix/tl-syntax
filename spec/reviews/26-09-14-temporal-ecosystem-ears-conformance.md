---
id: SR-073
title: "EARS conformance review of the complete temporal ecosystem"
type: SpecReview
analysis: ears-conformance
scope: "all requirement-bearing artifacts changed for tl-syntax#52 across nine repositories"
review_set: all
---
## Summary

Quire’s EARS diagnostics and a semantic sentence review found wrapped subjects,
multiple SHALL clauses and vague use of “support.” The reviewed trees now emit
no EARS warnings and each affected obligation names a concrete actor and
observable response.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | Resolved — wrapped Description statements separated the subject from SHALL in QSL, Contract IR, tl-syntax and tl-parse; each event statement is now one complete EARS sentence. | QSL FR-051; QCI FR-026; tl-syntax FR-014; tl-parse FR-009 |
| FND-002 | medium | Resolved — several descriptions packed two SHALL responses into one obligation; the second response is now coordinated under one modal or split into a separate actor sentence. | QSL FR-051; QCI FR-026; tl-parse FR-009; tl-rewrite FR-010 |
| FND-003 | medium | Resolved — “support” in criteria could mean capability or evidence; affected criteria now use exact decision-premise set terminology. | QProtocol FR-042-AC-2; QCI FR-025-AC-5; FR-026-AC-5; tl-mltl FR-018-AC-5 |
| FND-004 | medium | Resolved — timestamp order and replay error sentences placed SHALL on a condition rather than the responsible service; Observation requirements now name the library/service response. | qspec FR-113; QObs FR-002 |
| FND-005 | low | Resolved — FR-200 used two SHALL clauses for one ordering response; it now has one modal and an unambiguous compound output. | qspec FR-200 |
