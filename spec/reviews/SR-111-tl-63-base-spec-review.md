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

On 2026-09-29, `quoin review` created run `tl63-source-binding` using the
released plugin's review workflow version 0.2.1, definition SHA-256
`edadae3cc5d5b89f7185530b8af60dab1ade4b63485bb389993bc2abaaca1513`.
The AP-002 selection and all eight validated document paths were recorded,
and the run advanced to `validated`. Human acceptance was not advanced.
The disposable ix-flow run state is under `target/spec-workflow`; this
document records the procedure and does not substitute for retained evidence.

## Findings

| ID | Severity | Summary | Refs | Escape Cause |
| --- | --- | --- | --- | --- |
| FND-8001 | medium | TC-060 and TC-069 are allocated but remain planned; the draft TL-63 unit probe is partial evidence and cannot close the criterion. | FR-006-AC-9, TM-004 | correct-requirement-no-evidence |
