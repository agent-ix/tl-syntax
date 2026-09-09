---
id: SR-037
title: "Base review — versioned document wire-decoding fuzz obligation"
type: SpecReview
analysis: base
scope: "FR-004, TM-001 TC-036, public formula and proposition-map decode boundaries"
review_set: base
---

## Summary

The fuzz scope is limited to untrusted bytes at the two public versioned
document decode boundaries. It neither claims a completed campaign nor extends
the production API; accepted documents must still satisfy their public
validation boundary and rejected inputs must not unwind.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-3701 | medium | A generic "wire decoding" target could omit one of the two exchanged document families. Fixed: FR-004-AC-5 names both formula-document and proposition-map decoder boundaries. | FR-004-AC-5 |
| FND-3702 | medium | Treating deserialization success alone as acceptance would permit a fuzz target to miss a validation bypass. Fixed: the criterion requires every accepted result to pass its public validation boundary. | FR-004-AC-5, FR-004-AC-3 |
| FND-3703 | low | A fuzz target is a capability, not evidence that a campaign has run for a particular time, corpus, or coverage level. The matrix records the runnable target only; campaign measurements remain a later explicit scope. | TC-036, agent-ix/tl-syntax#26 |

## Reviewed boundary

The target receives one byte slice and supplies it independently to the public
formula-document and proposition-map decoding operations. It asserts no
semantic result for invalid bytes: rejection is expected. It asserts that a
successful decode validates through the same public method downstream callers
use. The target does not invoke Quire, Quoin, Engineering Assurance, a parser,
an evaluator, a rewrite engine, or a local evidence mechanism.
