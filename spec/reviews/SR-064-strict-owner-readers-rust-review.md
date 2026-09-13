---
id: SR-064
title: "Rust review — strict signal-catalog and proposition-map owner readers"
type: SpecReview
analysis: code-review
scope: "agent-ix/tl-syntax#61 Rust implementation and traced tests"
review_set: subset
---

# Rust review — strict signal-catalog and proposition-map owner readers

## Summary

Applied the Rust review checklist to the feature boundary, strict read path,
resource ceilings, public errors, serde visitors, and traced integration tests.

## Verdict

**PASS after remediation.** The implementation follows the repository's
`no_std`/feature boundaries and Rust idioms. Strict all-target/all-feature
Clippy, rustfmt, default-feature and all-feature builds, unsafe audit, and
cargo-deny pass.

## Review notes

| Area | Result |
|---|---|
| Panic and unsafe surface | No production `unwrap`, `expect`, panic, or unsafe code added. |
| Allocation and resource bounds | Bytes and nesting are rejected before serde allocation; both document populations use bounded visitors. Allocation failure propagates through serde without partial output. |
| Integer and wire boundaries | Counts remain `usize` and stable public ceilings; wire identities continue to decode through the existing checked `u32` newtypes. No lossy conversion was introduced. |
| Trait and ownership seams | One generic crate-private reader serves two public owner types; downstream callers receive validated owned documents rather than callbacks or mirrored structs. |
| Async, locks, blocking | No async runtime, locks, blocking bridge, or I/O was introduced. |
| Test quality | TC-028/029/032 tests call production readers and include positive controls plus single-fault negative boundaries. |

The depth preflight is deliberately only a resource guard: serde_json remains
the sole syntax and semantic decoder, so the scanner cannot create an alternate
acceptance path.

## Findings

| ID | Severity | Summary | Refs |
|---|---|---|---|
| FND-6401 | medium | The shared crate-private reader centralizes byte/depth bounds while each public owner type remains the semantic decoder. | `read_strict_document`; `from_json_bytes` |
| FND-6402 | medium | Bounded serde visitors stop both signal and proposition populations before an excess element is retained. | `deserialize_bounded_vec`; `deserialize_propositions` |
| FND-6403 | low | Public limits and errors are feature-gated with serde; the default crate remains allocation-free. | `src/lib.rs`; `tests/feature_boundary.rs` |
