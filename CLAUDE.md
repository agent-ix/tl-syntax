# tl-syntax

A no_std syntax tree and semantic profile model for Mission-time Linear Temporal Logic.

## Commands

```bash
make fmt              # format with rustfmt
make fmt-check        # verify formatting (CI gate)
make lint             # clippy with -D warnings
make test             # cargo test, after the producers have run
make build            # release build
make clean            # cargo clean and drop the assurance environment
make deny             # cargo-deny advisories, bans, licenses, and sources
make audit-unsafe     # enforce the unsafe-code policy guard
make check-corpus     # corpus digests, schemas, derived oracles, and their mutation probe
make conformance      # replay the shared temporal corpus through the crate
make spec             # validate the specification with Quire
make msrv             # test every target and feature at Rust 1.75
make assurance-env    # build the pinned shared-assurance interpreter
make assurance-inputs # run the producers and write their structured results
make assurance        # pins + compatibility view + the Quoin chain
make ci               # complete local gate set (hosted CI is manual-only)
```

## Assurance

This repository produces verification results with its own tools and hands them
to the released Engineering Assurance, Quire, and Quoin contracts. It keeps no
evidence framework of its own — no runner, envelope, manifest, tool-identity
lock, retention store, audit store, anchor file, or aggregate verdict. See
[`assurance/README.md`](./assurance/README.md) and
[`spec/requirements/FR-006-shared-assurance-intake.md`](./spec/requirements/FR-006-shared-assurance-intake.md).

Two Python lanes, deliberately:

- `.venv-assurance` holds `engineering-assurance` at the `v0.2.0` tag, which
  declares `jsonschema>=4.23`. Build it with `make assurance-env`.
- The corpus gate's Draft 7 lane pins `jsonschema==3.2.0`.

Both are right for their own job and neither may be bent to fit the other, so
they get one environment each.

`evidence/` holds 23 immutable retained records from the pre-migration collector.
They are read only through
`engineering_assurance.verification_semantics.map_pgm01_bytes` and are never
written. `evidence/README.md` describes the collector that produced them; it is
retained unchanged as part of the record and does not describe how this
repository works today. The gate proves the read-only claim by digesting the
whole tree before and after every run.

`schemas/` holds two frozen schemas. Nothing validates against them; they exist
because retained envelopes name them by digest. See
[`schemas/README.md`](./schemas/README.md).

The Makefile is orchestration and is not a trust root. Quoin binds its retained
inputs by digest, so a Makefile that misreports what it ran cannot make a sealed
attestation say otherwise.

## Safety scaffolding

Backported from `agent-ix/ecaz`:

- `clippy.toml` pins MSRV to `1.75` and caps cognitive complexity / arg count
- `deny.toml` allow-lists licenses and denies unknown registries/git sources
- `scripts/check_unsafe_comments.sh` runs in CI and locally via `make audit-unsafe`. Every `unsafe {` block must have a `// SAFETY:` comment within the 3 preceding lines, or be listed in `scripts/unsafe_comment_baseline.txt`. Update the baseline with `bash scripts/check_unsafe_comments.sh --update-baseline`.
- `rustfmt.toml` uses only stable 100-character-width settings.
- `Cargo.toml` declares Rust 1.75 as the MSRV; `make msrv` checks every target
  and feature at that version while `rust-toolchain.toml` selects stable rustfmt
  and clippy.

## Layout

```
src/lib.rs                     # crate root
src/document.rs                # bounded owned/wire documents (alloc + serde)
src/syntax.rs                  # intervals, spans, nodes, profiles, borrowed validation
examples/corpus_conformance.rs # the domain conformance runner over the shared corpus
tests/integration.rs           # end-to-end domain tests
tests/shared_assurance.rs      # FR-006 traced tests over the shared intake path
tests/fixtures/legacy-compat/  # one-named-edit fixtures for the compatibility view
corpus/                        # pinned formula schemas, fixtures, traces, and oracles
assurance/                     # the change declaration and the adopted release pins
evidence/                      # immutable retained records; read-only, never written
schemas/                       # frozen evidence schemas; referenced by nothing
spec/                          # requirements, plans, reviews, and the test matrix
scripts/                       # domain gates and the shared-assurance driver
```
