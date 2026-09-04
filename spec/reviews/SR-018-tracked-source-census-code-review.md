---
id: SR-018
title: Code review of tracked source census hardening
type: SpecReview
analysis: code-review
scope: "agent-ix/tl-syntax#17 candidate 1a00573; changes from a6d58aa4; FR-006-AC-6; TC-026; PLAN-004"
review_set: all
relationships:
  - target: ix://agent-ix/tl-syntax/FR-006
    type: reviews
  - target: ix://agent-ix/tl-syntax/PLAN-004
    type: references
---

# SR-018: Code review of tracked source census hardening

## Summary

The exact `1a00573` implementation closes the ignored-artifact, fail-open-root,
aggregate-only, and non-local-diagnostic defects identified by the final PR #14
review. It uses the same Git tracked/non-ignored-untracked split already used by
the sibling temporal repositories and adds no production code or generic
assurance machinery. No high or medium finding remains open.

## Review coverage

| Surface | Result |
|---|---|
| Enumeration | `git ls-files -z` defines the tracked population; `--others --exclude-standard` adds only non-ignored untracked scan inputs; nonzero Git status and non-UTF-8 paths fail closed. |
| Three-class control | An isolated repository proves one tracked source enters both sets, one ordinary untracked source enters only the scan, and an ignored proptest seed enters neither. |
| Repository boundary | `GIT_CEILING_DIRECTORIES` prevents the non-repository negative control from inheriting an ancestor repository; the control requires the census-specific refusal text. |
| Completeness | Seven source directories and seven root files are required; the 42-file tracked total and eight per-area cardinalities are independently asserted. |
| Scan | Every selected tracked or non-ignored untracked path is read with a path-naming failure and checked for all five deleted-machinery names. |
| Scope | The test remains TC-026 under FR-006-AC-6; spec reviews/plans and the declaring test remain deliberate exemptions. |

## Findings

| ID | Severity | Summary | Refs | Escape Cause |
|---|---|---|---|---|
| FND-1801 | low | Exact total and per-area cardinalities do not detect a same-area one-for-one path substitution. | FR-006-AC-6, TC-026 | missing-requirement |
| FND-1802 | low | The explicit seven-directory scope still excludes corpus and several gate-configuration paths; full subject-scope completeness remains outside this change. | agent-ix/tl-syntax#16 | missing-requirement |

## Dispositions

| Finding | Disposition | Evidence |
|---|---|---|
| FND-1801 | **ACCEPTED** | FR-006-AC-6 requires exact total and per-area populations, not a manifest of every path. A same-area replacement remains scanned for forbidden bytes; exact scope completeness is separately owned by #16. |
| FND-1802 | **DEFERRED** | Issue #16 explicitly requires gate-configuration and corpus scope coverage through the released shared contract. Expanding this local absence census would neither bind the change record nor close that shared-contract gap. |

## Verification observed

Focused TC-026 passed after its negative control was corrected to use a true
non-repository directory. Full local
`make ci CARGO_TARGET_DIR=target/cargo-review` passed at exact committed
candidate `1a00573`: 55/55 documents were grammar-clean, 53/59 rows were backed,
27/27 Rust trace symbols reconciled, every Rust/MSRV/corpus/supply-chain lane
passed, four adapter mutation probes detected, and the complete assurance chain
passed. Hosted CI was not dispatched.

## Conclusion

The implementation is ready for closing gap analysis and exact-final-head local
verification. External reviewer clearance remains required before merge.
