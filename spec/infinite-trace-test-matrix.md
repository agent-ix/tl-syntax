---
id: TM-005
title: "Infinite-trace syntax and corpus test matrix"
type: TestMatrix
relationships:
  - target: ix://agent-ix/tl-syntax/MRS-002
    type: covers
---

# Infinite-trace syntax and corpus test matrix

## Functional Requirement Coverage

| Functional Req | Acceptance Criteria | Test Cases | Status |
|---|---|---|---|
| FR-020 | FR-020-AC-1 through FR-020-AC-3 | TC-148 through TC-150 | ✅ implemented |
| FR-021 | FR-021-AC-1 through FR-021-AC-3 | TC-151 through TC-153 | ✅ implemented |
| FR-022 | FR-022-AC-1 through FR-022-AC-3 | TC-154 through TC-156 | ✅ implemented |
| FR-023 | FR-023-AC-1 through FR-023-AC-3 | TC-157 through TC-159 | ✅ implemented |
| FR-024 | FR-024-AC-1 through FR-024-AC-3 | TC-160 through TC-162 | ✅ implemented |

TC-150's v1/v2 golden-byte and TL profile refusal assertions execute in
`tests/infinite_formula.rs`. Its QSL correspondence is an inspection: no
tl-syntax, tl-parse, tl-mltl, or tl-rewrite source names QSL or
`quire.temporal`, so no QSL-to-TL lowering exists (a recursive search of
`src` in the four crates for those names returns nothing at the revisions
below). The native QSL result comparison is owned by the QSL repo and is not
asserted here. TC-162 is an inspection: tl-parse
(`tests/owner_infinite_corpus.rs`), tl-mltl (`tests/infinite_corpus.rs`), and
tl-rewrite (`tests/infinite_owner_corpus.rs`) read the owner corpus through
`tl_syntax::CORPUS_DIR`, and none of them carries a copy of
`infinite-trace/cases.json`; the owner-side resolvability is executed by
`tc_162_owner_corpus_is_resolvable_only_through_corpus_dir`. Inspected at
the tl-syntax TL-257 branch, tl-parse `5f9a21c`, tl-mltl `c2e7448`, and
tl-rewrite `3f5cc0c`.

The corpus includes an unfair loop and an explicitly selected finite-prefix
subject alongside its valid lasso carrier; only the prefix observations enter
that case's oracle derivation. Executing syntax evidence is in `tests/infinite_formula.rs`,
`tests/infinite_trace.rs`, and `tests/infinite_trace_corpus.rs`. A matrix row
becomes implemented only when its assertion executes against the real public
API and passes; an ignored placeholder is not coverage.

## Test Case Summary

| Test ID | Title | Type | Priority | Traces To | Status |
|---|---|---|---|---|---|
| TC-148 | Keep all TL profile IDs distinct and refuse QSL-as-TL or absent identity | Unit | P0 | FR-020-AC-1 | ✅ implemented |
| TC-149 | Preserve event-position identity and refuse each incompatible clock selection | Property | P0 | FR-020-AC-2 | ✅ implemented |
| TC-150 | Golden-byte v1/v2 compatibility and QSL correspondence inspection | Integration | P0 | FR-020-AC-3 | ✅ implemented |
| TC-151 | Round-trip empty and nonempty ordered fairness premises with graph/clock identity | Property | P0 | FR-021-AC-1 | ✅ implemented |
| TC-152 | Refuse duplicate, foreign, invalid, or mismatched fairness roots | Property | P0 | FR-021-AC-2 | ✅ implemented |
| TC-153 | Refuse any syntax-level fairness or liveness verdict on a finite prefix | Integration | P0 | FR-021-AC-3 | ✅ implemented |
| TC-154 | Round-trip empty-prefix and nonempty-prefix lassos with nonempty loops | Property | P0 | FR-022-AC-1 | ✅ implemented |
| TC-155 | Refuse empty loops, invalid positions, propositions, and identities | Property | P0 | FR-022-AC-2 | ✅ implemented |
| TC-156 | Unroll loop positions without inventing trace closure | Property | P0 | FR-022-AC-3 | ✅ implemented |
| TC-157 | Preserve each of four partial-valuation states and stable order | Property | P0 | FR-023-AC-1 | ✅ implemented |
| TC-158 | Refuse unknown, duplicate, omitted, or unordered valuation entries | Property | P0 | FR-023-AC-2 | ✅ implemented |
| TC-159 | Keep missing distinct from conflicting with no Boolean coercion | Unit | P0 | FR-023-AC-3 | ✅ implemented |
| TC-160 | Verify manifest identities, human oracle derivations, and schemas | Integration | P0 | FR-024-AC-1 | ✅ implemented |
| TC-161 | Replay each positive/negative family and fail under input or oracle mutation | Integration | P0 | FR-024-AC-2 | ✅ implemented |
| TC-162 | Inspect downstream `CORPUS_DIR` use and absence of vendored copies | Inspection | P1 | FR-024-AC-3 | ✅ implemented |

## Integration Test Matrix

| Purpose | Target | Type | Test Cases |
|---|---|---|---|
| Parse, rewrite, and evaluate the same owner corpus | tl-parse, tl-rewrite, tl-mltl | service | TC-150, TC-160, TC-161, TC-162 |
| Compare results where both native and TL infinite-trace profiles apply | QSL native temporal provider | service | TC-150, TC-160 |
