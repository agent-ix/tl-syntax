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
| SUITE-007 | Retained-evidence compatibility view | `python3 scripts/legacy_evidence_view.py` | engineering-assurance 0.2.0 `map_pgm01_bytes` | Static |

## Notes

SUITE-001 was `make ci` when this repository ran its own collector. It is now the
domain conformance runner, because a suite whose command is "everything" cannot
say which obligation a result discharged, and `make ci` is a gate rather than a
producer of transcribable results.

SUITE-003 was a repository-local traceability reimplementation. Quire is the
authority on static specification, obligation and coverage facts, so the suite
now names Quire's own export.

SUITE-006 and SUITE-007 were the PGM-01 evidence schema and envelope conformance
checks run by this repository's deleted collector. Both concerns moved upstream:
Quoin owns intake, retention, and receipts, and Engineering Assurance owns the
read-only mapping of retained bytes. The identifiers are reused here deliberately
and the change is recorded in this note, because these two rows never appeared in
a retained record's discharged-obligation list.
