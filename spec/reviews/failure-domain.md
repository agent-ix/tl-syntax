---
id: SR-1506
title: "failure-domain review of TL-150 numeric specs (TL-150-spec-tl-syntax)"
type: SpecReview
analysis: failure-domain
scope: "tl-syntax ADR-014..018, StR-015/016, FR-302..311, NFR-018/019, TM-017, PLAN-021; cross-checked against tl-parse, tl-mltl, tl-solve numeric specs"
review_set: all
---

## Summary

Unstated failure modes, identity confusion, purity, topology.

Method note: independent fresh-context review, no build tools run. Skill lenses were applied manually by the reviewer in one pass; the criterion-strength lens normally uses the external Jev service, which was not used. All specs are unbuilt and ADRs are proposed.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | Identity confusion: same ADR/StR numbers mean different things per repo (see base FND-003); fresh proposition ids from aggregation (first_proposition) and tl-parse predicate allocation (1+max) are two allocators with no stated precedence when a document has both. | FR-306; tl-parse FR-029 |
| FND-002 | medium | Padding/pending topology for contracts at trace end and empty traces (zero instants) is undefined: FR-059 evaluates "0 through last observed position". | tl-mltl FR-059 |
| FND-003 | low | Lexical: a literal absorbs a preceding minus, so "x -5" is ambiguous between subtraction and two tokens; "(x)" parenthesised formula versus "(arith cmp arith)" needs lookahead; v5 forbids reserved words as catalog names so a valid v1 catalog can be unusable in v5. | tl-parse FR-029 |
| FND-004 | low | tl-syntax is no_std today; Integer/FixedDecimal domains are i64 coefficients with u8 scale (no Decimal type), i128 is in core, so exact arithmetic needs no alloc. But content identity (graph_identity) is behind serde/sha2, so a borrowed no_std ContractView cannot verify binding. | src/lib.rs, src/signal/domain.rs; FR-309; NFR-018 |
