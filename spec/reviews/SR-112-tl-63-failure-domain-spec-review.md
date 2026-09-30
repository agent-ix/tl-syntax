---
id: SR-112
title: TL-63 source-binding failure-domain specification review
type: SpecReview
analysis: failure-domain
scope: "FR-006-AC-9, FR-015, AP-002"
review_set: all
---

## Summary

The new criterion names path substitution, duplicate identities, changed
specification bytes, statement drift and export-premise mismatch as refusals.
FR-015 retains the separate dirty-root, symlink, mount and execution-race
failure domains; an incomplete scope is never a verified scope.

## Findings

| ID | Severity | Summary | Refs | Escape Cause |
| --- | --- | --- | --- | --- |
| FND-8002 | low | An export supplied as prior JSON is not fresh execution evidence; the draft adapter must continue to mark this open until FR-015 binds its producer. | FR-006-AC-9, FR-015-AC-5 | correct-requirement-no-evidence |
