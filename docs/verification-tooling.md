# Verification tooling

How this crate's mutation, coverage evidence is produced and gated (TL-231, TL-234). The measured numbers below come from a
single machine (Apple M5 Pro, Darwin 25.4.0, rustc 1.98.1, cargo-mutants 27.0.0, cargo-llvm-cov 0.8.7,
nightly rustc 1.100.0-nightly 2026-09-21 for branch coverage) that was heavily loaded by other jobs.

## Targets

| Target | What it does | Budget |
| --- | --- | --- |
| `make mutants` | Full cargo-mutants run (`--no-shuffle`, so the population and order are fixed), then `scripts/mutation_gate.py` against `mutation-thresholds.toml` | hours; weekly, not part of `make ci` |
| `make mutants-diff` | cargo-mutants on `git diff $(MUTANTS_BASE)...HEAD` only (default `origin/main`); every mutant in the diff must be killed | minutes to an hour, per PR |
| `make coverage` | `cargo llvm-cov` line, function and region coverage on the pinned toolchain with a line floor (`COVERAGE_MIN_LINES`) | about 1 to 5 minutes |
| `make coverage-branch` | the same plus branch coverage (unstable, needs nightly with llvm-tools) | about 1 to 5 minutes |


`CI-plumbing` modules are excluded from every threshold (`.cargo/mutants.toml` `exclude_globs` and
`exclude` in `mutation-thresholds.toml`). Equivalent mutants are listed in `exclude_re` with a one-line
reason. `cargo mutants` tests every mutant against the whole test suite, so a full run costs
(mutants x suite time) divided by `MUTANTS_JOBS`; use `MUTANTS_JOBS` to trade memory for time.

## Mutation thresholds

Kill rate is `caught / (caught + missed + timeout)`; unviable mutants count on neither side. The campaign
targets are 95% for the core and 85% for the whole crate. The values enforced by the gate are the measured
floors (they only ratchet up).

| Group | Files | Enforced floor |
| --- | --- | --- |
| `core` | `["src/future.rs", "src/formula/graph.rs"]` | 95.0% |
| `crate` | `["src/*.rs", "src/*/*.rs"]` | 76.0% |

Measured at c62cbe4 on this machine: core run: 135 mutants (89 caught, 9 missed, 37 unviable = 90.82%); `tests/mutation_survivors.rs` then killed all 9 (targeted re-run of the 24 affected mutants: 23 caught, 1 unviable), so the core is 98/98 = 100%. Crate sample: core plus `--shard 0/3` (248 mutants: `src/bounded_string.rs`, `src/context.rs`, `src/contracts/*`) = 203 caught, 46 missed, 6 timeout, 54 unviable, 79.61%.

The crate floor is the sampled rate minus a margin, because the sample covers only the files named
above; the first full `make mutants` run replaces the sample and the floor should then be raised to the
measured value.

The gate is shown to fail: with a floor above the measured rate it prints `FAIL` and exits 1.
Unresolved survivors and timeouts of the measured populations (each is a `test_gap` to kill with a test,
or an equivalent mutant to list in `exclude_re` with a reason):

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

## Coverage

`make coverage-branch` on this tree (all features, `ci_guard` excluded):

| Metric | Value | Metric | Value | Metric | Value | Metric | Value |
| --- | --- | --- | --- | --- | --- | --- | --- |
| lines | 91.53% | regions | 89.88% | functions | 89.28% | branches | 84.20% |

Both coverage targets skip `no_std_feature_matrix_and_default_dependency_gate_execute`: it builds for
`thumbv7em-none-eabi`, which the coverage toolchains do not install, and fails under `cargo llvm-cov`
(`error[E0463]: can't find crate for core`). Coverage therefore excludes that test's contribution.

The stable-toolchain `make coverage` reports slightly different totals than the nightly branch run (it counts test sources too), so `COVERAGE_MIN_LINES` is set a point or two below the lower of the two. It is a floor, not a target; it fails the target when line coverage drops below it.
