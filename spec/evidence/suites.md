---
id: SUR-001
title: tl-syntax v0.1 evidence suite registry
type: SuiteRegistry
---

# tl-syntax v0.1 evidence suite registry

## Suites

| ID | Name | Command | Tool | Evidence Kind |
|---|---|---|---|---|
| SUITE-001 | Shared temporal corpus conformance | `cargo run --example corpus_conformance --features serde -- --manifest corpus/manifest.json` | tl-syntax corpus conformance runner (the crate itself) | Integration |
| SUITE-002 | Strict specification validation | `quire validate --scope . 'spec/**/*.md' --strict --summary` | quire 0.31.0 / quire-rs 0.46.0 | Analysis |
| SUITE-003 | Static specification and coverage export | `quire coverage --scope . --json` | quire 0.31.0 / quire-rs 0.46.0 | Static |
| SUITE-004 | Public API documentation | `RUSTDOCFLAGS='-D warnings' cargo doc --no-deps --all-features` | rustdoc | Static |
| SUITE-005 | Corpus schema, derived horizon, and closed-trace oracle | `python3 scripts/validate_corpus.py --json` | Python jsonschema Draft 7, tl-syntax corpus oracle | Analysis |
| SUITE-006 | Shared assurance intake chain | `python3 scripts/assurance_chain.py --candidate-revision <sha>` | quoin 0.23.1 change-assurance and evidence surfaces | Integration |

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
