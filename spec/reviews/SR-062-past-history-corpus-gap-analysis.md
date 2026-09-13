---
id: SR-062
title: "Gap analysis — shared past/history corpus"
type: SpecReview
analysis: gap-analysis
scope: "PLAN-010 Task-005; tl-syntax#54; FR-012-AC-3 through AC-5; FR-013-AC-3 through AC-5; TC-056; TC-058"
review_set: subset
---

# Gap analysis — shared past/history corpus

## Summary

Traced Task-005 from accepted criteria through canonical bytes, native consumer
tests, target dispositions, and authorization mutations.

## Verdict

**VALIDATED for the implemented Task-005 feature set.** Every declared corpus
dimension has a concrete row, every row is schema closed and checksum bound,
and the parser, evaluator, and rewrite branches carry byte-identical retained
copies with native replay tests. Task status remains active only until those
three exact consumer commits merge; no feature slice is deferred.

## Trace

| Obligation | Evidence | Result |
|---|---|---|
| Semantic and required-history cases | Eight formula-v2/source pairs; O/H/Y/S/T, endpoints, pre-origin, Boolean nesting | complete |
| History, clock, identity, correction, resources | Three histories, nine results, twelve executed refusals | complete |
| Rewrite behavior | Both reviewed folds and an unchanged boundary, with deterministic replay | complete |
| Target dispositions | Exact FRET/R2U2/C2PO/Isabelle supported/unavailable/unsupported rows | complete |
| Dependency authorization | Production bounded reader plus every prerequisite/task/order/cardinality mutation | complete |
| Cross-repository replay | `tl-parse`, `tl-mltl`, and `tl-rewrite` exact manifest digest pins and native tests | complete pending merge only |

## Findings

| ID | Severity | Summary | Refs |
|---|---|---|---|
| FND-6201 | high | All twelve refusal identities now execute through native profile/history/clock/owner/resource boundaries. | `tl-mltl/tests/past_history_corpus.rs` |
| FND-6202 | medium | TC-056's TL corpus allocation is separated from the remaining Tasks 006–007 native allocation. | TM-003; PLAN-010 |
| FND-6203 | low | A separate identity and directory preserve all existing corpus identities and bytes. | `PAST_HISTORY_CORPUS_V1`; `corpus/past-history/` |
