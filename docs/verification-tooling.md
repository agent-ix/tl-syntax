# Verification tooling

How tl-syntax's mutation, coverage evidence is produced and gated (TL-231, TL-234). This page describes the targets and what
the gates enforce; it carries no measured results. Floors live in `mutation-thresholds.toml` and the
`COVERAGE_MIN_LINES` variable of the Makefile, and dated measurements live in `spec/reviews/SR-231-*`
and the Linear tickets.

Tools (inputs): cargo-mutants 27.0.0, cargo-llvm-cov 0.8.7 (branch coverage needs a nightly toolchain with
llvm-tools), Python 3.11 or newer (the gate reads TOML with `tomllib`).

## Targets

| Target | What it does |
| --- | --- |
| `make mutants` | Full cargo-mutants run (`--no-shuffle`, so the population and order are fixed), then `tl-gate gate` against `mutation-thresholds.toml`. Takes hours; run it weekly. |
| `make mutants-diff` | cargo-mutants on `git diff $(MUTANTS_BASE)...HEAD` (default `origin/main`) only; every mutant in the diff must be killed. Only committed changes are mutated; uncommitted edits are not. |
| `make coverage` | `cargo llvm-cov` line, function and region coverage on the pinned toolchain, failing below `COVERAGE_MIN_LINES`. |
| `make coverage-branch` | The same on nightly, adding branch coverage. |


None of the mutation or coverage targets is part of `make ci`.

## Mutation gate

Kill rate is `caught / (caught + missed + timeout)`: a timeout counts as not killed, and unviable mutants
count on neither side. Each group in `mutation-thresholds.toml` has a floor; the gate prints a table and
exits 1 when a group is below its floor, when a group has no viable mutants, or when the unmutated
baseline failed. The campaign targets are a core group and a whole-crate group (the core is `src/future.rs` and `src/formula/graph.rs` (future lowering, formula graph, interval arithmetic)).

Excluded from every threshold: nothing (the crate has no CI-plumbing module). One equivalent mutant family is listed in `exclude_re`: `minimum` in `src/contracts/limits.rs` returns the same value for `<` and `<=`.

`tl-gate mutants` accepts cargo-mutants exit statuses 0, 2 (survivors) and 3 (timeouts) so that
the gate always runs and prints its table; any other status is passed on.
The gate and the runner are one Rust package, `tools/tl-gate` in this repository; tl-parse, tl-rewrite and tl-mltl
build it through a `tools/gate` shim that names this repository's revision in its `Cargo.toml`. `--source-env` is used
only by tl-mltl. A thresholds file may set `require_survivor_reasons = true` to demand a `[[survivor]]` entry with a
reason for every missed or timed-out mutant.

## Limits of the recorded measurements

Crate-wide kill rates recorded so far are core plus one `--shard 0/3` file sample, not a full run, and the
floors are the sampled rate minus a margin. The first full `make mutants` run should replace them, and floors
only ratchet up. In the recorded run the core met its mutation target after `tests/mutation_survivors.rs` was added; the crate-wide figure is a file sample, not a full run (see SR-231). Open survivors are listed in SR-231.

## Coverage

Stable and nightly toolchains report different line totals, so `COVERAGE_MIN_LINES` is set below the lower of
the two. It is a floor, not a target. Coverage reports exclude CI-plumbing code.

Both coverage targets skip `no_std_feature_matrix_and_default_dependency_gate_execute`: it builds for
`thumbv7em-none-eabi`, which the coverage toolchains do not install, and fails under `cargo llvm-cov`
(`error[E0463]: can't find crate for core`).
