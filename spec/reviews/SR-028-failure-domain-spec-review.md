---
id: SR-028
title: "Failure-domain review of the complete tl-syntax specification corpus"
type: SpecReview
analysis: failure-domain
scope: "spec/"
review_set: all
---

## Summary

The review covered source identity, external-tool, producer-boundary, and graph
failure modes. Existing formula and source-census requirements fail closed, but
the shared source/scope contract has not yet supplied the required released
capability for the remaining declaration-integrity boundary.

## Findings

| ID | Severity | Summary | Refs | Escape Cause |
| --- | --- | --- | --- | --- |
| FND-2801 | medium | A substituted source path or incomplete declared subject scope cannot yet be rejected by the released shared contract; the required Quire/Engineering-Assurance migration remains tracked rather than locally reconstructed. | tl-syntax#16, FR-006, NFR-003 | correct-requirement-no-evidence |
