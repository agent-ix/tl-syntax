# =============================================================================
# TL Syntax Makefile
# =============================================================================

CARGO ?= cargo

.PHONY: help
help:
	@echo "Available targets:"
	@echo "  make fmt              - Format with rustfmt"
	@echo "  make fmt-check        - Verify formatting (CI gate)"
	@echo "  make lint             - Clippy with -D warnings"
	@echo "  make test             - cargo test"
	@echo "  make check-features   - check no-default, alloc, serde, and all features"
	@echo "  make check-corpus     - verify retained corpus SHA-256 digests"
	@echo "  make spec             - validate the specification with Quire"
	@echo "  make evidence-tool    - syntax-check the PGM-01 evidence tooling"
	@echo "  make build            - Release build"
	@echo "  make clean            - cargo clean"
	@echo "  make deny             - cargo deny check licenses"
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

.PHONY: check-features
check-features:
	$(CARGO) check --lib --no-default-features
	$(CARGO) check --lib --no-default-features --features alloc
	$(CARGO) check --lib --no-default-features --features serde
	$(CARGO) check --lib --all-features

.PHONY: check-corpus
check-corpus:
	sha256sum --check corpus/SHA256SUMS

.PHONY: spec
spec:
	quire validate --scope . 'spec/**/*.md'

.PHONY: evidence-tool
evidence-tool:
	python3 -m py_compile scripts/build_evidence_envelope.py scripts/validate_json_schema.py

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
	$(CARGO) deny check licenses

.PHONY: cargo-audit
cargo-audit:
	$(CARGO) audit

.PHONY: audit-unsafe
audit-unsafe:
	bash scripts/check_unsafe_comments.sh

# =============================================================================
# Composite
# =============================================================================

.PHONY: ci
ci: fmt-check check-features lint test check-corpus deny audit-unsafe evidence-tool
