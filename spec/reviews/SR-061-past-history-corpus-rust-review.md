---
id: SR-061
title: "Rust review — shared past/history corpus and manifest gate"
type: SpecReview
analysis: code-review
scope: "src/past_manifest.rs, src/lib.rs, Cargo.toml, tests/past_profile_manifest.rs, tests/past_history_corpus.rs"
review_set: subset
---

# Rust review — shared past/history corpus and manifest gate

## Summary

Applied the Rust review checklist to the production reader, feature graph, and
all new corpus replay tests.

## Verdict

**PASS.** Production parsing remains `no_std + alloc`, forbids unsafe code,
bounds bytes before deserialization, denies unknown fields, and returns a stable
non-panicking error envelope. Integer conversions and required-history additions
are checked. Panics and process execution occur only in trusted tests.

## Findings

| ID | Severity | Summary | Refs |
|---|---|---|---|
| FND-6101 | medium | The optional no-std serde feature now pulls only `serde_json` alloc support; default and alloc-only builds stay isolated. | `Cargo.toml`; `src/past_manifest.rs` |
| FND-6102 | medium | The reader now rejects bytes beyond 64 KiB before JSON allocation. | `PAST_PROFILE_MANIFEST_MAX_BYTES`; `tests/past_profile_manifest.rs` |
| FND-6103 | low | Dynamic failures preserve stable code, field, and optional DAG task identity. | `PastProfileManifestError` |

## Review checks

No unsafe, unchecked wire integer casts, blocking production calls, locks,
recursive untrusted parser, stubs, ignored tests, or tautological production
seams were found. Strict Clippy passes over all targets and features.
