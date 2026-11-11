# Verification tooling

How tl-syntax's mutation, coverage evidence is produced and gated (TL-231, TL-234). This page describes the targets and what
the gates enforce; it carries no measured results. Floors live in `mutation-thresholds.toml` and the
`COVERAGE_MIN_LINES` variable of the Makefile, and dated measurements live in `spec/reviews/SR-231-*`
and the Linear tickets.

Tools (inputs): cargo-mutants 27.0.0, cargo-llvm-cov 0.8.7 (branch coverage needs a nightly toolchain with
llvm-tools).

## Targets

| Target | What it does |
| --- | --- |
| `make mutants` | Full cargo-mutants run (`--no-shuffle`, so the population and order are fixed), then `tl-gate gate` against `mutation-thresholds.toml`. Takes hours; run it weekly. |
| `make mutants-diff` | A gate in `make ci`. cargo-mutants on `git diff $(MUTANTS_BASE)...HEAD` (default `origin/main`) only; every mutant in the diff must be killed. Only committed changes are mutated; uncommitted edits are not. |
| `make coverage` | `cargo llvm-cov` line, function and region coverage on the pinned toolchain, failing below `COVERAGE_MIN_LINES`. |
| `make coverage-branch` | The same on nightly, adding branch coverage. |


`make mutants-diff` is part of `make ci`, so a change cannot add logic its tests would not notice; `make mutants` and the coverage targets are not.

## Mutation gate

Kill rate is `caught / (caught + missed + timeout)`: a timeout counts as not killed, and unviable mutants
count on neither side. Each group in `mutation-thresholds.toml` has a floor; the gate prints a table and
exits 1 when a group is below its floor, when a group has no viable mutants, or when the unmutated
baseline failed. The campaign targets are a core group and a whole-crate group (the core is `src/future.rs` and `src/formula/graph.rs` (future lowering, formula graph, interval arithmetic)).

Excluded from every threshold: nothing (the crate has no CI-plumbing module). Mutants that cannot change behavior are excluded in `exclude_re` of `.cargo/mutants.toml`, each with a one-line reason, so they are never generated.

`tl-gate mutants` accepts cargo-mutants exit statuses 0, 2 (survivors) and 3 (timeouts) so that
the gate always runs and prints its table; any other status is passed on.
The gate and the runner are one Rust package, `tools/tl-gate` in this repository; tl-parse, tl-rewrite and tl-mltl
build it through a `tools/gate` shim that names this repository's revision in its `Cargo.toml`. `--source-env` is used
only by tl-mltl. A thresholds file may set `require_survivor_reasons = true` to demand a `[[survivor]]` entry with a
reason for every missed or timed-out mutant.

## Limits of the recorded measurements

The floors are the rates of the first full `make mutants` run (SR-231), and they only ratchet up.
The Campaign runner cannot contain `make mutants` (TL-28, 2026-09-24): cargo-mutants starts its own Cargo child
in a new process group, and the runner refuses a descendant whose process group differs from its producer. That
containment is unresolved, so `make mutants` is the supported way to run the mutation gate.

The cargo-mutants test-timeout floor is 300 s (`minimum_test_timeout` in `.cargo/mutants.toml`); a mutant times out
when its test run exceeds the larger of that floor and three times the baseline. The floor cannot tell a hang from a
run that a loaded host slowed past it, so run `make mutants` on an unloaded machine. Timeouts count as unkilled.
Mutation runs skip `tests/feature_boundary.rs`, which rebuilds the crate for every feature set and checks feature
gating, not behavior.

## Coverage

Stable and nightly toolchains report different line totals, so `COVERAGE_MIN_LINES` is set below the lower of
the two. It is a floor, not a target. Coverage reports exclude CI-plumbing code.

Both coverage targets skip `no_std_feature_matrix_and_default_dependency_gate_execute`: it builds for
`thumbv7em-none-eabi`, which the coverage toolchains do not install, and fails under `cargo llvm-cov`
(`error[E0463]: can't find crate for core`).
