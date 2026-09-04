---
id: SR-017
title: Composite review of tracked source census hardening
type: SpecReview
analysis: base
scope: "agent-ix/tl-syntax#17 author specification candidate 130c521; FR-006-AC-6/AC-7; NFR-002 retired criteria; TC-026/TC-034; SR-013"
review_set: all
relationships:
  - target: ix://agent-ix/tl-syntax/FR-006
    type: reviews
  - target: ix://agent-ix/tl-syntax/PLAN-004
    type: references
---

# SR-017: Composite review of tracked source census hardening

## Summary

This author-performed pre-implementation composite review translated the
independent PR #14 findings TS14R3-01 through TS14R3-05 into specification work.
It did not grant closure to those external findings; only a later independent
exact-head review may do that. The change remains a repository-owned test of
repository content and introduces no runner, collector, Make parser, evidence
envelope, retention layer, or duplicate shared contract.

## Findings

| ID | Severity | Summary | Refs | Escape Cause |
|---|---|---|---|---|
| FND-1701 | high | TS14R3-01: counting a recursive filesystem walk makes ignored proptest regression seeds part of the exact reviewed population even though Git reports a clean tree. | FR-006-AC-7, TC-034 | wrong-requirement |
| FND-1702 | medium | TS14R3-03 cardinality half: exact total equality alone cannot detect a compensating deletion and addition in different source areas. | FR-006-AC-7, TC-034 | correct-requirement-no-evidence |
| FND-1703 | medium | TS14R3-03 root half: optional root-file enumeration fails open and identifies only a changed total when a required root disappears. | FR-006-AC-7, TC-034 | implementation-bug-despite-evidence |
| FND-1704 | medium | TS14R3-04: a diagnostic that reports only the total sends maintainers back to the manual recount that produced repeated wrong constants. | TC-034 | missing-requirement |
| FND-1705 | low | TS14R3-02: the NFR-002 successor sentence names a retired criterion and assigns a deleted per-record-validator subject to live intake criteria. | NFR-002 | wrong-requirement |
| FND-1706 | low | TS14R3-05: SR-013 does not distinguish its local bracketed scenario label from the Quoin fields its predicate reads. | SR-013 | missing-requirement |

## Dispositions

| Finding | Disposition | Evidence |
|---|---|---|
| FND-1701 | **SPECIFIED; EXTERNAL CLEARANCE REQUIRED** | The planned population derives from version-control paths; non-ignored untracked files are separate and ignored files cannot enter either set. |
| FND-1702 | **SPECIFIED; EXTERNAL CLEARANCE REQUIRED** | TC-034 requires both an exact path set and per-area cardinalities. |
| FND-1703 | **SPECIFIED; EXTERNAL CLEARANCE REQUIRED** | Every non-archival tracked path must appear in the exact live set. |
| FND-1704 | **SPECIFIED; EXTERNAL CLEARANCE REQUIRED** | The exact expected and observed path sets identify the delta. |
| FND-1705 | **AUTHOR REMEDIATED; EXTERNAL CLEARANCE REQUIRED** | The text enumerates each retired clause without blanket succession. |
| FND-1706 | **AUTHOR REMEDIATED; EXTERNAL CLEARANCE REQUIRED** | SR-013 states the local-label and audited-receipt boundary. |

## Falsifiability and controls

- A scratch Git repository proves tracked files enter the population,
  non-ignored untracked files enter only the scan, and ignored generated files
  enter neither.
- Running the helper outside a Git repository must fail with the census-specific
  unable-to-enumerate diagnostic rather than return an empty set.
- The exact non-archival path set and independently authored area cardinalities
  are checked separately, so missing, new, renamed, or compensating paths name
  their own delta rather than hiding behind an aggregate.
- The live repository census scans every readable selected path and fails with
  the path named when a selected path cannot be read.

## Boundary

The test establishes source-set enumeration and absence of deleted local
machinery. It does not qualify Git, GNU Make, Quire, Quoin, a producer, or a
consumer, and it does not close the separately tracked Make execution-control
class. Hosted CI remains manual-only.

SR-014 through SR-016 and PLAN-003 were already allocated by the concurrent
issue #15 branch when this issue began. This branch therefore starts at SR-017
and PLAN-004; the gap is reservation, not a missing record.

## Conclusion

The specification is sufficiently bounded for implementation. This author
review grants no merge authority; independent exact-head clearance remains
required.
