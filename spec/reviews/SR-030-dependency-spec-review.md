---
id: SR-030
title: "Dependency review of the complete tl-syntax specification corpus"
type: SpecReview
analysis: dependency
scope: "spec/"
review_set: all
---

## Summary

The plans sequence checked syntax values, wire documents, assurance intake,
source census, and qualification ownership without a repository-local evidence
layer. The outstanding dependency is external and explicit: the released
Engineering Assurance compatibility artifact must identify the released Quire
assurance export before declaration/source binding can be implemented.

## Findings

| ID | Severity | Summary | Refs | Escape Cause |
| --- | --- | --- | --- | --- |
| FND-3001 | medium | The #16 source/scope migration is blocked on a released Engineering Assurance compatibility artifact for Quire 0.32.0; a branch pin or local matrix would violate the declared dependency boundary. | tl-syntax#16, quire-cli#74, NFR-003 | correct-requirement-no-evidence |
