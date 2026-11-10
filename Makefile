# =============================================================================
# TL Syntax Makefile
# =============================================================================
#
# Native orchestration. Every target calls the toolchain that owns the job:
# cargo for the crate, the corpus conformance runner for the shared temporal
# corpus, quire for the specification. Nothing here computes a verdict, attests
# to its own correctness, or retains evidence of its own.

CARGO ?= cargo
QUIRE ?= quire
EMBEDDED_TARGET ?= thumbv7em-none-eabi
# Miri runs on the pinned nightly named here; the unit and property suites run
# with isolation off (property tests read their environment) and few cases.
MIRI_TOOLCHAIN ?= nightly-2026-08-21
MIRIFLAGS_TL ?= -Zmiri-disable-isolation
# Unit tests plus the property, boundary, corpus and fuzz-seed suites. Tests that
# need up to 100,000 elements or a 64 MiB buffer are `cfg(not(miri))`; their
# small-limit twins run here. Isolation is off so the corpus suites read the
# checked-in files. Excluded (NFR-006-AC-2): feature_boundary (spawns cargo) and
# future_operator_corpus (its mutation tests replay the whole corpus dozens of
# times; the suite took 58 minutes under Miri).
MIRI_TESTS ?= --lib --test props_fr_007 --test wire_round_trip_properties \
	--test infinite_trace --test infinite_formula --test past_formula_v2 \
	--test typed_signal_context --test future_lowering --test integration \
	--test strict_syntax_artifacts --test limit_boundaries \
	--test strict_reject_classes --test boundary_corpus --test infinite_fuzz_seeds \
	--test infinite_trace_corpus --test mutation_survivors --test past_history_corpus \
	--test past_profile_manifest --test wire_decode_fuzz_seeds --test miri_gate

.PHONY: help
help:
	@echo "Available targets:"
	@echo "  make mutants          - Full cargo-mutants run + kill-rate thresholds (hours, weekly)"
	@echo "  make mutants-diff     - cargo-mutants on the diff against origin/main + gate"
	@echo "  make coverage         - llvm-cov line/function/region coverage (stable) with floor"
	@echo "  make coverage-branch  - llvm-cov line + branch coverage (nightly)"
	@echo "  make fmt              - Format with rustfmt"
	@echo "  make fmt-check        - Verify formatting (CI gate)"
	@echo "  make lint             - Clippy with -D warnings"
	@echo "  make test             - cargo test"
	@echo "  make check-features   - check no-default, alloc, serde, all features, and embedded builds"
	@echo "  make test-features    - run the test suites under default, alloc, and serde features"
	@echo "  make miri             - run the unit and property suites under Miri"
	@echo "  make check-corpus     - verify corpus schemas and derived oracles"
	@echo "  make conformance      - replay the shared temporal corpus through the crate"
	@echo "  make spec             - validate specs and report coverage"
	@echo "  make spec-release     - require every active specification row to be backed"
	@echo "  make msrv             - test all targets and features with Rust 1.98.1"
	@echo "  make rustdoc          - build warning-free public documentation"
	@echo "  make build            - Release build"
	@echo "  make clean            - cargo clean"
	@echo "  make deny             - run all declared cargo-deny policy checks"
	@echo "  make fuzz-check       - compile and dependency-audit the manual fuzz targets"
	@echo "  make fuzz-smoke       - replay the checked-in fuzz corpora once per target"
	@echo "  make ci               - All CI gates locally (hosted CI is manual-only)"

# =============================================================================
# Format / Lint / Test
# =============================================================================

.PHONY: fmt
fmt:
	$(CARGO) fmt --all

.PHONY: fmt-check
fmt-check:
	$(CARGO) fmt --all -- --check
	$(CARGO) fmt --manifest-path tools/tl-gate/Cargo.toml -- --check

.PHONY: lint
lint:
	$(CARGO) clippy --all-targets --all-features -- -D warnings
	$(CARGO) clippy --manifest-path tools/tl-gate/Cargo.toml --all-targets -- -D warnings

.PHONY: test
test:
	$(CARGO) test --all-features
	$(CARGO) test --manifest-path tools/tl-gate/Cargo.toml

.PHONY: check-features
check-features:
	$(CARGO) check --lib --no-default-features
	$(CARGO) check --lib --no-default-features --features alloc
	$(CARGO) check --lib --no-default-features --features serde
	$(CARGO) check --lib --all-features
	$(CARGO) build --lib --no-default-features --target $(EMBEDDED_TARGET)
	$(CARGO) build --lib --no-default-features --features alloc --target $(EMBEDDED_TARGET)
	$(CARGO) build --lib --no-default-features --features alloc,serde --target $(EMBEDDED_TARGET)

# Feature matrix for the test suites (NFR-001-AC-2, TC-019): default (no features),
# alloc, and serde (which implies alloc, so alloc+serde is the same build). Each
# combination runs every lib, integration, example and doc test it enables.
.PHONY: test-features
test-features:
	$(CARGO) test --locked --all-targets
	$(CARGO) test --locked --all-targets --features alloc
	$(CARGO) test --locked --all-targets --features serde
	$(CARGO) test --locked --doc
	$(CARGO) test --locked --doc --features alloc
	$(CARGO) test --locked --doc --features serde

# Miri is the UB gate for the unit and property suites. Properties run with a
# small case count and no failure-persistence files; the full counts run natively.
.PHONY: miri
miri:
	MIRIFLAGS="$(MIRIFLAGS_TL)" PROPTEST_CASES=2 PROPTEST_DISABLE_FAILURE_PERSISTENCE=true \
		$(CARGO) +$(MIRI_TOOLCHAIN) miri test --all-features $(MIRI_TESTS)

.PHONY: check-default-dependencies
check-default-dependencies:
	$(CARGO) run --quiet --example default_dependencies
	$(CARGO) test --quiet --example default_dependencies

.PHONY: conformance
conformance:
	$(CARGO) run --quiet --example corpus_conformance --features serde -- \
		--manifest corpus/manifest.json

.PHONY: check-corpus
check-corpus:
	$(CARGO) run --quiet --example corpus_gate --features serde
	$(CARGO) test --quiet --example corpus_gate --features serde

# Specification-first work is allowed to land before its implementation. The
# authoring gate reports those rows without treating them as evidence; the
# human release task requires the strict sibling once every routed ticket lands.
.PHONY: spec spec-release
spec:
	$(QUIRE) validate --scope . 'spec/**/*.md' --strict --summary
	$(QUIRE) coverage --scope .

spec-release:
	$(QUIRE) validate --scope . 'spec/**/*.md' --strict --summary
	$(QUIRE) coverage --scope . --strict

.PHONY: build
build:
	$(CARGO) build --release

.PHONY: clean
clean:
	$(CARGO) clean

# =============================================================================
# Supply chain & safety
# =============================================================================

.PHONY: deny
deny:
	$(CARGO) deny check advisories
	$(CARGO) deny check bans
	$(CARGO) deny check licenses
	$(CARGO) deny check sources

# This compiles and audits the target but never starts a fuzzing campaign. The
# campaign remains an explicit, manual operator action documented in README.
.PHONY: fuzz-check
fuzz-check:
	$(CARGO) +nightly fuzz build
	cd fuzz && $(CARGO) deny check --config ../deny.toml

# fuzz-smoke replays every checked-in corpus entry through its target once
# (no mutation, no campaign). Longer campaigns are manual: see README.
.PHONY: fuzz-smoke
fuzz-smoke:
	@for target in $$($(CARGO) +nightly fuzz list); do \
		$(CARGO) +nightly fuzz run $$target fuzz/corpus/$$target -- -runs=0 || exit 1; \
	done

.PHONY: msrv
msrv:
	rustup run 1.98.1 $(CARGO) test --all-features

.PHONY: rustdoc
rustdoc:
	RUSTDOCFLAGS=-Dwarnings $(CARGO) doc --no-deps --all-features

# =============================================================================
# Verification effectiveness (TL-231, TL-234). Heavy, so deliberately outside
# `make ci`; see docs/verification-tooling.md for budgets and measured numbers.
# =============================================================================

PYTHON ?= python3
GATE ?= $(CARGO) run --quiet --manifest-path tools/tl-gate/Cargo.toml --
MUTANTS_OUT ?= target/mutants
MUTANTS_JOBS ?= 4
MUTANTS_BASE ?= origin/main
COVERAGE_MIN_LINES ?= 85
# The skipped test builds for thumbv7em-none-eabi, which the coverage toolchain
# does not install and which instrumented builds cannot target.
COVERAGE_IGNORE = '(ci_guard)'

# The wrapper accepts cargo-mutants' "survivors found" status and judges the
# result against mutation-thresholds.toml (see tools/tl-gate).
.PHONY: mutants
mutants:
	$(GATE) mutants --jobs $(MUTANTS_JOBS) $(MUTANTS_OUT)

.PHONY: mutants-diff
mutants-diff:
	$(GATE) mutants --jobs $(MUTANTS_JOBS) --diff-base $(MUTANTS_BASE) $(MUTANTS_OUT)

.PHONY: coverage
coverage:
	$(CARGO) llvm-cov --all-features --summary-only \
		--ignore-filename-regex $(COVERAGE_IGNORE) \
		--fail-under-lines $(COVERAGE_MIN_LINES) \
		-- --skip no_std_feature_matrix_and_default_dependency_gate_execute

.PHONY: coverage-branch
coverage-branch:
	$(CARGO) +nightly llvm-cov --all-features --branch --summary-only \
		--ignore-filename-regex $(COVERAGE_IGNORE) \
		--fail-under-lines $(COVERAGE_MIN_LINES) \
		-- --skip no_std_feature_matrix_and_default_dependency_gate_execute

# =============================================================================
# Composite
# =============================================================================

# `ci` is the development composite. A release candidate additionally runs
# `spec-release` under Task-007 after planned roadmap rows have backing.
.PHONY: ci
ci: fmt-check check-features check-default-dependencies lint test test-features check-corpus \
	conformance deny fuzz-check spec msrv rustdoc miri
