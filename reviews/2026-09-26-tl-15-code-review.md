---
id: SR-111
title: "TL-15 code review of infinite-trace syntax"
type: SpecReview
analysis: code-review
scope: "agent-ix/tl-syntax@e2ca5aabfe5a9cf00c8d9806e812854a5c4061c7; README.md; corpus/infinite-trace/*; corpus/schema/formula-unbounded-v1.schema.json; fuzz/*; src/formula/{document,graph,infinite,liveness,mod,profile,trace}.rs; src/lib.rs; tests/{infinite_formula,infinite_fuzz_seeds,infinite_trace,infinite_trace_corpus,shared_assurance,v1_spec_stubs}.rs"
review_set: subset
---

## Summary

Ticket: TL-15. Reviewed the feature diff from stacked base 4a5b5fd, including Rust, wire schema, corpus, fuzz seeds, and tests. Rust format, Clippy, and 33 focused tests passed.

## Verdict

**FAIL** — the alloc-only public fairness constructor accepts a false formula identity.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | Under the alloc-only feature set, fairness construction accepts any nonempty graph identity instead of binding to the supplied formula. | src/formula/trace.rs:688 |
| FND-002 | medium | TC-147's backend ignores the graph and subject, so the test cannot establish that a provider received the identities it claims to attribute. | tests/infinite_formula.rs:268 |

## Review Notes

FND-001 scenario: compile with `--no-default-features --features alloc`, construct a valid infinite formula, then call `FairnessPremisesDocument::new(&formula, "other-graph".into(), InfiniteClock::EventPosition, vec![])`. The only equality check is `#[cfg(feature = "serde")]`, so this succeeds and publishes a foreign graph identity. FR-021-AC-2 requires a distinct typed refusal. Either make exact binding available in alloc-only mode or make this constructor require serde; preserve the documented feature contract.

FND-002 scenario: change the router to pass a different formula or subject to `backend.settle` while returning the original pair in `LivenessSettlement`; this test still passes because `Selected::settle` ignores both arguments. Use a recording backend and assert the received identity and subject, including each disposition path.

No new vendored upstream source was identified in the feature diff: the new schema and corpus are tl-syntax-owned artifacts. No CI workflow was changed. Full `make ci` was not run because it includes the halted qualification chain; focused feature checks above do not establish that aggregate gate.

## Dispositions

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | ceaddff739ef98546c1ebc99c806ff767cd06700; fairness construction and exact graph binding now require `serde`, with an alloc-only compile-fail doctest. |
| FND-002 | fixed | ceaddff739ef98546c1ebc99c806ff767cd06700; the backend asserts the exact formula pointer and subject and the test checks one call for each disposition. |

Round 1 reviewed `ceaddff739ef98546c1ebc99c806ff767cd06700`. The original findings above remain unchanged.
