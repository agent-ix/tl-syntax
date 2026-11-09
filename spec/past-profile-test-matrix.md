---
id: TM-003
title: "Past/history profile test matrix"
type: TestMatrix
relationships:
  - target: ix://agent-ix/tl-syntax/MRS-003
    type: covers
---

# Past/history profile test matrix

## Functional Requirement Coverage

| Functional Req | Acceptance Criteria | Test Cases | Status |
|---|---|---|---|
| FR-011 | FR-011-AC-4 | TC-053 | ✅ implemented (AC-1, AC-2, AC-3 and AC-5 re-homed to tl-mltl, TL-378) |
| FR-012 | FR-012-AC-3, FR-012-AC-4 | TC-051, TC-056 | ✅ implemented (AC-1, AC-2 and AC-5 re-homed to tl-mltl) |
| FR-013 | FR-013-AC-1, FR-013-AC-3 through FR-013-AC-5 | TC-054, TC-056 through TC-058 | ✅ implemented (AC-2 re-homed to tl-parse) |
| FR-014 | FR-014-AC-1 through FR-014-AC-5 | TC-075 | ✅ covered |

## Profile evidence allocation

| Concern | Syntax/wire | Evaluate/progress | History/resource | Parse/format | Rewrite/corpus | Interoperability |
|---|---|---|---|---|---|---|
| O/H/Y | TC-054, TC-057 | TC-048, TC-050, TC-052 | TC-051, TC-052 | TC-055, TC-057 | TC-056 | TC-056 |
| S/T | TC-054, TC-057 | TC-049, TC-050, TC-052 | TC-051, TC-052 | TC-055, TC-057 | TC-056 | TC-056 |
| existing future profiles | TC-054 unchanged/upgrade | unchanged | lookahead unchanged | old v1 unchanged | existing corpus unchanged | existing target states |
| mixed/refused profiles | TC-053, TC-054, TC-057 | no result | no fabricated history | stable refusal | negative corpus | unsupported/unavailable |

## Test Case Summary

| Test ID | Title | Type | Priority | Traces To | Status |
|---|---|---|---|---|---|
| TC-048 | Evaluate O/H against an independent reverse-offset oracle across Boolean, endpoint, singleton, and pre-origin cases | Property | P0 | FR-011-AC-1 | ⛔ retired — re-homed to tl-mltl, which implements and tests the evaluator in `tests/past_history.rs`; its criteria are withdrawn here pending TL-378 |
| TC-049 | Evaluate S with the exact reverse `[a,j)` left range and inclusive witness endpoints | Property | P0 | FR-011-AC-2 | ⛔ retired — re-homed to tl-mltl, which implements and tests the evaluator in `tests/past_history.rs`; its criteria are withdrawn here pending TL-378 (`since_ignores_offsets_before_nonzero_lower_bound_and_accepts_both_endpoints`) |
| TC-050 | Evaluate T against the structural Boolean dual of S and strong Previous against O[1,1] across all bounded/origin cases | Property | P0 | FR-011-AC-5 | ⛔ retired — re-homed to tl-mltl, which implements and tests the evaluator in `tests/past_history.rs`; its criteria are withdrawn here pending TL-378 |
| TC-051 | Validate origin-complete histories, anchors, digests, ordering, gaps, duplicates, and result attribution | Property | P0 | FR-012-AC-3 | ✅ result attribution implemented in `tests/past_history_corpus.rs`; history admission is tl-mltl and withdrawn here pending TL-378 |
| TC-052 | Compare evaluator and required-history analysis with the independent Rust oracle across both valid clocks and all semantic/resource boundaries | Property | P0 | FR-011-AC-1, FR-012-AC-4, FR-013-AC-3 | ⛔ retired — re-homed to tl-mltl, which implements and tests the evaluator in `tests/past_history.rs`; its criteria are withdrawn here pending TL-378; the evaluator-versus-oracle comparison is a tl-mltl test |
| TC-053 | Refuse weak Previous, future/mixed graphs, invalid histories, malformed/overflowing/rounded/resampled clocks, and preserve owner-produced capture/predicate non-values without coercion | Integration | P0 | FR-011-AC-4 | ✅ implemented in `tests/past_formula_v2.rs` (graph, profile and wire admission); evaluator-entry refusal and history admission are tl-mltl `tests/past_history.rs` |
| TC-054 | Round-trip formula-v2 profiles and enforce v1 preservation, v1-to-v2 upgrade, guarded down-conversion, and profile/operator compatibility | Integration | P0 | FR-013-AC-1 | ✅ implemented |
| TC-055 | Parse and canonically format O/H/Y/S/T in `tl-parse.clean-ascii/v3` with exact precedence, intervals, associativity, and spans | Integration | P0 | FR-013-AC-2 | ⛔ retired — re-homed to tl-parse, which owns and tests `tl-parse.clean-ascii/v3` in `tests/clean_ascii_v3.rs` (tl-parse FR-009); FR-013-AC-2 is withdrawn here |
| TC-056 | Replay the paired corpus, every mutation, late-data/anchor/closure identity, rewrite rule, and external-target state | Integration | P0 | FR-012-AC-3, FR-012-AC-4, FR-013-AC-3, FR-013-AC-4 | ✅ implemented across the TL corpus/rewrite suites and Contract IR TC-039/TC-040 |
| TC-057 | Exercise bounded arbitrary formula-v2 decoding without unwind or profile misattribution (v3-dialect parsing is tl-parse’s) | Property | P1 | FR-013-AC-1 | ✅ implemented |
| TC-058 | Run the dependency-manifest gate over every implementation ticket owner, predecessor, M0 gate, and MRS acceptance prerequisite | Integration | P0 | FR-013-AC-5 | ✅ implemented |
| TC-075 | Strict-read and compatibility-check every syntax owner artifact | Integration | P0 | FR-014-AC-1, FR-014-AC-2, FR-014-AC-3, FR-014-AC-4, FR-014-AC-5 | ✅ implemented in `tests/strict_syntax_artifacts.rs` |

## Owner evidence for re-homed criteria

`quire coverage` scopes trace tags to one repository, so a criterion whose
tests live elsewhere is withdrawn here and verified by its owner. No behavior is
dropped; each row names where the evidence runs.

| Criterion here | Owner | Evidence |
|---|---|---|
| FR-011-AC-1, AC-2, AC-3, AC-5 (O/H/S/T and strong-Previous evaluation) | tl-mltl | `tests/past_history.rs`; tl-mltl has no counterpart requirement or trace tags yet; TL-378 adds them, and the criterion text is preserved in the withdrawn rows until then |
| FR-012-AC-1, FR-012-AC-2, FR-012-AC-5 (history admission, required-history analysis, clocks and owner non-values) | tl-mltl | `tests/past_history.rs`, `tests/past_readers.rs`; TL-378 |
| FR-013-AC-2 (`tl-parse.clean-ascii/v3`) | tl-parse | tl-parse FR-009, `tests/clean_ascii_v3.rs` (traced there as FR-009-AC-1 through AC-5) |
