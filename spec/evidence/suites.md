---
id: SUR-001
title: tl-syntax evidence suite registry
type: SuiteRegistry
---

# tl-syntax evidence suite registry

## Suites

| ID | Name | Command | Tool | Evidence Kind |
|---|---|---|---|---|
| SUITE-001 | Shared temporal corpus conformance | `cargo run --example corpus_conformance --features serde -- --manifest corpus/manifest.json` | tl-syntax corpus conformance runner (the crate itself) | Integration |
| SUITE-002 | Strict specification validation | `quire validate --scope . 'spec/**/*.md' --strict --summary` | quire 0.31.0 / quire-rs 0.46.0 | Analysis |
| SUITE-003 | Static specification and coverage export | `quire coverage --scope . --json` | quire 0.31.0 / quire-rs 0.46.0 | Static |
| SUITE-004 | Public API documentation | `RUSTDOCFLAGS='-D warnings' cargo doc --no-deps --all-features` | rustdoc | Static |
| SUITE-005 | Corpus schema, derived horizon, and closed-trace oracle | `python3 scripts/validate_corpus.py --json` | Python jsonschema Draft 7, tl-syntax corpus oracle | Analysis |
| SUITE-006 | Shared assurance intake chain | `python3 scripts/assurance_chain.py --candidate-revision <sha>` | quoin 0.23.1 change-assurance and evidence surfaces | Integration |
| SUITE-008 | Shared assurance contract tests | `cargo test --test shared_assurance --all-features` | cargo/rustc; Git supplies the version-control path inventory | Integration |
| SUITE-009 | Source-readiness property and state-model tests | `cargo test --test source_readiness_properties --all-features` | planned Rust proptest/state-machine harness | Property |
| SUITE-010 | Source-readiness fault and freshness tests | `cargo test --test source_readiness_failures --all-features` | planned Rust fault-injection harness | Integration |
| SUITE-011 | Real shared source-readiness contract tests | `cargo test --test source_readiness_shared --all-features -- --ignored` | planned Rust integration harness plus released Engineering Assurance/Quire/Quoin | Integration |
| SUITE-012 | Real integrator-package contract tests | `cargo test --test integrator_package --all-features -- --ignored` | planned Rust contract/state/concurrency harness plus released shared package | Integration |
| SUITE-013 | Rust dependency license/source facts | `cargo deny check licenses sources` | cargo-deny input to the independent TC-072 rights-source review; not a complete material-population disposition by itself | Static |

## Notes

SUITE-001 was `make ci` when this repository ran its own collector. It is now the
domain conformance runner, because a suite whose command is "everything" cannot
say which obligation a result discharged, and `make ci` is a gate rather than a
producer of transcribable results.

SUITE-003 was a repository-local traceability reimplementation. Quire is the
authority on static specification, obligation and coverage facts, so the suite
now names Quire's own export.

SUITE-006 and SUITE-007 were originally the PGM-01 evidence schema and envelope
conformance checks run by this repository's deleted collector. `#9` reused both
identifiers rather than minting new ones, because neither row had ever appeared
in a retained record's discharged-obligation list: `SUITE-006` became the shared
assurance intake chain and `SUITE-007` became the read-only compatibility view.

**`SUITE-007` is now retired, and its identifier is not reused a second time.**
The compatibility view read this repository's retained evidence, and that
evidence was deleted under `agent-ix/tl-syntax#12`, on the preservation
constraint `agent-ix/engineering-assurance#7` released for the pre-stable phase
on 2026-09-02 — so the suite has no subject. `SR-008` names `SUITE-007` by that
identifier, so binding it to a third meaning would make that closed review
unreadable. The retirement is recorded alongside `FR-006-AC-4` and `TC-024` in
[`FR-006`](../requirements/FR-006-shared-assurance-intake.md). This repository
now retains no evidence of its own.

SUITE-008 is a local verification suite, not a structured-result producer and
not an attestation input to SUITE-006. Its TC-021, TC-022, TC-023, TC-025,
TC-026, TC-034, TC-035, and TC-038
outcomes are reported by the local gate and exact-head pull-request record; the
Quoin record does not claim those tests ran. Git is required because TC-034
compares the checked-out version-control inventory with the reviewed live path
set and refuses execution outside a repository boundary.

[NFR-003](../requirements/NFR-003-qualification-integrity.md) owns this identity
and lifecycle boundary. TC-038 checks the exact command and verifies that the
change-assurance proof declarations do not convert SUITE-008 into an attested
proof input. Independent review, not that self-check, establishes whether the
suite ran at the reported exact head.

SUITE-009 through SUITE-013 are planned M6 producer identities, not present
executables and not current evidence. SUITE-011 remains blocked on the
`tl-syntax#16` accepted release set. SUITE-012 remains blocked on a compatible
released integrator-package contract. The durable-retention branch of
SUITE-011 remains blocked until a shared backend/operator, immutable handle and
lifecycle pass TC-067. Every P0 M6 suite requires a load-bearing negative
mutation before its positive result may support a source-readiness claim.
