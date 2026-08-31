---
id: SR-006
title: Exact-head qualification hardening review
type: SpecReview
analysis: code-review
scope: local CI, evidence qualification, traceability, and retained-record history
review_set: all
---

# Exact-head qualification hardening review

## Summary

The sixth and seventh review rounds identified false-green paths in tool
identity, compiled-test enumeration, Make controls, shell-gate wiring, retained
profiles, and traceability mappings. The remediation fixes the finding classes:
the qualification environment is source-locked by absolute path and SHA-256;
the Cargo-compiled, non-ignored test set must equal the requirement-tagged
source census; every Make control spelling is rejected; successful active
records require source-derived positive transcripts; and traceability tables
are cross-checked against requirement files and test-case mappings.

Retained evidence is collected outside the evidence tree, each record name is
cross-checked against its source revision and collection time, and Git history
is used to reject removal or post-introduction mutation of record manifests.
All records predating the complete active profile are preserved immutably but
explicitly retracted in `evidence/RETRACTIONS.json`; none remains an active
qualification claim.
The in-repository anchor remains a review-visible trust root rather than an
external attestation. Hosted CI remains manual-only and was not dispatched.

## Findings

| ID | Severity | Summary | Refs |
|---|---|---|---|
| FND-601 | high | PATH shadowing, shell-gate gutting, excluded tests, Make execution controls, empty passing transcripts, disabled per-record validators, and a zero-active evidence census could produce false-green evidence; fixed-path checks, an exact active/retracted census, and behavioral shell-gate probes now reject each route. | NFR-002, TC-018 |
| FND-602 | medium | Gate entry points, historical parameter derivation, record-set identity, current inspections, and verification-method provenance were incomplete; source-revision and Git-history checks now bind them. | NFR-002, StR-001, StR-002, AA-001 |
| FND-603 | low | Collection staging, non-test assertions, finding identifiers, review artifacts, validator environments, and matrix cell completeness required hardening or clarification. | FR-005, NFR-002, AA-001 |

## Residual boundary

Git history and pull-request review are the authority for the committed anchor
and record-history checks. They detect deletion, renaming, cloning with a false
timestamp or revision, and mutation after introduction, but they do not provide
an independent timestamp or notarization. AA-001 therefore remains open for the
human source-release decision.
