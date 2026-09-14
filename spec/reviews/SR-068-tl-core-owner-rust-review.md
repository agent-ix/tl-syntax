---
id: SR-068
title: "Rust review — syntax owner reconciliation"
type: SpecReview
analysis: code-review
scope: "agent-ix/tl-syntax#65 Rust implementation and TC-075"
review_set: subset
---

# Rust review — syntax owner reconciliation

## Summary

Applied the Rust review checklist to the module split, public contracts,
untrusted JSON admission, allocation boundaries, integer conversions,
dependency feature graph, error surface and traced tests.

## Verdict

**PASS after remediation.** No unresolved Rust finding remains.

## Findings

| ID | Severity | Summary | Refs | Escape Cause |
|---|---|---|---|---|
| FND-6801 | medium | **FIXED:** catalog validation used production `expect` calls and lossy `as usize` scratch conversions. It now uses checked conversions, typed invariant refusals and fail-closed optional lookup. | `signal::catalog` | implementation-bug-despite-evidence |
| FND-6802 | medium | **FIXED:** caller population budgets were first checked only after deserialization. A quote-aware structural preflight now charges target arrays and total deterministic work before typed retention, with bounded serde visitors retained as defense in depth. | `contracts::reader`; TC-075 | implementation-bug-despite-evidence |
| FND-6803 | low | **FIXED:** a resource-validation trait returned a work count after work had moved to preflight. Its unused value was removed so the trait expresses only validation. | `StrictDocument` | implementation-bug-despite-evidence |

## Rust checklist

| Area | Result |
|---|---|
| Panic/unsafe | No production `unwrap`, `expect`, panic, unsafe block, or unsafe dependency feature added. |
| Integer boundaries | Wire identities remain checked `u32` newtypes; every `u32`/`usize` boundary is fallible and typed. |
| Resource bounds | Bytes, decoded string size, JSON depth, populations and deterministic work use saturating arithmetic and caller-lowered owner maxima. |
| Feature discipline | Default remains `no_std` and allocation-free; `alloc` compiles independently; serde and SHA-256 are optional together. |
| Ownership seams | Public documents own validated values; the shared reader and wire helpers stay crate-private. No callbacks, trait objects, locks, async, or I/O were introduced. |
| Tests | TC-075 exercises real public APIs, independent digests, exact boundaries and semantic mutations; existing corpus/profile suites remain green. |

## Gates

- `cargo fmt --all -- --check`
- `cargo clippy --all-targets --all-features -- -D warnings`
- current and Rust 1.75 feature checks and non-qualification test suites
- warning-free rustdoc, unsafe audit, corpus digest/oracle replay, and cargo-deny
