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

The explicit semantic identity API excludes diagnostic spans from semantic-view
equality, ordering, hashing, and serialization while retaining spans in the
public structural `Node`/`FormulaDocument` contracts and diagnostic wire records.
No local assurance implementation or CI trigger changes are included.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-3901 | high | Replacing public structural `Node` traits with kind-only traits silently changes downstream collection semantics. Fixed by retaining derived structural traits and placing span-insensitive identity on `SemanticFormulaDocument`. | src/syntax.rs, src/document.rs, FR-003-AC-4 |
| FND-3903 | medium | The hand-written semantic serializer's v1 field shape was unproven. Fixed by decoding its bytes through the public `FormulaDocument` v1 boundary, validating them, and proving the decoded document has no spans but an equal semantic view. | src/document.rs, tests/integration.rs, TC-037 |
| FND-3902 | medium | Existing local Python assurance scripts remain LR04/LR03 shared-migration work and are not copied or extended here. | quire-research#60 |
