---
id: SR-014
title: "Code review — semantic formula identity"
type: SpecReview
analysis: code-review
scope: "src/syntax.rs, src/document.rs, src/lib.rs, tests/integration.rs, spec/requirements/FR-003-identities-and-profiles.md, spec/test-matrix.md"
review_set: subset
---

# Code review — semantic formula identity

## Summary

This review examined the semantic-identity change against FR-003-AC-4, its
public Rust API, serde boundary, no-std/alloc feature boundary, and the new
integration control. Source spans remain wire-visible diagnostic provenance,
but equality, ordering, hashing, and the new semantic serialization view now
exclude them. No defect was found in the changed Rust surface.

## Verdict

**CONDITIONAL** — the changed semantic-identity implementation and its source
gates pass; the pre-existing local Python assurance-chain surface remains a
separate shared-assurance migration concern outside this patch.

## Assurance Context

AP-001 (`spec/assurance/AP-001.md`) applies because this change directly
addresses its `impact-semantic-misidentification` scenario. The evaluated
baseline is `a6d58aa` and the reviewed change set is the paths in this review's
scope. Available context was FR-003, TM-001, AP-001, the repository safety
conventions, and the changed Rust/test/specification paths. No architecture
description or current Quoin record specific to this uncommitted candidate was
available; five shared-assurance tests therefore remain unrun in this fresh
worktree because their required pinned environment and producer inputs are
absent. No AP-001 exception applies. The repository's existing Python
assurance-chain scripts were observed but not changed or treated as a local
replacement in this work; their migration belongs to the shared
Quoin/Quire/Engineering-Assurance program.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-1401 | low | No defect found in the changed Rust surface: manual `Node` traits consistently use `NodeKind`, the semantic serializer cannot reach `span`, and TC-037 fails if span-sensitive identity or serialization returns. | src/syntax.rs:203, src/document.rs:56, tests/integration.rs:143, FR-003-AC-4 |
| FND-1402 | medium | Pre-existing repository-local Python assurance scripts remain outside this patch. They must be resolved through the shared assurance migration rather than copied or extended in this Rust change. | scripts/assurance_chain.py:1, scripts/check_shared_pins.py:1, CLAUDE.md:27 |
