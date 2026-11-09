---
id: SR-231
title: "spec-correctness - cargo-mutants configuration, kill-rate thresholds and coverage reporting (tl-syntax, TL-231, TL-234)"
type: SpecReview
analysis: spec-correctness
scope: ".cargo/mutants.toml, mutation-thresholds.toml, scripts/, tests/mutation_survivors.rs, Makefile verification targets"
review_set: all
---

## Summary

TL-231 and TL-234 add a reproducible cargo-mutants configuration, a kill-rate gate, a diff gate, and
llvm-cov line and branch coverage reporting to tl-syntax. All gates are separate Make targets (`make mutants`,
`make mutants-diff`, `make coverage`, `make coverage-branch`) outside `make ci`, because a full mutation
run costs hours. The thresholds are measured baselines, not the campaign targets of 95% (core) and 85%
(crate): the core kill rate before any change was 89 caught, 9 missed, 37 unviable = 90.82%; after: 98 of 98 viable mutants caught = 100% (targeted re-run of the 24 affected mutants: 23 caught, 1 unviable).

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-2311 | medium | Measured kill rates (cargo-mutants 27.0.0, `--no-shuffle`, base c62cbe4 rebased onto d1c17ff): core 98 of 98 viable mutants caught = 100% (targeted re-run of the 24 affected mutants: 23 caught, 1 unviable); crate sample 79.61% (203 caught, 46 missed, 6 timeout, 54 unviable over the core plus `--shard 0/3`). Timeouts count against the rate. The crate rate is a file sample, not a full run: the machine was loaded (load average 35 to 45), so a full run did not fit the budget and sharded sampling was used. The first full `make mutants` run should replace the sample and the floors should then be raised. | `mutation-thresholds.toml` |
| FND-2312 | low | The 9 first-run survivors were three error `Display` impls, `Interval::cardinality` (`&&` to `||`), `SourceSpan::is_empty`, and four deleted `past_operator` match arms; all are killed by `tests/mutation_survivors.rs`. | `docs/verification-tooling.md` |
| FND-2313 | low | The enforced floors are core 95.0, crate 76.0. A floor below its target means survivors are recorded, not accepted: each is a test gap listed in `docs/verification-tooling.md`. No survivor was classified equivalent by a machine; the only `exclude_re` entries carry a one-line reason. | `.cargo/mutants.toml` |
| FND-2314 | low | Every gate was shown able to fail. `scripts/mutation_gate.py` exits 1 with `FAIL` when a floor exceeds the measured rate (raised floor of 101 against a 100% core; and the pre-fix core run against the 95% floor), exits 1 on an empty population or a failed unmutated baseline, and exits 2 on unreadable input. `make coverage COVERAGE_MIN_LINES=99` fails with a non-zero exit. | `scripts/mutation_gate.py` |
| FND-2315 | low | Coverage (`make coverage-branch`, nightly, all features, `ci_guard` excluded): lines 91.53%, regions 89.88%, functions 89.28%, branches 84.20% (the no_std feature-boundary test is skipped). `COVERAGE_MIN_LINES` is set to 87 on the stable `make coverage` target, a floor below the measured line coverage. Branch coverage is unstable in cargo-llvm-cov 0.8.7 and needs nightly. | `Makefile` |
| FND-2316 | low | The Makefile guard (NFR-006) rejected the first version of the recipes (bare `;` in a recipe and in a help string), so the cargo-mutants exit-status handling lives in `scripts/run_mutants.py` and the Makefile only invokes it. | `scripts/run_mutants.py` |

## Execution record

Host: Darwin 25.4.0, arm64 (Apple M5 Pro). Toolchain: rustc 1.98.1, nightly rustc 1.100.0-nightly
(1303417c4 2026-09-21), cargo-mutants 27.0.0, cargo-llvm-cov 0.8.7, quire 0.38.3 (engine 0.52.1). Mutation
order is fixed by `--no-shuffle`; cargo-mutants takes no seed in this mode. Per-mutant timeout is the
cargo-mutants automatic value (three times the unmutated baseline, minimum 60 s). Files: `.cargo/mutants.toml`, `mutation-thresholds.toml`, `scripts/mutation_gate.py`, `scripts/run_mutants.py`, `tests/mutation_survivors.rs`.
Wall times are not stable on a shared machine and are not equality inputs.
