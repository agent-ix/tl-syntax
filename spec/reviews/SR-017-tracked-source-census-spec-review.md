---
id: SR-017
title: Composite review of tracked source census hardening
type: SpecReview
analysis: base
scope: "agent-ix/tl-syntax#17; FR-006-AC-6; NFR-002 retired criteria; TC-026; SR-013"
review_set: all
relationships:
  - target: ix://agent-ix/tl-syntax/FR-006
    type: reviews
  - target: ix://agent-ix/tl-syntax/PLAN-004
    type: references
---

# SR-017: Composite review of tracked source census hardening

## Summary

Dependency, risk, evidence, integrity, scope, failure-domain, and EARS review
found no unresolved blocking ambiguity. The change remains a repository-owned
test of repository content. It reuses Git as the authoritative tracked-path
producer and does not introduce a runner, collector, Make parser, evidence
envelope, retention layer, or duplicate shared contract.

## Findings

| ID | Severity | Summary | Refs | Escape Cause |
|---|---|---|---|---|
| FND-1701 | high | Counting a recursive filesystem walk makes ignored proptest regression seeds part of the exact reviewed population even though Git reports a clean tree. | FR-006-AC-6, TC-026 | wrong-requirement |
| FND-1702 | medium | Exact total equality alone cannot detect a compensating deletion and addition in different source areas. | FR-006-AC-6, TC-026 | correct-requirement-no-evidence |
| FND-1703 | medium | Optional root-file enumeration fails open and identifies only a changed total when a required root disappears. | FR-006-AC-6, TC-026 | implementation-bug-despite-evidence |
| FND-1704 | medium | A diagnostic that reports only the total sends maintainers back to the manual recount that produced repeated wrong constants. | TC-026 | missing-requirement |
| FND-1705 | low | The NFR-002 successor sentence names a retired criterion and assigns a deleted per-record-validator subject to live intake criteria. | NFR-002 | wrong-requirement |
| FND-1706 | low | SR-013 does not distinguish its local bracketed scenario label from the Quoin fields its predicate reads. | SR-013 | missing-requirement |

## Dispositions

| Finding | Disposition | Evidence |
|---|---|---|
| FND-1701 | **FIXED IN SPEC** | The population is derived only from `git ls-files`; non-ignored untracked files are scanned separately and ignored files cannot enter either set. |
| FND-1702 | **FIXED IN SPEC** | TC-026 binds an independently stated per-area cardinality map in addition to the exact total. |
| FND-1703 | **FIXED IN SPEC** | Every declared root path is required explicitly before it participates in the census. |
| FND-1704 | **FIXED IN SPEC** | The failure reports expected and observed total and per-area populations. |
| FND-1705 | **FIXED** | The text names the five live criteria and denies blanket succession; the validator clause retired with its subject. |
| FND-1706 | **FIXED** | SR-013 now states the local-label and audited-receipt boundary. |

## Falsifiability and controls

- A scratch Git repository proves tracked files enter the population,
  non-ignored untracked files enter only the scan, and ignored generated files
  enter neither.
- Running the helper outside a Git repository must fail with the census-specific
  unable-to-enumerate diagnostic rather than return an empty set.
- Required root paths and independently authored area cardinalities are checked
  separately from the exact total, so neither a missing root nor a cross-area
  compensating pair can hide behind the aggregate.
- The live repository census scans every readable selected path and fails with
  the path named when a selected path cannot be read.

## Boundary

The test establishes source-set enumeration and absence of deleted local
machinery. It does not qualify Git, GNU Make, Quire, Quoin, a producer, or a
consumer, and it does not close the separately tracked Make execution-control
class. Hosted CI remains manual-only.

## Conclusion

The specification is sufficiently bounded for implementation. No high or
medium finding remains open.
