---
id: SR-093
title: "Independent failure-domain review of progressive tl-syntax source readiness"
type: SpecReview
analysis: failure-domain
scope: "MRS-004, FR-015..FR-019, NFR-004..NFR-005, AP-002, IT-001..IT-002"
review_set: all
---

## Summary

Independent review at `33678fa` exercised candidate substitution, stale output,
path and symlink races, event-set incompleteness, expiry authority, retention,
supersession, concurrent publication, typed decode and license-resolution
failures. The closed outcomes remain non-promoting and resource-bounded.

## Findings

| ID | Severity | Summary | Refs | Escape Cause |
| --- | --- | --- | --- | --- |
| FND-6701 | high | FR-015 previously allowed before/after pathname revalidation, which cannot detect change-then-restore substitution and did not require the identified bytes to be the consumed bytes. It now requires one descriptor-backed snapshot/capability and refuses pathname reopen. | FR-015-AC-5, FR-015-AC-6, IT-001-SC-05 | wrong-requirement |
| FND-6702 | high | Decision intake previously lacked immutable event-population identity, completeness/bounds and an authoritative expiry instant, permitting a favorable page or guessed expiry. The requirement now keeps incomplete/cross-snapshot/over-bound input open and binds expiry to a verified time authority and canonical ordering. | FR-018-AC-2, FR-018-AC-7, AP-002, IT-001-SC-07 | missing-requirement |
| FND-6703 | medium | Real failure evidence is unavailable until the released source-grounding, retention, policy/event and integrator-package contracts are admitted; a mock or local substitute cannot resolve this finding. | Task-018, IT-001, IT-002 | correct-requirement-no-evidence |
