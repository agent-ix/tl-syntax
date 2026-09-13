---
id: SR-058
title: "Rust/code review — formula-v2 and past syntax"
type: SpecReview
analysis: code-review
scope: "src/syntax.rs, src/document.rs, src/future.rs, src/lib.rs, tests/past_formula_v2.rs, tests/past_profile_manifest.rs, tests/integration.rs, tests/future_operator_corpus.rs"
review_set: subset
---

# Rust/code review — formula-v2 and past syntax

## Summary

The Task-001 implementation is idiomatic, allocation-free in the borrowed core,
bounded at owned decode/construction boundaries, strict about schema/profile
compatibility, panic-free in production code, and free of unsafe code. Formula-v1
serialization and admission remain unchanged for previously valid documents.
Every implementation finding below was fixed before this review was closed.

## Verdict

**PASS** — no open Rust or code-review finding remains in Task-001 scope.

## Findings

| ID | Severity | Summary | Refs |
|---|---|---|---|
| FND-5801 | high | The first implementation applied the new depth limit to formula-v1 and would have changed a previously admitted v1 graph. Fixed by applying `MAX_FORMULA_DOCUMENT_DEPTH` only to v2 and adding a paired v1 compatibility control. | `src/document.rs`, `tests/past_formula_v2.rs`, FR-013-AC-1 |
| FND-5802 | high | The existing future-lowering request initially discovered profiles through the newly expanded global profile catalog, allowing an origin-history Boolean graph to receive future nodes. Fixed by retaining the future-lowering/v1 profile set as exactly closed-trace/v1 and online-prefix/v1 and testing that the new profile refuses. | `src/future.rs`, `tests/past_formula_v2.rs`, FR-013-AC-1 |
| FND-5803 | medium | A defensive down-conversion path initially used a sentinel node identity if a `usize` conversion failed. Fixed with checked conversion and a typed `NodeIdentityOutOfRange` refusal; no wire or persistence boundary truncates an integer. | `src/document.rs` |
| FND-5804 | medium | New implementation tests were initially outside the fail-closed live-source census. Fixed by adding both exact tracked paths and updating the tests-area population; the mutation-backed census test passes. | `tests/shared_assurance.rs`, TC-058 |
| FND-5805 | low | New enum variants made two future-only test helpers non-exhaustive. Fixed with explicit past-node handling; no wildcard can hide a later node addition. | `tests/integration.rs`, `tests/future_operator_corpus.rs` |

## Gate Results

- Rustfmt and strict Clippy over all targets/features: pass.
- no-default, alloc, serde, and all-feature checks: pass.
- Rust 1.75: every implementation, compatibility, and feature test passes.
- Rustdoc with warnings denied and unsafe-comment audit: pass.
- Cargo Deny advisories, bans, licenses, and sources: pass; unmatched license allowances are warnings only.
- Full shared-assurance execution refuses because the host tool versions do not match the repository pins; no check was weakened or reclassified.
