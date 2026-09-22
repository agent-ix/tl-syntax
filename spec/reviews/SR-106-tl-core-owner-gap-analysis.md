---
id: SR-106
title: "Gap analysis — syntax owner reconciliation"
type: SpecReview
analysis: gap-analysis
scope: "FR-014 and PLAN-010 Task-009 tl-syntax allocation"
review_set: subset
---

# Gap analysis — syntax owner reconciliation

## Summary

Traced every FR-014 criterion and `tl-syntax#65` deliverable through the final
production modules, immutable artifacts and executable TC-075 tests. This
review covers the tl-syntax allocation only; Task-009 correctly remains open
for tl-parse, tl-mltl and tl-rewrite.

## Verdict

**COMPLETE for the tl-syntax allocation after remediation.** No owner feature,
test, trace, or compatibility correction is deferred.

## Requirement-to-evidence map

| Obligation | Production evidence | Test evidence | Status |
|---|---|---|---|
| Four immutable schemas/digests | Formula constants in `formula::document`; signal/map constants in `signal::document` | independent SHA-256 recomputation in TC-075 | complete |
| Bounded canonical strict readers | `contracts::reader`; `SyntaxArtifactLimits`; three public document reader APIs covering four formula selections | canonical/malformed/boundary TC-075 cases | complete |
| Preserve accepted behavior and paths | semantic module tree plus crate-root re-exports; unchanged v1 and profile-partitioned v2 validation | existing corpus/profile suites plus TC-075 import/byte checks | complete |
| Closed formula-v2 profile partition | `Formula::new`; `FormulaDocument::new_v2/from_json_bytes` | every O/H/Y/S/T kind, both future profiles and cross-profile refusals | complete |
| Allocation-free default | serde/SHA dependency feature gating; borrowed graph/catalog/binding APIs | feature matrix and Rust 1.75 builds | complete |
| Content identity separation | public domain prefix and contract-specific hash input | independent identity recomputation and schema/content inequality | complete |

## Findings

| ID | Severity | Summary | Refs | Escape Cause |
|---|---|---|---|---|
| FND-6901 | medium | **FIXED:** TC-075 originally lacked a direct FR-014-AC-5 trace even though the feature test existed; the feature-boundary test now carries the exact criterion and case tags. | TC-075; FR-014-AC-5 | correct-requirement-no-evidence |
| FND-6902 | medium | **FIXED:** proposition-map implementation still lived in the formula document module after the first structural split. It now has one private signal-owned implementation re-exported through `signal::document` and the stable crate root. | ADR-003; `signal::proposition`; `signal::document` | implementation-bug-despite-evidence |
| FND-6903 | medium | **FIXED:** generic strict-reader and canonicalization policy initially remained coupled to the formula document module. It now lives under `contracts::reader` and is consumed by all owner documents. | ADR-003; `contracts::reader` | implementation-bug-despite-evidence |

Quire reports FR-014 at 5/5 backed criteria and all Rust evidence symbols
tagged. Repo-wide rows assigned to other repositories or the retired
qualification lane are outside this allocation and were neither claimed nor
modified.
