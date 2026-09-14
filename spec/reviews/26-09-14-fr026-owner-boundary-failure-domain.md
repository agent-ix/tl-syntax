---
id: SR-076
title: "Failure-domain review of the corrected FR-026 owner boundary"
type: SpecReview
analysis: failure-domain
scope: "QCI FR-026, QSL FR-052/TC-140, IF-003/005/009, VO-005/007/009, ADR-003 and PLAN-010 Tasks 007/010"
review_set: all
---
## Summary

The review followed substitution, identity, missing-input, multi-leaf,
correction, malformed-byte and resource failures across the corrected native/TL
result path. Every discovered failure-domain gap is resolved in the authored
design before implementation resumes.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | Resolved — one Protocol result mapping is checked-leaf-bound, so equating its Boolean with a multi-leaf TL formula was a type-confusion path; FR-026/ADR-003 now reserve it for FR-025 valuation and require a formula-wide QSL result. | QCI FR-025/026; QProtocol FR-042; ADR-003; IF-003/005 |
| FND-002 | high | Resolved — the existing QSL evaluator accepts caller-built trace assertions and emits no canonical strict-readable result; QSL FR-052 adds canonical request/result identities, strict rederivation and constructor-private views while QCI cross-checks opaque observation references. | QSL FR-043/052; IF-009; VO-009 |
| FND-003 | high | Resolved — correction predecessors live in distinct QSL and TL identity domains; FR-026 requires a strict-read prior join binding both exact predecessor identities/digests and refuses mixed, missing, self or cross-domain edges. | QCI FR-026-AC-6; IF-005; VO-007 |
| FND-004 | medium | Resolved — leaf multiplicity could alter result arity or permit a representative-leaf shortcut; the projection now requires the complete position/leaf valuation product while each evaluator emits exactly one formula-wide result. | QCI FR-026; QSL FR-052-AC-1/2 |
| FND-005 | medium | Resolved — unbounded request/result rederivation could allocate before refusal; FR-052 and FR-026 independently bound every retained collection and total work, charge before allocation/traversal, and expose no partial Boolean. | QSL FR-052-AC-7; QCI FR-026-AC-8 |
