---
id: SR-023
title: Source-census reproducibility closing gap analysis
type: SpecReview
analysis: gap-analysis
scope: "agent-ix/tl-syntax#20 implementation candidate 1b6c1b5; FR-006; PLAN-005; source-census and shared-assurance boundaries"
review_set: all
relationships:
  - target: ix://agent-ix/tl-syntax/FR-006
    type: reviews
  - target: ix://agent-ix/tl-syntax/PLAN-005
    type: references
---

# SR-023: Source-census reproducibility closing gap analysis

## Summary

No high or medium implementation gap remains inside issue #20. The landed
tracked-path census now has reproducible ignore semantics, byte-safe content
inspection, independently reachable diagnostics, and quiet explicit refusal
controls. Remaining limitations are either deliberate fail-closed boundaries
or owned by existing shared-assurance tickets.

## Requirement census

| Requirement | Evidence | Gap |
|---|---|---|
| FR-006-AC-7 area-before-path diagnostics | Cross-area and within-area synthetic substitutions exercise distinct returned errors before the real exact-set check. | none |
| FR-006-AC-7 clean untracked policy | The existing forbidden-name untracked fixture reaches the scan consumer, and the live-tree delta must then be empty. | none |
| FR-006-AC-7 enumeration refusal | The repository-contained non-repository fixture returns the census-specific Git diagnostic without panic interception. | none |
| FR-006-AC-8 arbitrary bytes | A tracked non-UTF-8 input scans successfully, then returns a path/name-specific refusal after an ASCII forbidden identity is embedded. | none |
| FR-006-AC-8 ignore reproducibility | Workstation and administrative excludes cannot hide their named sources; untracked or index-divergent `.gitignore` policy is refused. | none |

## Findings

| ID | Severity | Summary | Refs | Escape Cause |
|---|---|---|---|---|
| FND-2301 | low | Repository-relative Git path bytes remain a strict UTF-8 boundary rather than a byte-preserving path model. | FR-006-AC-7 | missing-requirement |
| FND-2302 | low | SUITE-008 declares the local Rust command but explicitly remains outside Quoin attestation. | `spec/evidence/suites.md`, `tl-syntax#19` | correct-requirement-no-evidence |
| FND-2303 | low | The shared traceability module still reports its known status-column and empty-inspection diagnostics. | `tl-syntax#16`, `quire-contract-ir#21` | correct-requirement-no-evidence |

## Dispositions

| Finding | Disposition | Evidence |
|---|---|---|
| FND-2301 | **ACCEPTED** | Enumeration refuses rather than skips a non-UTF-8 path. Expanding repository path identity is not required to close the content-byte finding and needs its own specification. |
| FND-2302 | **DEFERRED** | Issue #19 already owns verification-suite identity and stable qualification without converting Quoin or Quire into a runner. |
| FND-2303 | **DEFERRED** | Existing tickets own the shared contract corrections; this repository adds no local traceability implementation. |

## Architecture and ownership audit

- The diff touches only FR-006, the test matrix, one existing Rust integration
  test, and plan/review records.
- Git performs path enumeration and `.gitignore` interpretation; the test does
  not implement an ignore parser.
- The Rust test consumes repository state and returns local assertions. It does
  not execute through Quoin, claim Quoin attestation, or retain an evidence
  artifact.
- No Python script, shell gate, Make target, schema, runner, collector,
  tool-identity registry, or alternate evidence contract is introduced.
- Hosted CI remains workflow-dispatch-only and was not dispatched.

## Conclusion

Issue #20 has no unresolved high or medium gap at the implementation candidate.
Exact-head full local verification and independent review remain before merge.
