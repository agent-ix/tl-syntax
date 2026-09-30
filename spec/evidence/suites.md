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
| SUITE-002 | Strict specification validation | `quire validate --scope . 'spec/**/*.md' --strict --summary` | quire | Analysis |
| SUITE-003 | Static specification and coverage export | `quire coverage --scope . --json` | quire | Static |
| SUITE-004 | Public API documentation | `RUSTDOCFLAGS='-D warnings' cargo doc --no-deps --all-features` | rustdoc | Static |
| SUITE-005 | Corpus schema, derived horizon, and closed-trace oracle | `python3 scripts/validate_corpus.py` | Python jsonschema Draft 7, tl-syntax corpus oracle | Analysis |
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

SUITE-009 through SUITE-013 are planned M6 producer identities, not present
executables and not current evidence. SUITE-011 remains blocked on the
`tl-syntax#16` accepted release set. SUITE-012 remains blocked on a compatible
released integrator-package contract. The durable-retention branch of
SUITE-011 remains blocked until a shared backend/operator, immutable handle and
lifecycle pass TC-067. Every P0 M6 suite requires a load-bearing negative
mutation before its positive result may support a source-readiness claim.
