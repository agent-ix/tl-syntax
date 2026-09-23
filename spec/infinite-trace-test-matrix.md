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

| Functional Req | Acceptance Criteria | Test Cases | Coverage Status |
|---|---|---|---|
| FR-020 | FR-020-AC-1 through FR-020-AC-3 | TC-148 through TC-150 | 🚧 planned |
| FR-021 | FR-021-AC-1 through FR-021-AC-3 | TC-151 through TC-153 | 🚧 planned |
| FR-022 | FR-022-AC-1 through FR-022-AC-3 | TC-154 through TC-156 | 🚧 planned |
| FR-023 | FR-023-AC-1 through FR-023-AC-3 | TC-157 through TC-159 | 🚧 planned |
| FR-024 | FR-024-AC-1 through FR-024-AC-3 | TC-160 through TC-162 | 🚧 planned |

The focused ignored-stub lane in `tests/v1_spec_stubs.rs` is intentionally red
until TL-15 implements the reviewed syntax contracts. Normal local gates keep
their pre-existing green baseline during the spec cycle. A matrix row becomes
implemented only when its corresponding assertion executes against the real
public API and passes; an ignored placeholder is not coverage.

## Test Case Summary

| Test ID | Title | Type | Priority | Traces To | Status |
|---|---|---|---|---|---|
| TC-148 | Keep all TL profile IDs distinct and refuse QSL-as-TL or absent identity | Unit | P0 | FR-020-AC-1 | 🚧 planned |
| TC-149 | Preserve event-position identity and refuse each incompatible clock selection | Property | P0 | FR-020-AC-2 | 🚧 planned |
| TC-150 | Golden-byte v1/v2 compatibility and QSL correspondence inspection | Integration | P0 | FR-020-AC-3 | 🚧 planned |
| TC-151 | Round-trip empty and nonempty ordered fairness premises with graph/clock identity | Property | P0 | FR-021-AC-1 | 🚧 planned |
| TC-152 | Refuse duplicate, foreign, invalid, or mismatched fairness roots | Property | P0 | FR-021-AC-2 | 🚧 planned |
| TC-153 | Refuse any syntax-level fairness or liveness verdict on a finite prefix | Integration | P0 | FR-021-AC-3 | 🚧 planned |
| TC-154 | Round-trip empty-prefix and nonempty-prefix lassos with nonempty loops | Property | P0 | FR-022-AC-1 | 🚧 planned |
| TC-155 | Refuse empty loops, invalid positions, propositions, and identities | Property | P0 | FR-022-AC-2 | 🚧 planned |
| TC-156 | Unroll loop positions without inventing trace closure | Property | P0 | FR-022-AC-3 | 🚧 planned |
| TC-157 | Preserve each of four partial-valuation states and stable order | Property | P0 | FR-023-AC-1 | 🚧 planned |
| TC-158 | Refuse unknown, duplicate, omitted, or unordered valuation entries | Property | P0 | FR-023-AC-2 | 🚧 planned |
| TC-159 | Keep missing distinct from conflicting with no Boolean coercion | Unit | P0 | FR-023-AC-3 | 🚧 planned |
| TC-160 | Verify manifest identities, human oracle derivations, schemas, and file digests | Integration | P0 | FR-024-AC-1 | 🚧 planned |
| TC-161 | Replay each positive/negative family and fail under input, oracle, or digest mutation | Integration | P0 | FR-024-AC-2 | 🚧 planned |
| TC-162 | Inspect pinned downstream `CORPUS_DIR` use and absence of vendored copies | Inspection | P1 | FR-024-AC-3 | 🚧 planned |

## Integration Test Matrix

| Purpose | Target | Type | Test Cases |
|---|---|---|---|
| Parse, rewrite, and evaluate the same owner corpus at exact pins | tl-parse, tl-rewrite, tl-mltl | service | TC-150, TC-160, TC-161, TC-162 |
| Compare results where both native and TL infinite-trace profiles apply | QSL native temporal provider | service | TC-150, TC-160 |
