---
id: SR-005
title: Exact-head review remediation of tl-syntax v0.1
type: SpecReview
analysis: code-review
scope: current pull-request source, specification, policy gates, and evidence tooling
review_set: all
---

# Exact-head review remediation of tl-syntax v0.1

## Summary

The exact-head review identified four blocking policy defects plus medium- and
low-severity hardening opportunities. The candidate now behaviorally proves
failure propagation for every mandatory local gate, removes the dry-run bypass,
resolves every criterion target against minted tests, and refuses to claim JSON
Schema validity when a required format checker is unavailable.

The same remediation also derives corpus outcomes independently, executes the
no-std feature boundary, bounds formula wire allocation, declares the mandatory
semantic-schema boundary, uses stable rustfmt settings, rejects orphan evidence
directories, auto-anchors collected manifests, distinguishes unavailable tools,
and documents the Git/PR-review trust boundary. Local `make ci`, Rust 1.75 tests,
and behavioral mutation checks are required again before exact-candidate
evidence is retained. Hosted CI remains manual-only.

## Findings

| ID | Severity | Summary | Refs |
|---|---|---|---|
| FND-501 | high | Failure propagation, bypass, fabricated targets, and optional format enforcement could produce false green results; executable policy and mutation tests now close those paths. | NFR-002, TC-018 |
| FND-502 | medium | Feature, corpus, formatting, verification-method, and diagnostic controls were incomplete; executable boundary tests and stricter policy parsing close them. | NFR-001, FR-005, TC-019 |
| FND-503 | low | Evidence anchoring, documentation, API ownership, semantic-schema disclosure, and decode allocation needed hardening; exact-candidate collection and the human release decision remain separate. | FR-001, FR-003, FR-004, MP-001, AA-001 |
