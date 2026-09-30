---
id: SR-113
title: TL-63 source-binding integrity specification review
type: SpecReview
analysis: integrity
scope: "FR-006-AC-9, TM-001, TM-004"
review_set: all
---

## Summary

The criterion keeps source identity in the sealed Quoin record and makes the
unsealed lookup nonauthoritative. It does not grant authority to top-level
metadata or a declared scope without a complete footprint check. TC-060 and
TC-069 trace the adverse cases without marking them implemented.

## Findings

| ID | Severity | Summary | Refs | Escape Cause |
| --- | --- | --- | --- | --- |
| FND-8003 | low | No conflicting normative source of path identity was found in this delta; the existing legacy intake remains a separate migration dependency. | FR-006, TL-23 | correct-requirement-no-evidence |
