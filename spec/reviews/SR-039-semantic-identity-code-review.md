---
id: SR-039
title: "Code review — semantic formula identity"
type: SpecReview
analysis: code-review
scope: "src/syntax.rs, src/document.rs, src/lib.rs, tests/integration.rs, spec/requirements/FR-003-identities-and-profiles.md, spec/test-matrix.md"
review_set: subset
---

# Code review — semantic formula identity

## Summary

The semantic identity API excludes diagnostic spans from equality, ordering,
hashing, and semantic serialization while retaining spans in diagnostic wire
records. No local assurance implementation or CI trigger changes are included.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-3901 | low | No defect found: manual `Node` traits and semantic serialization cannot incorporate `span`; TC-037 fails if span-sensitive identity returns. | FR-003-AC-4, TC-037 |
| FND-3902 | medium | Existing local Python assurance scripts remain LR04/LR03 shared-migration work and are not copied or extended here. | quire-research#60 |
