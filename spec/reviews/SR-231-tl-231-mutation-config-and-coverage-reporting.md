---
id: SR-231
title: "spec-correctness - cargo-mutants configuration, kill-rate thresholds and coverage reporting (tl-syntax, TL-231, TL-234)"
type: SpecReview
analysis: spec-correctness
scope: "mutation configuration, thresholds, scripts and Makefile verification targets of tl-syntax"
review_set: all
---

## Summary

TL-231 and TL-234 add a reproducible cargo-mutants configuration, a kill-rate gate, a diff gate and llvm-cov
line and branch coverage reporting to tl-syntax. TL-371 (2026-11-09 and 2026-11-10) replaces the sampled floors with
the first full run, requires a reason for every surviving mutant and puts `make mutants-diff` in `make ci`. This
record is the dated evidence; `docs/verification-tooling.md` carries no figures. The core group is at 100.00% and the
crate at 100.00%; floors are 100.0 and 100.0.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-2311 | medium | First full `make mutants` (cargo-mutants 27.0.0, `--no-shuffle`, 2026-11-09): 764 mutants, 367 caught, 22 missed, 184 timeouts, 189 unviable. Crate kill rate 64.05%. Wall time 42795 s. The 184 timeouts were mostly not hangs: `tests/feature_boundary.rs` rebuilds the crate for every feature set and takes minutes under load, and cargo runs it before the later test binaries, so any mutant killed in a later binary ran past the 300 s limit. | `mutation-thresholds.toml` |
| FND-2312 | low | `.cargo/mutants.toml` now skips `no_std_feature_matrix_and_default_dependency_gate_execute` in mutation test runs (it checks feature gating, which no mutant changes). `tests/early_mutation_survivors.rs` and unit tests in `src/contracts/reader.rs` add behavior tests for the strict scanner (escape widths, hex digits, array population), the wire readers (exact work charged, limit at and one past, limits enforced on the decoded document when a field name hides from the byte scan), content identities, error messages and schema identities, the catalog lookup and iterator, and the manifest refusal. The test file name sorts first so a mutant that hangs a later suite is still killed. The retest of every missed and timed-out mutant took 29663 s for the main pass and 3025 s for the last nine. | `tests/early_mutation_survivors.rs` |
| FND-2313 | low | Result after the new tests, equivalent mutants excluded as `.cargo/mutants.toml` now does: core 100.00% (99 caught, 0 missed, 0 timeout, 37 unviable), crate 100.00% (566 caught, 0 missed, 0 timeout, 192 unviable). The crate figure is the earlier 556 caught plus the ten scanner mutants listed below, which a later targeted pass (`-j 6`, 600 s timeout, 41 mutants of the four scanners: 41 caught) killed; the other mutants were not rerun. Floors are the measured rates truncated: core 100.0, crate 100.0. Two mutants are excluded as equivalent, each with a reason in `.cargo/mutants.toml`. | `mutation-thresholds.toml`, `.cargo/mutants.toml` |
| FND-2314 | low | The gate and the runner are `tools/tl-gate` (TL-370). With `require_survivor_reasons`, a missed or timed-out mutant that is not listed with a reason fails the gate. The stale-outcomes file is cleared before each `make mutants-diff`. `make coverage COVERAGE_MIN_LINES=99` exits non-zero. | `tools/tl-gate` |
| FND-2315 | low | Coverage re-measured from a clean `target/llvm-cov-target` on 2026-11-09, all features, `ci_guard` excluded. Stable `make coverage`: 91.53% lines / 90.88% regions / 89.00% functions. Nightly `make coverage-branch`: 87.93% lines / 68.61% regions / 89.28% functions / 84.20% branches. The two toolchains report different line totals and the nightly region figure is much lower, so `COVERAGE_MIN_LINES` is 85, below the lower line figure; both targets pass at that floor. A reviewer measured lower totals on a fresh clone than these; the difference could not be reproduced here, so the floor sits below both readings that were available. | `Makefile` |

## Execution record

Host: Darwin 25.4.0, arm64 (Apple M5 Pro), heavily loaded (load average 12 to 17 during the retests; 35 to 45 earlier).
Toolchain: rustc 1.98.1, nightly rustc 1.100.0-nightly (1303417c4 2026-09-21), cargo-mutants 27.0.0,
cargo-llvm-cov 0.8.7, quire 0.38.3 (engine 0.52.1). Mutation order is fixed by `--no-shuffle`; there is no seed in
this mode. The per-mutant timeout is the cargo-mutants automatic value (three times the unmutated baseline, minimum
300 s in this crate's `.cargo/mutants.toml`, raised from 60 s in TL-371). The floor cannot tell a hang from a run
that a loaded host slowed past it, so timeouts are counted as unkilled. Wall times are not equality inputs. The
2026-11-10 rates are a full run plus retests of every missed and timed-out mutant; a mutant a new test kills stays
killed, so the merge is the rate a fresh full run would report.

## Survivors

No mutant is left unlisted. Ten mutants once hung the suite so that the timeout was the detection: `count_array_items`, `scan_json_string`, `scan_string_end` and `skip_json_whitespace` in `src/contracts/reader.rs` returning a constant. In-module tests now run each scanner on a thread and abort the test process if it does not finish in 20 s, so the hang is a failure; the abort is needed because a hung scanner also hangs every other test that reaches it. The four binary-search mutants of `SignalCatalog::signal` are killed by the same kind of lookup test. The Campaign runner cannot contain `make mutants` (TL-28, 2026-09-24); `make mutants` is the supported path.
