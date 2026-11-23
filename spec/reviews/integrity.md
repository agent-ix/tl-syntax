---
id: SR-1501
title: "integrity review of TL-150 numeric specs (TL-150-spec-tl-syntax)"
type: SpecReview
analysis: integrity
scope: "tl-syntax ADR-014..018, StR-015/016, FR-302..311, NFR-018/019, TM-017, PLAN-021; cross-checked against tl-parse, tl-mltl, tl-solve numeric specs"
review_set: all
---

## Summary

Internal and cross-document consistency, including the contract and arithmetic semantics the specs define.

Method note: independent fresh-context review, no build tools run. Skill lenses were applied manually by the reviewer in one pass; the criterion-strength lens normally uses the external Jev service, which was not used. All specs are unbuilt and ADRs are proposed.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | Contract semantics diverge from P-TL-149 ADR-004 decision 5. 149: contract (A,G) contributes A->G over the whole fixed trace, winning condition via evaluate_closed_at at position zero, status coincides with a play outcome, every proposition of A must be an Input signal, closed profile only. 150 ADR-017 option C: realizability defines winning as no contract ever invalid, status per position (all positions), no input-only rule, closed and prefix. "Never invalid at any position" is the stronger reading 149 lists as an unspecified known limit (a); position-zero implication differs from per-position. The two do not agree; 149 also recommends contracts ARE environment assumptions while 150 refuses to claim it. Both defer to Peter, but the stated recommendation C is not what 149 describes. | ADR-017 option C (line 76); FR-311; tl-mltl-149 ADR-004 decision 5 |
| FND-002 | medium | There is no trace-level contract status. TL-425 asks for three states; FR-059/ADR-017 give per-position series plus counts and first invalid time. A never-met assumption is all-inactive, a mid-trace violation is invalid at t then possibly verified again later (non-sticky); the whole-trace answer 149 needs is undefined. | FR-059 outputs; FR-310 |
| FND-003 | medium | Closed-profile padding makes the nine-row truth table partly unreachable and misleading: closed profile never yields Pending, so FR-059-AC-1 "nine combinations on closed and prefix" cannot hold on closed; near the trace end padded-false guarantees with future operators produce spurious invalid (and padded-false assumptions spurious inactive) at the last positions. Not discussed. | tl-mltl FR-059 behavior and AC-1; FR-055 |
| FND-004 | medium | StR-003 conflict resolution is only partly coherent; see base FND-001. In addition TL text dialect v5 is parsed from author-written source, which is an editing authority in practice despite the "derived" claim. | ADR-014 d6; tl-parse StR-014 |
| FND-005 | low | ADR-017 consequences say four outcomes while ticket and StR-016 say three statuses; vocabulary is consistent but the 4th (pending) is not in TL-425. | ADR-017 Consequences |
| FND-006 | low | Numeric typing is consistent across tl-syntax FR-302, tl-parse FR-029 and tl-solve FR-201 (derived ranges i64, widths bitlength+1). No contradiction found. | FR-302; tl-solve FR-201 |
