---
id: SR-019
title: Closing gap analysis of tracked source census hardening
type: SpecReview
analysis: gap-analysis
scope: "agent-ix/tl-syntax#17 candidate 1a00573; FR-006-AC-6; TC-026; retained PR #14 findings"
review_set: all
relationships:
  - target: ix://agent-ix/tl-syntax/FR-006
    type: reviews
  - target: ix://agent-ix/tl-syntax/PLAN-004
    type: references
---

# SR-019: Closing gap analysis of tracked source census hardening

## Summary

No implementation gap remains inside issue #17. The source population is now
Git-derived, ignored generated files cannot perturb it, untracked live sources
remain scanned, required roots fail distinctly, and both the exact total and
per-area populations are checked. The two retained prose defects are corrected.

## Requirement census

| Obligation | Evidence | Gap |
|---|---|---|
| FR-006-AC-6 tracked population | Git tracked-set helper plus exact 42-file assertion | none |
| FR-006-AC-6 broader live scan | non-ignored-untracked union and three-class fixture | none |
| Required roots | seven directory checks plus seven tracked root-file checks | none |
| Compensating cross-area changes | independent eight-entry area-cardinality map | none |
| Unable to enumerate or read | non-repository control and path-naming read failure | none |
| NFR-002 retirement statement | five live criteria named without blanket succession | none |
| SR-013 scenario label | local expected label separated from audited Quoin predicate | none |

## Findings

| ID | Severity | Summary | Refs | Escape Cause |
|---|---|---|---|---|
| FND-1901 | low | The shared contract still does not bind source paths or prove declared subject-scope completeness. | agent-ix/tl-syntax#16 | missing-requirement |
| FND-1902 | low | The common Make execution-control qualification class remains open after the documented local guard removal. | agent-ix/engineering-assurance#11 | missing-requirement |

## Dispositions

| Finding | Disposition | Evidence |
|---|---|---|
| FND-1901 | **DEFERRED** | #16 names path substitution, false scope, gate configuration, corpus, and shared-contract consumption; no local envelope or duplicate contract is introduced here. |
| FND-1902 | **DEFERRED** | The class is owned by shared Engineering Assurance qualification work. Issue #17 changes no Make parsing or execution behavior. |

## Compatibility and downstream impact

The crate API, formula/corpus wire bytes, dependency graph, MSRV, license, and
publication policy are unchanged. The next feature branch must merge this
reviewed change and update its exact population from Git after its own tracked
files are present; it must not restore the recursive filesystem walk.

## Conclusion

The candidate has no unresolved high or medium gap. After exact-final-head local
verification and explicit reviewer clearance, issue #17 can land ahead of
tl-syntax#15. Hosted CI remains manual-only.
