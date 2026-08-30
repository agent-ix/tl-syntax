---
id: SUR-001
title: tl-syntax v0.1 evidence suite registry
type: SuiteRegistry
---

# tl-syntax v0.1 evidence suite registry

## Suites

| ID | Name | Command | Tool | Evidence Kind |
|---|---|---|---|---|
| SUITE-001 | Complete repository CI | `make ci` | GNU Make, Cargo, rustfmt, clippy, cargo-deny | Integration |
| SUITE-002 | Strict specification validation | `quire validate --scope . 'spec/**/*.md' --strict --summary` | quire 0.31.0 / quire-rs 0.46.0 | Analysis |
| SUITE-003 | Strict requirement coverage | `quire coverage --scope . --strict` | quire 0.31.0 / quire-rs 0.46.0 | Analysis |
| SUITE-004 | Public API documentation | `RUSTDOCFLAGS='-D warnings' cargo doc --no-deps --all-features` | rustdoc | Static |
| SUITE-005 | Corpus byte integrity | `sha256sum --check corpus/SHA256SUMS` | GNU coreutils | Static |
