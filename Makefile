# =============================================================================
# TL Syntax Makefile
# =============================================================================

CARGO ?= cargo
PYTHON ?= python3
QUIRE ?= quire
SHA256SUM ?= sha256sum
BASH ?= bash

ifneq ($(filter ci ci-for-evidence,$(MAKECMDGOALS)),)
# Parallelism/output flags are harmless; dry-run, touch, eval, and other
# command-changing MAKEFLAGS are refused because this target is an execution gate.
tl_ci_unsafe_makeflags := $(filter-out j% -j% l% -l% O% -O% w -w --jobs% --jobserver-auth=% --jobserver-fds=% --load-average% --output-sync% --print-directory --no-print-directory,$(MAKEFLAGS))
ifneq ($(strip $(tl_ci_unsafe_makeflags)),)
$(error local CI refuses MAKEFLAGS that alter command execution)
endif
ifneq ($(strip $(PYTHONOPTIMIZE)),)
$(error local CI refuses optimized Python policy execution)
endif
ifneq ($(strip $(RUSTUP_TOOLCHAIN)$(RUSTUP_HOME)$(CARGO_HOME)$(RUSTC)$(RUSTDOC)$(RUSTC_WRAPPER)$(RUSTC_WORKSPACE_WRAPPER)$(RUSTFLAGS)$(CARGO_ENCODED_RUSTFLAGS)$(RUSTDOCFLAGS)$(LD_PRELOAD)$(LD_LIBRARY_PATH)$(PYTHONPATH)),)
$(error local CI refuses ambient compiler, loader, or Python-path overrides)
endif
ifneq ($(filter ci-for-evidence,$(MAKECMDGOALS)),)
tl_ci_qualified_target := $(shell /usr/bin/python3 scripts/tool_identity.py --cargo-target-dir)
ifneq ($(strip $(CARGO_TARGET_DIR)),)
ifneq ($(CARGO_TARGET_DIR),$(tl_ci_qualified_target))
$(error candidate CI refuses an unqualified CARGO_TARGET_DIR)
endif
else
export CARGO_TARGET_DIR := $(tl_ci_qualified_target)
endif
endif
ifneq ($(notdir $(CARGO)),cargo)
$(error local CI refuses a CARGO override)
endif
ifneq ($(notdir $(PYTHON)),python3)
$(error local CI refuses a PYTHON override)
endif
ifneq ($(notdir $(QUIRE)),quire)
$(error local CI refuses a QUIRE override)
endif
ifneq ($(notdir $(SHA256SUM)),sha256sum)
$(error local CI refuses a SHA256SUM override)
endif
ifneq ($(notdir $(BASH)),bash)
$(error local CI refuses a BASH override)
endif
tl_ci_static_status := $(shell /usr/bin/env -u PYTHONOPTIMIZE MAKEFLAGS= /usr/bin/python3 scripts/check_failure_propagation.py --makefile '$(firstword $(MAKEFILE_LIST))' --static-only >/dev/null; echo $$?)
ifneq ($(tl_ci_static_status),0)
$(error local CI refuses unsafe Make recipe controls)
endif
endif

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
	@echo "  make msrv             - test all targets and features with Rust 1.75"
	@echo "  make rustdoc          - build warning-free public documentation"
	@echo "  make build            - Release build"
	@echo "  make clean            - cargo clean"
	@echo "  make deny             - run all declared cargo-deny policy checks"
	@echo "  make audit-unsafe     - Enforce // SAFETY: comments on unsafe blocks"
	@echo "  make ci               - All CI gates locally (fmt-check + lint + test + deny + audit-unsafe)"
	@echo "  make ci-for-evidence  - Host-qualified candidate gates before self-binding"

# =============================================================================
# Format / Lint / Test
# =============================================================================

.PHONY: fmt
fmt:
	cargo fmt --all

.PHONY: fmt-check
fmt-check:
	cargo fmt --all -- --check

.PHONY: lint
lint:
	cargo clippy --all-targets --all-features -- -D warnings

.PHONY: test
test:
	cargo test --all-features

.PHONY: check-failure-propagation
check-failure-propagation:
	/usr/bin/python3 scripts/check_failure_propagation.py

.PHONY: check-tool-identities
check-tool-identities:
	/usr/bin/env PATH="$$(/usr/bin/python3 scripts/tool_identity.py --trusted-path)" /usr/bin/python3 scripts/tool_identity.py --verify-live

.PHONY: check-features
check-features:
	cargo check --lib --no-default-features
	cargo check --lib --no-default-features --features alloc
	cargo check --lib --no-default-features --features serde
	cargo check --lib --all-features

.PHONY: check-default-dependencies
check-default-dependencies:
	/usr/bin/python3 scripts/check_default_dependencies.py

.PHONY: check-corpus
check-corpus:
	/usr/bin/sha256sum --check corpus/SHA256SUMS
	/usr/bin/python3 scripts/validate_corpus.py

.PHONY: verify-evidence
verify-evidence:
	/usr/bin/python3 scripts/check_evidence_shell_contract.py
	/usr/bin/bash scripts/verify_evidence.sh

.PHONY: spec
spec:
	quire validate --scope . 'spec/**/*.md' --strict --summary
	/usr/bin/python3 scripts/check_traceability_coverage.py

.PHONY: evidence-tool
evidence-tool:
	/usr/bin/python3 -m compileall -q scripts
	/usr/bin/python3 scripts/run_policy_tests.py

.PHONY: build
build:
	cargo build --release

.PHONY: clean
clean:
	cargo clean

# =============================================================================
# Supply chain & safety
# =============================================================================

.PHONY: deny
deny:
	cargo deny check advisories
	cargo deny check bans
	cargo deny check licenses
	cargo deny check sources

.PHONY: cargo-audit
cargo-audit:
	cargo audit

.PHONY: audit-unsafe
audit-unsafe:
	/usr/bin/bash scripts/check_unsafe_comments.sh

.PHONY: msrv
msrv:
	rustup run 1.75.0 cargo test --all-features

.PHONY: rustdoc
rustdoc:
	RUSTDOCFLAGS=-Dwarnings cargo doc --no-deps --all-features

# =============================================================================
# Composite
# =============================================================================

.PHONY: ci ci-for-evidence
ci-for-evidence: check-failure-propagation check-tool-identities fmt-check check-features check-default-dependencies lint test check-corpus deny audit-unsafe evidence-tool spec msrv rustdoc

ci: check-failure-propagation fmt-check check-features check-default-dependencies lint test check-corpus deny audit-unsafe evidence-tool spec msrv rustdoc verify-evidence
