# =============================================================================
# TL Syntax Makefile
# =============================================================================
#
# Native orchestration. Every target calls the toolchain that owns the job:
# cargo for the crate, the corpus conformance runner for the shared temporal
# corpus, quire for the specification. Nothing here computes a verdict, attests
# to its own correctness, or retains evidence of its own.

CARGO ?= cargo
PYTHON ?= python3
QUIRE ?= quire

.PHONY: help
help:
	@echo "Available targets:"
	@echo "  make fmt              - Format with rustfmt"
	@echo "  make fmt-check        - Verify formatting (CI gate)"
	@echo "  make lint             - Clippy with -D warnings"
	@echo "  make test             - cargo test"
	@echo "  make check-features   - check no-default, alloc, serde, and all features"
	@echo "  make check-corpus     - verify corpus digests, schemas, and derived oracles"
	@echo "  make conformance      - replay the shared temporal corpus through the crate"
	@echo "  make spec             - validate specs, check id: uniqueness, and report coverage"
	@echo "  make spec-release     - require every active specification row to be backed"
	@echo "  make msrv             - test all targets and features with Rust 1.98.1"
	@echo "  make rustdoc          - build warning-free public documentation"
	@echo "  make build            - Release build"
	@echo "  make clean            - cargo clean"
	@echo "  make deny             - run all declared cargo-deny policy checks"
	@echo "  make fuzz-check       - compile and dependency-audit the manual fuzz targets"
	@echo "  make audit-unsafe     - Enforce // SAFETY: comments on unsafe blocks"
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

.PHONY: lint
lint:
	$(CARGO) clippy --all-targets --all-features -- -D warnings

.PHONY: test
test:
	$(CARGO) test --all-features

.PHONY: check-features
check-features:
	$(CARGO) check --lib --no-default-features
	$(CARGO) check --lib --no-default-features --features alloc
	$(CARGO) check --lib --no-default-features --features serde
	$(CARGO) check --lib --all-features

.PHONY: check-default-dependencies
check-default-dependencies:
	$(PYTHON) scripts/check_default_dependencies.py

.PHONY: conformance
conformance:
	$(CARGO) run --quiet --example corpus_conformance --features serde -- \
		--manifest corpus/manifest.json

.PHONY: check-corpus
check-corpus:
	sha256sum --check corpus/SHA256SUMS
	sha256sum --check corpus/future-operators/SHA256SUMS
	sha256sum --check corpus/past-history/SHA256SUMS
	$(PYTHON) scripts/validate_corpus.py
	$(PYTHON) scripts/test_corpus_gate.py

# Specification-first work is allowed to land before its implementation. The
# authoring gate reports those rows without treating them as evidence; the
# human release task requires the strict sibling once every routed ticket lands.
.PHONY: spec spec-release
spec:
	$(QUIRE) validate --scope . 'spec/**/*.md' --strict --summary
	$(PYTHON) scripts/check_spec_id_uniqueness.py
	$(PYTHON) scripts/test_check_spec_id_uniqueness.py
	$(QUIRE) coverage --scope .

spec-release:
	$(QUIRE) validate --scope . 'spec/**/*.md' --strict --summary
	$(PYTHON) scripts/check_spec_id_uniqueness.py
	$(PYTHON) scripts/test_check_spec_id_uniqueness.py
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
	$(CARGO) +nightly fuzz build wire_decode
	cd fuzz && $(CARGO) deny check --config ../deny.toml

.PHONY: audit-unsafe
audit-unsafe:
	bash scripts/check_unsafe_comments.sh

.PHONY: msrv
msrv:
	rustup run 1.98.1 $(CARGO) test --all-features

.PHONY: rustdoc
rustdoc:
	RUSTDOCFLAGS=-Dwarnings $(CARGO) doc --no-deps --all-features

# =============================================================================
# Composite
# =============================================================================

# `ci` is the development composite. A release candidate additionally runs
# `spec-release` under Task-007 after planned roadmap rows have backing.
.PHONY: ci
ci: fmt-check check-features check-default-dependencies lint test check-corpus \
	conformance deny fuzz-check audit-unsafe spec msrv rustdoc
