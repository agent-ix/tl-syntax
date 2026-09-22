---
id: SR-103
title: "Base specification review — syntax owner reconciliation"
type: SpecReview
analysis: base
scope: "FR-013, FR-014, ADR-003, IF-004, VO-006, TM-001, TM-003 and PLAN-010 Task-009 tl-syntax allocation"
review_set: base
---

# Base specification review — syntax owner reconciliation

## Summary

Reviewed the focused syntax-owner requirements and TC-075 allocation against
the base specification checklist after implementation exposed an inconsistency
between the accepted formula-v2 contract and the new reconciliation wording.
IDs, dependencies, error domains, resource boundaries and all five acceptance
criteria are now consistent and executable.

## Verdict

**PASS after remediation.** This is a focused correction within the already
reviewed ecosystem architecture, not a new system design; the seven architecture
lenses remain the accepted Task-008 review set.

## Findings

| ID | Severity | Summary | Refs | Escape Cause |
|---|---|---|---|---|
| FND-6601 | high | **FIXED:** FR-014 and ticket #65 had narrowed the already-published profile-partitioned formula-v2 identity to past-only, contradicting FR-013 and ADR-003's successor-identity rule. FR-014/TC-075 now preserve future-profile v2 graphs, admit origin-complete past graphs, and refuse cross-profile mixtures. | FR-013; FR-014-AC-3/4; ADR-003 | wrong-requirement |
| FND-6602 | medium | **FIXED:** TC-075 was still marked planned after its complete implementation; both governing matrices now mark all FR-014 criteria covered and name the executable test module. | TM-001; TM-003; TC-075 | correct-requirement-no-evidence |

## Base checklist result

- FR-014 has one stable ID, explicit owner inputs/outputs, closed versions and
  profiles, typed refusal classes, measurable byte/depth/string/population/work
  ceilings, and owner/dependency boundaries.
- Every FR-014 acceptance criterion has a TC-075 production-path trace. Exact
  maxima and one-over/lowered boundaries, malformed fields, versions, topology,
  domains, bindings, profile partitions and no-std features are exercised.
- `quire validate --scope . 'spec/**/*.md' --strict --summary` reports 204/204
  grammar-clean documents; FR-014 coverage is 5/5.
