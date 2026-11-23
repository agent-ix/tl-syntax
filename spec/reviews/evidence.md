---
id: SR-1507
title: "evidence review of TL-150 numeric specs (TL-150-spec-tl-syntax)"
type: SpecReview
analysis: evidence
scope: "tl-syntax ADR-014..018, StR-015/016, FR-302..311, NFR-018/019, TM-017, PLAN-021; cross-checked against tl-parse, tl-mltl, tl-solve numeric specs"
review_set: all
---

## Summary

Verification method fit and grounding of evidence.

Method note: independent fresh-context review, no build tools run. Skill lenses were applied manually by the reviewer in one pass; the criterion-strength lens normally uses the external Jev service, which was not used. All specs are unbuilt and ADRs are proposed.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | C2PO/R2U2 claims are not grounded in the retained corpus: corpus/r2u2-v4.2 formulas.c2po contains only bool INPUTs and FTSPEC/PTSPEC; no int/float types, tolerance, abs, contracts, foreach/forsome/atleast, integer division. Unverified facts: float-with-tolerance semantics (ADR-015 context), typed INPUT declarations, compile-time aggregation expansion (ADR-016), contract status decodable as A / A&&G / A&&!G (FR-060 labels it "grounded"). Authors flag some as DEFERRED; the FR-060 "grounded" label should go. | ADR-015; ADR-016; tl-mltl FR-060:26; corpus/r2u2-v4.2/formulas.c2po |
| FND-002 | medium | Per-step differential has dispositions agree/disagree/refused-at-export only; R2U2 does not emit verdicts for the last positions of F/G[0,1] formulas on a 4-step trace, so 6,912 and 192 record counts have no home for "no R2U2 verdict". | tl-mltl FR-058; FR-060 |
| FND-003 | low | Verification labels: FR-204-AC-1 exhaustive enumeration is labelled property-based-testing; FR-058-AC-4 existence of bytes is "demonstration" (inspection). | tl-solve FR-204; tl-mltl FR-058 |
| FND-004 | low | Population arithmetic reproduced independently (see criterion-strength and tl-solve gap-analysis). 567, 6,912 and 192 all recompute. | FR-308; FR-058; FR-060 |
