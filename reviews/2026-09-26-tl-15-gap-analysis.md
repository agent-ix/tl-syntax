---
id: SR-112
title: "TL-15 feature trace gap analysis"
type: SpecReview
analysis: gap-analysis
scope: "agent-ix/tl-syntax@e2ca5aabfe5a9cf00c8d9806e812854a5c4061c7; spec/infinite-trace-test-matrix.md; spec/requirements/FR-020 through FR-024 and FR-289 through FR-291; tests/infinite_formula.rs; tests/infinite_trace.rs; tests/infinite_trace_corpus.rs; tests/v1_spec_stubs.rs"
review_set: subset
---

## Summary

Ticket: TL-15. Compared the new requirements and matrix to executable feature tests. The matrix correctly retains cross-repository cases as planned, but one local TC-147 assertion is weaker than its criterion.

## Verdict

**CONDITIONAL** — TL-15's local feature tests execute, but TC-147 has an oracle gap and downstream TC-150/TC-162 remain explicitly planned outside this PR.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | TC-147 tags FR-291-AC-1 but does not verify the backend consumed the selected graph and subject. | tests/infinite_formula.rs:268 |
| FND-002 | low | TC-150's QSL comparison and TC-162's downstream corpus-pin inspection remain planned and are not TL-15 completion evidence. | spec/infinite-trace-test-matrix.md:43 |

## Coverage

The focused 33 tests passed. `quire coverage --scope . --json` reports TC-162 unbacked; the source file contains only an ignored pending test. TC-150's local golden-byte assertion runs, while its native correspondence portion is explicitly assigned to TL-13 and TL-212. TC-146 exercises the local optional backend routing and absence path, within TL-15 ownership. This analysis does not claim downstream qualification or release readiness.

## Dispositions

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | ceaddff739ef98546c1ebc99c806ff767cd06700; the TC-147 backend now checks the graph pointer, subject and call count. |
| FND-002 | deferred | TC-150 native QSL/paired-corpus comparison belongs to TL-13; TC-162 exact downstream pin inspection belongs to TL-212, whose qualification campaign is halted. Neither is TL-15 syntax evidence or a TL-15 merge blocker. |

Round 1 reviewed `ceaddff739ef98546c1ebc99c806ff767cd06700`. The original findings above remain unchanged.
