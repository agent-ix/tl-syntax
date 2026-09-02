# =============================================================================
# TL Syntax Makefile
# =============================================================================
#
# Native orchestration. Every target calls the toolchain that owns the job:
# cargo for the crate, the corpus conformance runner for the shared temporal
# corpus, quire for static export, quoin for evidence. Nothing here computes a
# verdict, attests to its own correctness, or retains evidence of its own.
#
# This file is not a trust root and no longer tries to be one. The gates that
# used to police Make's own execution controls went with the collector they were
# protecting, and nothing replaced them.
#
# What the structural replacement does cover: the targets that feed the
# assurance chain. Quoin binds its retained inputs by digest and the chain
# derives every attested result from the producer's own bytes, so a recipe that
# lies about running `assurance-inputs` yields an absent or empty input, which
# the chain reports as an error naming the missing target rather than as a pass.
#
# What it does not cover: the targets that feed nothing. `fmt-check`, `lint`,
# `deny`, `audit-unsafe` and `rustdoc` are among the `ci` prerequisites whose
# output no attestation reads -- the charted producers are exactly the five
# files `scripts/assurance_chain.py` lists in `INPUTS`. For everything else
# there is no record to contradict, so a `-` prefix, a `.IGNORE:` line or a
# `SHELL := /usr/bin/true` assignment neuters the check and it stays green.
# That gap is recorded, not closed, in agent-ix/tl-syntax#11 by owner decision,
# and #11 also owns measuring it here; do not re-add the guard.

CARGO ?= cargo
PYTHON ?= python3
QUIRE ?= quire
QUOIN ?= quoin

# The shared-assurance lane runs in its own interpreter. engineering-assurance
# declares jsonschema>=4.23 and this repository's Draft 7 corpus lane pins
# 3.2.0; both are right for their own job, so they get one environment each.
ASSURANCE_VENV ?= .venv-assurance
ASSURANCE_PYTHON ?= $(ASSURANCE_VENV)/bin/python

ASSURANCE_DIR := target/assurance
CONFORMANCE_RESULT := $(ASSURANCE_DIR)/corpus-conformance.jsonl
ORACLE_RESULT := $(ASSURANCE_DIR)/corpus-oracle.json
FEATURE_RESULT := $(ASSURANCE_DIR)/feature-boundary.json
QUIRE_EXPORT := $(ASSURANCE_DIR)/quire-static-export.json
MSRV_RESULT := $(ASSURANCE_DIR)/msrv.jsonl
REVISION ?= $(shell git rev-parse HEAD)

.PHONY: help
help:
	@echo "Available targets:"
	@echo "  make fmt              - Format with rustfmt"
	@echo "  make fmt-check        - Verify formatting (CI gate)"
	@echo "  make lint             - Clippy with -D warnings"
	@echo "  make test             - cargo test plus the shared-assurance tests"
	@echo "  make check-features   - check no-default, alloc, serde, and all features"
	@echo "  make check-corpus     - verify corpus digests, schemas, and derived oracles"
	@echo "  make conformance      - replay the shared temporal corpus through the crate"
	@echo "  make spec             - validate the specification with Quire"
	@echo "  make msrv             - test all targets and features with Rust 1.75"
	@echo "  make rustdoc          - build warning-free public documentation"
	@echo "  make build            - Release build"
	@echo "  make clean            - cargo clean and drop the assurance environment"
	@echo "  make deny             - run all declared cargo-deny policy checks"
	@echo "  make audit-unsafe     - Enforce // SAFETY: comments on unsafe blocks"
	@echo "  make assurance-env    - create the pinned shared-assurance interpreter"
	@echo "  make assurance-inputs - run the producers and write their structured results"
	@echo "  make pins             - classify the toolchain through the shared matrix"
	@echo "  make assurance-chain  - seal, retain, and verify through Quoin"
	@echo "  make assurance        - pins + assurance-chain"
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

# The traced tests invoke the assurance gates, so the producers must already have
# run. They are a prerequisite rather than something a test creates for itself: a
# test that can produce its own inputs can produce a green run out of nothing.
.PHONY: test
test: assurance-inputs
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
	$(PYTHON) scripts/validate_corpus.py
	$(PYTHON) scripts/test_corpus_gate.py

.PHONY: spec
spec:
	$(QUIRE) validate --scope . 'spec/**/*.md' --strict --summary
	$(QUIRE) coverage --scope . --strict

.PHONY: build
build:
	$(CARGO) build --release

.PHONY: clean
clean:
	$(CARGO) clean
	rm -rf $(ASSURANCE_VENV)

# =============================================================================
# Supply chain & safety
# =============================================================================

.PHONY: deny
deny:
	$(CARGO) deny check advisories
	$(CARGO) deny check bans
	$(CARGO) deny check licenses
	$(CARGO) deny check sources

.PHONY: audit-unsafe
audit-unsafe:
	bash scripts/check_unsafe_comments.sh

.PHONY: msrv
msrv:
	rustup run 1.75.0 $(CARGO) test --all-features

.PHONY: rustdoc
rustdoc:
	RUSTDOCFLAGS=-Dwarnings $(CARGO) doc --no-deps --all-features

# =============================================================================
# Shared assurance
# =============================================================================

$(ASSURANCE_PYTHON):
	$(PYTHON) -m venv $(ASSURANCE_VENV)
	$(ASSURANCE_VENV)/bin/pip install --quiet --disable-pip-version-check \
		-r requirements-assurance.txt

.PHONY: assurance-env
assurance-env: $(ASSURANCE_PYTHON)

# The only target that runs a producer. Everything downstream consumes these
# files and refuses to create them.
.PHONY: assurance-inputs
assurance-inputs: assurance-env
	mkdir -p $(ASSURANCE_DIR)
	$(CARGO) run --quiet --example corpus_conformance --features serde -- \
		--manifest corpus/manifest.json > $(CONFORMANCE_RESULT)
	$(PYTHON) scripts/validate_corpus.py --json > $(ORACLE_RESULT)
	$(PYTHON) scripts/check_default_dependencies.py --json > $(FEATURE_RESULT)
	$(QUIRE) coverage --scope . --json > $(QUIRE_EXPORT)
	rustup run 1.75.0 $(CARGO) check --locked --all-targets --all-features \
		--message-format=json > $(MSRV_RESULT)

.PHONY: pins
pins: assurance-env
	$(ASSURANCE_PYTHON) scripts/check_shared_pins.py

.PHONY: assurance-chain
assurance-chain: assurance-inputs
	$(PYTHON) scripts/assurance_chain.py --candidate-revision $(REVISION)

.PHONY: assurance
assurance: pins assurance-chain

# An operator target, not a CI gate. It writes into this repository's own Quoin
# evidence store, which is a reviewed change to spec/evidence/ rather than
# something a gate should do on every run.
.PHONY: assurance-record
assurance-record: assurance-inputs
	$(PYTHON) scripts/assurance_chain.py --adapt $(CONFORMANCE_RESULT) \
		> $(ASSURANCE_DIR)/entries.json
	$(QUOIN) evidence record \
		--repo . \
		--suite SUITE-001 \
		--commit $(REVISION) \
		--tool "tl-syntax-corpus-conformance 0.1.0" \
		--adapter entries \
		--kind Integration \
		--results $(ASSURANCE_DIR)/entries.json

# =============================================================================
# Composite
# =============================================================================

.PHONY: ci
ci: fmt-check check-features check-default-dependencies lint test check-corpus \
	conformance deny audit-unsafe spec msrv rustdoc assurance
