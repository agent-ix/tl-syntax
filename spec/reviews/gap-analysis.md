---
id: SR-1509
title: "gap-analysis review of TL-150 numeric specs (TL-150-spec-tl-syntax)"
type: SpecReview
analysis: gap-analysis
scope: "TL-419..TL-426 acceptance criteria mapped to requirements and matrix rows across all four repos"
review_set: all
---

## Summary

Planless gap analysis of the spec package: every AC of requirements FR-302..311, NFR-018/019 (31 ACs) maps to a TM-017 row; every ticket AC maps to a requirement and a row, with the exceptions below. Plan completion: not assessed. Computed matrix delta vs main: tl-syntax +31 rows, tl-parse +19, tl-mltl +27, tl-solve +21, all untagged (expected, unbuilt); quoin validate: no findings in all four, same as baseline. No code exists, so stub and tag checks are not applicable.

Method note: independent fresh-context review, no build tools run. Skill lenses were applied manually by the reviewer in one pass; the criterion-strength lens normally uses the external Jev service, which was not used. All specs are unbuilt and ADRs are proposed.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | TL-419 AC1 (ADRs accepted) and AC3 (/spec-review recorded): no requirement can satisfy them; they are human actions. TC-1533 only checks "proposed". | TL-419; TC-1533 |
| FND-002 | high | TL-423 AC1 (R2U2 differential on the numeric corpus, per-step verdicts recorded) maps to TC-1589/1590/1592, all blocked DEFERRED; with ADR-014/015 no numeric export is admitted until a pinned profile exists, so the AC cannot be met by the specified work alone. Human decision: who runs R2U2 and pins the profile. | TL-423; tl-mltl FR-058; ADR-015 d3 |
| FND-003 | medium | TL-425 AC2 (exported to C2PO contracts) is satisfied only by decomposition into three ordinary formulas; native C2PO contracts DEFERRED; differential TC-1598 blocked. | tl-mltl FR-060 |
| FND-004 | medium | TL-425 AC1 "evaluated with the three statuses" maps to per-position series (FR-059), no trace-level status. | tl-mltl FR-059 |
| FND-005 | low | TL-426 AC "witness traces re-evaluated by tl-mltl" depends on tl-mltl FR-053/054 which build under TL-422; order not stated. | tl-solve FR-203 |
| FND-006 | low | Requirement-without-design: FR-305, NFR-018, NFR-019 (+3 vs baseline); tl-parse +7 (no ADR for dialect v5), tl-mltl +7, tl-solve +7. | computed matrix |
