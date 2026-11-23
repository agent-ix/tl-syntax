---
id: SR-1502
title: "ears-conformance review of TL-150 numeric specs (TL-150-spec-tl-syntax)"
type: SpecReview
analysis: ears-conformance
scope: "tl-syntax ADR-014..018, StR-015/016, FR-302..311, NFR-018/019, TM-017, PLAN-021; cross-checked against tl-parse, tl-mltl, tl-solve numeric specs"
review_set: all
---

## Summary

EARS grammar of FR/NFR/StR statements.

Method note: independent fresh-context review, no build tools run. Skill lenses were applied manually by the reviewer in one pass; the criterion-strength lens normally uses the external Jev service, which was not used. All specs are unbuilt and ADRs are proposed.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | FR-310 and FR-311 descriptions use unconditional "shall define/expose" with no trigger; acceptable as ubiquitous but FR-311 is PROVISIONAL and cannot be verified until the decision exists. | FR-310; FR-311 |
| FND-002 | low | NFR-018 statement is one sentence with five unrelated clauses (no_std, no alloc, zero deps, features) and conditional "when used through default features"; split for atomicity. | NFR-018 |
| FND-003 | low | tl-parse FR-028 combines two behaviors (v5 selection and unchanged other dialects) and refers to "the two future profiles above", a garbled reference. | tl-parse FR-028:43 |
| FND-004 | low | FR-309 contains an ungrammatical sentence (see base). Other FRs are conformant When/If forms. | FR-309:39 |
