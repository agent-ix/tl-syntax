---
id: SR-022
title: Source-census reproducibility code review
type: SpecReview
analysis: code-review
scope: "agent-ix/tl-syntax#20 implementation candidate 1b6c1b5; changes from landed base ba292615; FR-006-AC-7/AC-8; TC-034/TC-035"
review_set: all
relationships:
  - target: ix://agent-ix/tl-syntax/FR-006
    type: reviews
  - target: ix://agent-ix/tl-syntax/PLAN-005
    type: references
---

# SR-022: Source-census reproducibility code review

## Summary

The implementation candidate closes all five nonblocking PR #18 findings
without adding a runner or evidence layer. One reproducibility escape found
during this author review was fixed before this record: active tracked
`.gitignore` bytes are now required to match the index. No unresolved high or
medium finding remains. This author review grants no merge authority.

## Review coverage

| Surface | Result |
|---|---|
| Git enumeration | Returns explicit errors; the repository-contained non-repository control still proves the ceiling is load-bearing. |
| Ignore semantics | Uses Git's per-directory `.gitignore` parser without importing standard workstation/administrative excludes; untracked and index-divergent ignore policy is refused. |
| Content scan | Reads arbitrary bytes and searches exact forbidden ASCII byte sequences with path-specific errors. |
| Partition diagnostics | Synthetic cross-area and within-area substitutions prove the area diagnostic runs first and exact paths remain authoritative. |
| Negative controls | Expected refusals are ordinary `Result` values, so passing tests no longer emit panic-hook output. |
| Architecture | No production crate code, dependency, Python helper, Make target, evidence contract, or hosted workflow changes. |

## Findings

| ID | Severity | Summary | Refs | Escape Cause |
|---|---|---|---|---|
| FND-2201 | medium | Candidate `ae16c3b` allowed unstaged edits to a tracked `.gitignore` to change which files the same indexed revision scanned. | FR-006-AC-8, TC-035 | implementation-bug-despite-evidence |
| FND-2202 | low | Git path identities remain required to be UTF-8 even though selected file contents no longer are. | FR-006-AC-7, `git_files` | missing-requirement |

## Dispositions

| Finding | Disposition | Evidence |
|---|---|---|
| FND-2201 | **AUTHOR REMEDIATED; EXTERNAL CLEARANCE REQUIRED** | Candidate `1b6c1b5` runs `git diff --quiet` over every `.gitignore` path and TC-035 mutates a tracked rule before proving the specific refusal. |
| FND-2202 | **ACCEPTED** | FR-006-AC-8 concerns arbitrary file content bytes. Repository-relative path strings were already an explicit fail-closed UTF-8 boundary in PR #18 and are not silently skipped. |

## Finding-by-finding closure proposed to the independent reviewer

- **TS18B-01:** `validate_tracked_partition` checks the area map first; one
  cross-area mutation reaches that error, while a within-area mutation reaches
  exact path equality.
- **TS18B-02:** `fs::read` plus byte-window matching accepts a benign non-UTF-8
  file and detects the same file after a forbidden ASCII sequence is appended.
- **TS18B-03:** FR-006-AC-7 and its rationale now state that an ordinary
  untracked source is scanned, named, and refuses a clean reviewed population.
- **TS18B-04:** TC-035 configures both external exclude sources and proves their
  paths remain visible; tracked-only, index-matching `.gitignore` is the sole
  generated-path policy.
- **TS18B-05:** neither negative control uses `catch_unwind`; helper refusals are
  inspected as returned errors.

## Verification observed

Focused TC-034, TC-035, and TC-026 runs pass. Strict Quire validation reports
65/65 documents grammar-clean; strict coverage backs all 26 matrix cases, all
seven FR-006 criteria, and 29/29 Rust trace symbols. The exact final-head full
local gate remains for the post-review-record commit. Hosted CI was not
dispatched.

## Conclusion

The candidate is ready for closing gap analysis and exact-final-head local
verification. Independent review must decide whether the external findings are
closed.
