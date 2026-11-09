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
line and branch coverage reporting to tl-syntax, as separate Make targets outside `make ci`. This record is the dated
evidence (2026-11-08 and 2026-11-09); `docs/verification-tooling.md` carries no figures. The mutation targets were not met crate-wide: the crate-wide rate (79.61%) is a core plus shard 0/3 sample, not a full run, below the 85% target, and the floors (core 95, crate 76) are the sampled rate minus a margin; the core itself met 95%.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-2311 | medium | Mutation measurements (cargo-mutants 27.0.0, `--no-shuffle`, taken on the tree before the final rebase (pre-rebase)): Core (`src/future.rs`, `src/formula/graph.rs`): first run 89 caught, 9 missed, 37 unviable (90.82%); `tests/mutation_survivors.rs` killed all 9 (targeted re-run of the 24 affected mutants: 23 caught, 1 unviable), so 98 of 98 viable. Crate sample (core plus `--shard 0/3`, 248 mutants): 203 caught, 46 missed, 6 timeout, 54 unviable (79.61%). Timeouts count against the rate. | `mutation-thresholds.toml` |
| FND-2312 | low | The 9 first-run survivors were three error `Display` impls, `Interval::cardinality`, `SourceSpan::is_empty` and four `past_operator` arms. `minimum` in `src/contracts/limits.rs` (`<` to `<=`) is equivalent (equal operands give equal results) and is listed in `exclude_re`, not a test gap; it is counted as missed in the figures above. 41 of the missed mutants are in `src/contracts/reader.rs`. | `docs/verification-tooling.md` |
| FND-2313 | low | The enforced mutation floors are the measured sampled rates minus a margin, not the 95% and 85% targets; groups below target have open survivors, listed below, that are test gaps to kill. | `mutation-thresholds.toml` |
| FND-2314 | low | Every gate was shown able to fail: `scripts/mutation_gate.py` exits 1 with `FAIL` when a floor exceeds the measured rate, on an empty population or a failed baseline, and exits 2 on unreadable input; `make coverage COVERAGE_MIN_LINES=99` exits non-zero. `scripts/run_mutants.py` accepts cargo-mutants exit 0, 2 and 3 so the gate always prints its table. | `scripts/mutation_gate.py` |
| FND-2315 | low | Coverage re-measured from a clean `target/llvm-cov-target` on 2026-11-09, all features, `ci_guard` excluded. Stable `make coverage`: 91.53% lines / 90.88% regions / 89.00% functions. Nightly `make coverage-branch`: 87.93% lines / 68.61% regions / 89.28% functions / 84.20% branches. The two toolchains report different line totals and the nightly region figure is much lower, so `COVERAGE_MIN_LINES` is 85, below the lower line figure; both targets pass at that floor. A reviewer measured lower totals on a fresh clone than these; the difference could not be reproduced here, so the floor sits below both readings that were available. | `Makefile` |

## Execution record

Host: Darwin 25.4.0, arm64 (Apple M5 Pro), heavily loaded (load average 35 to 45). Toolchain: rustc 1.98.1,
nightly rustc 1.100.0-nightly (1303417c4 2026-09-21), cargo-mutants 27.0.0, cargo-llvm-cov 0.8.7, quire 0.38.3
(engine 0.52.1). Mutation order is fixed by `--no-shuffle`; there is no seed in this mode. The per-mutant timeout
is the cargo-mutants automatic value (three times the unmutated baseline, minimum 60 s in this crate's
`.cargo/mutants.toml`). Wall times are not equality inputs. The crate-wide rates are core plus `--shard 0/3` file
samples (cargo-mutants shards by file), not full runs.

## Open survivors and timeouts of the measured populations

Each is a test gap to kill, unless listed as equivalent above.

- `src/bounded_string.rs:43:13: replace deserialize::<impl serde::de::Visitor<'de> for Visitor>::expecting -> fmt::Result with Ok(Default::default())`
- `src/bounded_string.rs:62:13: replace deserialize::<impl serde::de::Visitor<'de> for Visitor>::visit_str -> Result<Self::Value, E> with Ok(Default::default())`
- `src/context.rs:105:9: replace <impl fmt::Display for RequirementContextField>::fmt -> fmt::Result with Ok(Default::default())`
- `src/context.rs:136:9: replace <impl fmt::Display for RequirementContextError>::fmt -> fmt::Result with Ok(Default::default())`
- `src/contracts/limits.rs:5:49: replace * with +`
- `src/contracts/limits.rs:79:13: replace < with <= in minimum`
- `src/contracts/manifest_impl.rs:169:20: replace > with >= in validate_past_profile_implementation`
- `src/contracts/manifest_impl.rs:67:9: replace PastProfileManifestError::task -> Option<&str> with Some("")`
- `src/contracts/manifest_impl.rs:67:9: replace PastProfileManifestError::task -> Option<&str> with Some("xyzzy")`
- `src/contracts/manifest_impl.rs:73:9: replace <impl fmt::Display for PastProfileManifestError>::fmt -> fmt::Result with Ok(Default::default())`
- `src/contracts/reader.rs:158:21: replace > with == in preflight_document`
- `src/contracts/reader.rs:158:21: replace > with >= in preflight_document`
- `src/contracts/reader.rs:194:5: replace scan_json_string -> Result<usize, StrictDocumentReadError> with Ok(0)`
- `src/contracts/reader.rs:194:5: replace scan_json_string -> Result<usize, StrictDocumentReadError> with Ok(1)`
- `src/contracts/reader.rs:195:17: replace < with <= in scan_json_string`
- `src/contracts/reader.rs:198:13: delete match arm b'\\' in scan_json_string`
- `src/contracts/reader.rs:200:26: replace >= with < in scan_json_string`
- `src/contracts/reader.rs:203:33: replace == with != in scan_json_string`
- `src/contracts/reader.rs:226:5: replace decoded_unicode_escape -> (usize, usize) with (0, 0)`
- `src/contracts/reader.rs:226:5: replace decoded_unicode_escape -> (usize, usize) with (0, 1)`
- `src/contracts/reader.rs:226:5: replace decoded_unicode_escape -> (usize, usize) with (1, 0)`
- `src/contracts/reader.rs:226:5: replace decoded_unicode_escape -> (usize, usize) with (1, 1)`
- `src/contracts/reader.rs:230:49: replace == with != in decoded_unicode_escape`
- `src/contracts/reader.rs:230:9: replace && with || in decoded_unicode_escape`
- `src/contracts/reader.rs:231:49: replace == with != in decoded_unicode_escape`
- `src/contracts/reader.rs:231:9: replace && with || in decoded_unicode_escape`
- `src/contracts/reader.rs:232:9: replace && with || in decoded_unicode_escape`
- `src/contracts/reader.rs:238:9: delete match arm 0x0000..= 0x007F in decoded_unicode_escape`
- `src/contracts/reader.rs:239:9: delete match arm 0x0080..= 0x07FF in decoded_unicode_escape`
- `src/contracts/reader.rs:247:5: replace parse_hex_quad -> Option<u16> with None`
- `src/contracts/reader.rs:247:5: replace parse_hex_quad -> Option<u16> with Some(0)`
- `src/contracts/reader.rs:247:5: replace parse_hex_quad -> Option<u16> with Some(1)`
- `src/contracts/reader.rs:250:13: delete match arm b'0'..= b'9' in parse_hex_quad`
- `src/contracts/reader.rs:250:44: replace - with + in parse_hex_quad`
- `src/contracts/reader.rs:250:44: replace - with / in parse_hex_quad`
- `src/contracts/reader.rs:251:13: delete match arm b'a'..= b'f' in parse_hex_quad`
- `src/contracts/reader.rs:251:44: replace - with + in parse_hex_quad`
- `src/contracts/reader.rs:251:44: replace - with / in parse_hex_quad`
- `src/contracts/reader.rs:251:51: replace + with - in parse_hex_quad`
- `src/contracts/reader.rs:252:13: delete match arm b'A'..= b'F' in parse_hex_quad`
- `src/contracts/reader.rs:252:44: replace - with + in parse_hex_quad`
- `src/contracts/reader.rs:252:44: replace - with / in parse_hex_quad`
- `src/contracts/reader.rs:252:51: replace + with - in parse_hex_quad`
- `src/contracts/reader.rs:294:5: replace count_array_items -> (usize, usize) with (0, 0)`
- `src/contracts/reader.rs:294:5: replace count_array_items -> (usize, usize) with (0, 1)`
- `src/contracts/reader.rs:294:5: replace count_array_items -> (usize, usize) with (1, 0)`
- `src/contracts/reader.rs:294:5: replace count_array_items -> (usize, usize) with (1, 1)`
- `src/contracts/reader.rs:297:17: replace < with <= in count_array_items`
- `src/contracts/reader.rs:299:13: delete match arm b'"' in count_array_items`
- `src/contracts/reader.rs:314:13: delete match arm b']' in count_array_items`
- `src/contracts/reader.rs:67:9: replace <impl fmt::Display for StrictDocumentReadError>::fmt -> fmt::Result with Ok(Default::default())`
