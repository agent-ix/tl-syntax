---
id: SR-038
title: "Code review — Rust wire-decoding fuzz target"
type: SpecReview
analysis: code-review
scope: "fuzz/, FR-004-AC-5, TC-036"
review_set: base
---

## Summary

Reviewed the isolated Rust fuzz crate and its public-boundary target against
FR-004-AC-5. The target independently decodes the same byte slice through both
public versioned document types and has one unconditional assertion per
boundary: accepted documents validate; rejected bytes are an expected result.

## Verdict

**PASS** — no unresolved Rust, boundary, dependency, panic-surface, or
automatic-execution finding remains in the reviewed fuzz-target scope.

## Assurance Context

`AP-001` (`spec/assurance/AP-001.md`) applies to this v0.1 source-candidate
slice because malformed graph admission is a named material impact scenario.
The reviewed baseline is `3888f6a` plus this staged wire-fuzz change. The
candidate has no formed change-assurance record, no campaign-duration or
coverage measurement, and no human decision; none is implied by this target.
No exception is active. The target reinforces the existing checked public
validation boundary and creates no local evidence representation.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-3801 | low | No findings. The fuzz crate is isolated from the published library dependency graph, uses only public serde document boundaries, has no unsafe code or added production API, and is manual-only. | fuzz/Cargo.toml, fuzz/fuzz_targets/wire_decode.rs |

## Gates

- `cargo +nightly clippy --manifest-path fuzz/Cargo.toml --all-targets -- -D warnings` passed.
- `cargo +nightly fuzz run wire_decode -- -runs=100` passed.
- `quire coverage --scope . --strict` bound FR-004-AC-5 and TC-036.
