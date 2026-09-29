---
id: SR-111
title: TL-63 source-binding base specification review
type: SpecReview
analysis: base
scope: "FR-006-AC-9; TM-001 and TM-004 TC-060, TC-069; TL-63"
review_set: all
---

## Summary

The proposed FR-006 delta assigns sealed path identity, source-export comparison,
authorial metadata and incomplete scope to one consumer behavior. Existing
FR-015 owns full candidate materialization and freshness. AP-002 selects base
plus all seven analyses for M6. This review occurred after the draft adapter
was written; it does not claim a pre-implementation review.

## Findings

| ID | Severity | Summary | Refs | Escape Cause |
| --- | --- | --- | --- | --- |
| FND-8001 | medium | TC-060 and TC-069 are allocated but remain planned; the draft TL-63 unit probe is partial evidence and cannot close the criterion. | FR-006-AC-9, TM-004 | correct-requirement-no-evidence |
