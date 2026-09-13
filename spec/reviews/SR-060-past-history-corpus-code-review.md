---
id: SR-060
title: "Code review — shared past/history corpus and manifest gate"
type: SpecReview
analysis: code-review
scope: "Task-005 tl-syntax owner changes: corpus/past-history, src/past_manifest.rs, tests/past_history_corpus.rs, tests/past_profile_manifest.rs, Makefile"
review_set: subset
---

# Code review — shared past/history corpus and manifest gate

## Summary

Reviewed the Task-005 owner implementation against the accepted corpus,
authorization, compatibility, and resource-bound obligations.

## Verdict

**PASS after remediation.** The corpus is immutable, closed-schema validated,
digest pinned, and executable. The authorization gate is production code rather
than a test-local model. No qualification artifacts or foreign runtimes were
introduced.

## Findings

| ID | Severity | Summary | Refs |
|---|---|---|---|
| FND-6001 | high | The test-only reader was promoted to the bounded public API; tests now call only production validation. | `src/past_manifest.rs`; `tests/past_profile_manifest.rs` |
| FND-6002 | high | The initially open formula object was replaced with a closed formula-v2/past node union plus semantic validation. | `corpus/past-history/schema.json`; `tests/past_history_corpus.rs` |
| FND-6003 | medium | The unreachable predecessor-order branch was reordered and mutation tested. | `src/past_manifest.rs`; `tests/past_profile_manifest.rs` |
| FND-6004 | medium | The impossible arithmetic-overflow claim was replaced with an executable temporal-span refusal. | `corpus/past-history/cases.json`; `tl-mltl/tests/past_history_corpus.rs` |

## Gates

Rustfmt, strict all-target/all-feature Clippy, all non-qualification Rust tests,
the legacy corpus gates, the new checksum gate, and the Rust closed-wire reader
and replay tests pass.
