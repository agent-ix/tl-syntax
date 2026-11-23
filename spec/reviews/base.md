---
id: SR-1500
title: "base review of TL-150 numeric specs (TL-150-spec-tl-syntax)"
type: SpecReview
analysis: base
scope: "tl-syntax ADR-014..018, StR-015/016, FR-302..311, NFR-018/019, TM-017, PLAN-021; cross-checked against tl-parse, tl-mltl, tl-solve numeric specs"
review_set: all
---

## Summary

Base checklist over the tl-syntax portion: ids valid, quoin validate reports no findings, 31 new ACs all mapped to TC rows (computed matrix delta: +31 untagged rows, expected for unbuilt spec; +3 requirement-without-design gaps: FR-305, NFR-018, NFR-019). Main risks are a standing conflict with StR-003, the hash/digest antipattern, and the unresolved contract-scope decision.

Method note: independent fresh-context review, no build tools run. Skill lenses were applied manually by the reviewer in one pass; the criterion-strength lens normally uses the external Jev service, which was not used. All specs are unbuilt and ADRs are proposed.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | StR-003 states tl-syntax does not define the predicate expression and FSL owns typed-predicate lowering; ADR-014 now defines it. StR-003 and FR-007 are not amended, ADR-014 has no relationship to StR-003, and "derived interchange" has no mechanism: no requirement/clause/source-span context (StR-003-VC-2) in the predicate table, aggregation or contract-set documents, so derivation from FSL is unverifiable and FSL predicates using division or floats have no stated loss path. | StR-003:44; ADR-014 decision 6; StR-015 Context |
| FND-002 | high | Hash/digest/pin antipattern (checklist rule, fix is removal): schema SHA-256 constants, graph_identity digest in contract-set, per-file SHA-256 corpus manifest, byte-alteration digest tests, pinned target profile. Follows the family pattern but the checklist says precedent is not a reason to keep it. Human decision whether the family exemption applies. | FR-303; FR-304-AC-3; FR-305; FR-309; NFR-019-AC-2 |
| FND-003 | medium | ADR ids ADR-014/015/018 and StR-014 are reused with different meaning across tl-syntax, tl-mltl (own ADR-014, StR-014) and tl-parse (StR-014); tl-mltl FR-057 cites unqualified "ADR-015", which is not a tl-mltl ADR. Readers cannot resolve citations without repo context. No collision with sibling TL-147/149/151 branches found. | ADR-014; tl-mltl ADR-014; tl-mltl FR-057; tl-parse StR-014 |
| FND-004 | medium | TL-419 AC "ADRs accepted" and "/spec-review recorded" are mapped only to TC-1533, which checks the ADRs are proposed, not accepted; these ACs stay open until a human accepts and this review is recorded. | numeric-predicates-test-matrix.md TL-419 rows; TC-1533 |
| FND-005 | low | FR-309 behavior bullet is garbled ("be distinct nodes is not required"). | FR-309:39 |
| FND-006 | low | TC id ranges: tl-syntax reserves TC-1500..1599, tl-parse TC-1540..1599, tl-mltl uses TC-1570..1602; ranges overlap numerically across repos. Safe only if TC ids are per-repo; state it. | numeric-predicates-test-matrix.md header |
