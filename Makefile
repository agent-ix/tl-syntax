# =============================================================================
# TL Syntax Makefile
# =============================================================================

CARGO ?= cargo
PYTHON ?= python3
QUIRE ?= quire
SHA256SUM ?= sha256sum
BASH ?= bash

.PHONY: help
help:
	@echo "Available targets:"
	@echo "  make fmt              - Format with rustfmt"
	@echo "  make fmt-check        - Verify formatting (CI gate)"
	@echo "  make lint             - Clippy with -D warnings"
	@echo "  make test             - cargo test"
	@echo "  make check-failure-propagation - prove every mandatory gate propagates failures"
	@echo "  make check-features   - check no-default, alloc, serde, and all features"
	@echo "  make check-default-dependencies - require an empty default dependency graph"
	@echo "  make check-corpus     - verify retained corpus SHA-256 digests"
	@echo "  make verify-evidence  - verify every retained evidence SHA-256 manifest"
	@echo "  make spec             - validate the specification with Quire"
	@echo "  make evidence-tool    - syntax-check the PGM-01 evidence tooling"
	@echo "  make build            - Release build"
	@echo "  make clean            - cargo clean"
	@echo "  make deny             - run all declared cargo-deny policy checks"
	@echo "  make audit-unsafe     - Enforce // SAFETY: comments on unsafe blocks"
	@echo "  make ci               - All CI gates locally (fmt-check + lint + test + deny + audit-unsafe)"

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

.PHONY: check-failure-propagation
check-failure-propagation:
	$(PYTHON) scripts/check_failure_propagation.py

.PHONY: check-features
check-features:
	$(CARGO) check --lib --no-default-features
	$(CARGO) check --lib --no-default-features --features alloc
	$(CARGO) check --lib --no-default-features --features serde
	$(CARGO) check --lib --all-features

.PHONY: check-default-dependencies
check-default-dependencies:
	$(PYTHON) scripts/check_default_dependencies.py

.PHONY: check-corpus
check-corpus:
	$(SHA256SUM) --check corpus/SHA256SUMS
	$(PYTHON) scripts/validate_corpus.py

.PHONY: verify-evidence
verify-evidence:
	$(BASH) scripts/verify_evidence.sh

.PHONY: spec
spec:
	$(QUIRE) validate --scope . 'spec/**/*.md' --strict --summary
	$(PYTHON) scripts/check_traceability_coverage.py

.PHONY: evidence-tool
evidence-tool:
	$(PYTHON) -m py_compile scripts/build_evidence_envelope.py scripts/check_default_dependencies.py scripts/check_failure_propagation.py scripts/check_traceability_coverage.py scripts/finalize_collection.py scripts/test_corpus_gate.py scripts/test_evidence_tool.py scripts/test_failure_propagation.py scripts/test_json_schema_gate.py scripts/test_traceability_gate.py scripts/validate_corpus.py scripts/validate_json_schema.py scripts/verify_evidence_manifest.py
	$(PYTHON) scripts/test_corpus_gate.py
	$(PYTHON) scripts/test_evidence_tool.py
	$(PYTHON) scripts/test_failure_propagation.py
	$(PYTHON) scripts/test_json_schema_gate.py
	$(PYTHON) scripts/test_traceability_gate.py

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

.PHONY: cargo-audit
cargo-audit:
	$(CARGO) audit

.PHONY: audit-unsafe
audit-unsafe:
	$(BASH) scripts/check_unsafe_comments.sh

# =============================================================================
# Composite
# =============================================================================

.PHONY: ci
ci: check-failure-propagation fmt-check check-features check-default-dependencies lint test check-corpus deny audit-unsafe evidence-tool spec verify-evidence
