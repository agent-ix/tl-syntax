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

| Functional Req | Acceptance Criteria | Test Cases | Coverage Status |
|---|---|---|---|
| FR-011 | FR-011-AC-1 through FR-011-AC-4 | TC-048, TC-049, TC-050, TC-052, TC-053 | 🚧 planned |
| FR-012 | FR-012-AC-1 through FR-012-AC-5 | TC-051, TC-052, TC-053, TC-056 | 🚧 planned |
| FR-013 | FR-013-AC-1 through FR-013-AC-5 | TC-052, TC-054 through TC-058 | 🚧 planned |

## Profile evidence allocation

| Concern | Syntax/wire | Evaluate/progress | History/resource | Parse/format | Rewrite/corpus | Interoperability |
|---|---|---|---|---|---|---|
| O/H | TC-054, TC-057 | TC-048, TC-052 | TC-051, TC-052 | TC-055, TC-057 | TC-056 | TC-056 |
| S/T | TC-054, TC-057 | TC-049, TC-050, TC-052 | TC-051, TC-052 | TC-055, TC-057 | TC-056 | TC-056 |
| existing future profiles | TC-054 unchanged/upgrade | unchanged | lookahead unchanged | old v1 unchanged | existing corpus unchanged | existing target states |
| mixed/refused profiles | TC-053, TC-054, TC-057 | no result | no fabricated history | stable refusal | negative corpus | unsupported/unavailable |

## Test Case Summary

| Test ID | Title | Type | Priority | Traces To | Status |
|---|---|---|---|---|---|
| TC-048 | Evaluate O/H against an independent reverse-offset oracle across Boolean, endpoint, singleton, and pre-origin cases | Property | P0 | FR-011-AC-1 | 🚧 planned |
| TC-049 | Evaluate S with the exact reverse `[a,j)` left range and inclusive witness endpoints | Property | P0 | FR-011-AC-2 | 🚧 planned |
| TC-050 | Evaluate T against the structural Boolean dual of S across all bounded cases | Property | P0 | FR-011-AC-3 | 🚧 planned |
| TC-051 | Validate origin-complete histories, anchors, digests, ordering, gaps, duplicates, and result attribution | Property | P0 | FR-012-AC-1, FR-012-AC-3 | 🚧 planned |
| TC-052 | Compare evaluator and required-history analysis with the independent Rust oracle across both valid clocks and all semantic/resource boundaries | Property | P0 | FR-011-AC-1, FR-011-AC-2, FR-011-AC-3, FR-012-AC-2, FR-012-AC-4, FR-013-AC-3 | 🚧 planned |
| TC-053 | Refuse Previous, future/mixed graphs, invalid histories, duration/rounded/resampled clocks, missing captures, and non-total predicates without coercion | Integration | P0 | FR-011-AC-4, FR-012-AC-1, FR-012-AC-5 | 🚧 planned |
| TC-054 | Round-trip formula-v2 profiles and enforce v1 preservation, v1-to-v2 upgrade, guarded down-conversion, and profile/operator compatibility | Integration | P0 | FR-013-AC-1 | 🚧 planned |
| TC-055 | Parse and canonically format O/H/S/T in `tl-parse.clean-ascii/v3` with exact precedence, intervals, associativity, and spans | Integration | P0 | FR-013-AC-2 | 🚧 planned |
| TC-056 | Replay the paired corpus, every mutation, late-data/anchor/closure identity, rewrite rule, and external-target state | Integration | P0 | FR-012-AC-3, FR-012-AC-4, FR-012-AC-5, FR-013-AC-3, FR-013-AC-4 | 🚧 planned |
| TC-057 | Fuzz formula-v2 decoding and v3-dialect parsing without unwind or profile misattribution | Fuzz | P1 | FR-013-AC-1, FR-013-AC-2 | 🚧 planned |
| TC-058 | Run the dependency-manifest gate over every implementation ticket owner, predecessor, M0 gate, and MRS acceptance prerequisite | Integration | P0 | FR-013-AC-5 | 🚧 planned |
