---
id: SR-613
title: "Evidence record - tl-syntax fuzz runs (TL-219, TL-29)"
type: SpecReview
analysis: evidence
scope: "fuzz/, wire_decode and infinite_wire_decode targets, FR-004-AC-5, FR-289-AC-3, TC-036, TC-145"
review_set: subset
---

## Summary

Recorded libFuzzer runs of the targets listed below, three RNG seeds of 60 seconds
each per target (19,978,188 executions in total). No run produced a crash, timeout, out-of-memory or sanitizer finding.
These are bounded observations of finite runs: a run without a crash and a coverage
curve that had not flattened are not evidence of correctness (FR-021-AC-5).

## Verdict

**PASS** for the recorded evidence: every run completed, each new target is shown
able to find a seeded fault, and the fuzz gate (`make fuzz-check`) builds every target.
Targets are discharged only by these recorded runs, not by existing.

## Run record

Source revision 7c5a42c06741 (campaign tool state clean for `src/`, `Cargo.toml`, `Cargo.lock`);
tools: rustc 1.100.0-nightly (1303417c4 2026-09-21), cargo-fuzz 0.13.1, host aarch64-apple-darwin; sanitizer: address (cargo-fuzz default); libFuzzer flags
`-max_total_time=60 -max_len=2048 -rss_limit_mb=8192`; seeds 101, 102, 103 (distinct, run in that order on one
growing corpus per target, or on a throwaway copy of the checked-in corpus for
targets whose checked-in seeds are tested one-by-one). Time is libFuzzer's
reported run time and excludes build time. Machine-readable record:
`fuzz/evidence/2026-11-08/campaign.json`.

| Target | Seed | Seconds | Executions | Coverage edges | Features | Corpus entries | Findings |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | --- |
| wire_decode | 101 | 61 | 3,341,145 | 2,168 | 6,412 | 1,992 | none |
| wire_decode | 102 | 61 | 4,551,432 | 2,231 | 6,995 | 1,824 | none |
| wire_decode | 103 | 61 | 4,936,454 | 2,273 | 7,369 | 1,942 | none |
| infinite_wire_decode | 101 | 61 | 2,981,315 | 3,745 | 10,102 | 2,603 | none |
| infinite_wire_decode | 102 | 61 | 2,236,860 | 3,858 | 10,702 | 2,419 | none |
| infinite_wire_decode | 103 | 61 | 1,930,982 | 3,936 | 11,044 | 2,507 | none |

## Seeded-fault check

A target is credited only if it fails on a deliberately introduced fault. Each
fault was applied temporarily to the library source (and reverted), and the
target was run from an empty or seed-only corpus.

| Target | Fault | Result |
| --- | --- | --- |
| infinite_wire_decode | bounded `Interval` deserializer made to accept a missing `end` as `u32::MAX` and to ignore unknown fields (src/formula/graph.rs), i.e. the bounded reader silently reads `[a,)` as `[a,4294967295]` | found on the first execution of the checked-in `formula.json` seed (seed 219, assertion that the relabelled bounded document is refused) |

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-6101 | high | `make fuzz-check` (part of `make ci`) could not build any fuzz target on main: the root manifest carries `[workspace.lints]`, which makes it a workspace root, and `fuzz/Cargo.toml` did not declare its own workspace (`current package believes it's in a workspace when it's not`). Fixed by an own `[workspace]` table in the fuzz manifest; `make fuzz-check` now builds both targets and audits the fuzz crate. | fuzz/Cargo.toml, Makefile |
| FND-6102 | medium | `wire_decode` had no seed corpus, so the first campaign would have started from nothing. Nine seeds (five formulas, one proposition map, three malformed documents) are checked in and `tests/wire_decode_fuzz_seeds.rs` proves they reach both the accepting and refusing decoder paths. | fuzz/corpus/wire_decode, TC-036 |
| FND-6103 | low | The unbounded-interval version boundary was asserted only in a unit test. `infinite_wire_decode` now also asserts, for every accepted document containing `[a,)`, that the bounded readers refuse it, including after its schema label, profile and clock are rewritten to the bounded forms. | fuzz/fuzz_targets/infinite_wire_decode.rs, TC-145, FR-289-AC-3 |
| FND-6104 | low | No crash, timeout or out-of-memory result in any recorded run. Coverage was still growing at the end of every run, so no plateau is claimed. | campaign.json |
| FND-6105 | low | On macOS libFuzzer counts the address sanitizer's freed memory toward RSS, so the default 2048 MB limit aborted runs whose live heap was tens of megabytes; recorded runs pass `-rss_limit_mb=8192`. Not a target defect. | fuzz/README |

## Limitations

The runs are three 60-second repetitions, not the 3 x 900 s baseline profile of
FR-021, and carry no feature-identity snapshots, so FR-021-AC-1 to AC-4 are not
claimed. The checked-in corpora are hand-picked seeds only; grown corpora from
these runs were scratch copies and are not fixtures.

