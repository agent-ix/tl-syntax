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
make assurance        # pins + the Quoin chain
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

This repository retains no evidence. The 23 `quire.derivation-evidence/v1`
records its pre-migration collector wrote, the two schemas frozen because those
records named them by digest, and the read-only compatibility view over them
were all deleted under
[issue #12](https://github.com/agent-ix/tl-syntax/issues/12), on the
preservation constraint `agent-ix/engineering-assurance#7` released for the
pre-stable phase. Deleted, not rewritten — no claim that historical evidence
still verifies survives them. The constraint re-applies at the move toward
stable releases.

The Makefile is orchestration and is not a trust root. For the targets that feed
the assurance chain, Quoin binds its retained inputs by digest, so a Makefile
that misreports what it ran yields an absent or empty input rather than a pass.
For the targets that feed nothing — `fmt-check`, `lint`, `deny`, `audit-unsafe`
and `rustdoc` among them — there is no record to contradict and no guard; that
gap is measured and recorded, not closed. At base `4cb5787`, an invalid Rust
item made the no-`.IGNORE:` control exit 2; with global `.IGNORE:`, eight of the
thirteen `ci` prerequisite paths emitted ignored failures while Make treated all
thirteen as successful and exited 0. The per-prerequisite result is in
[`SR-013`](./spec/reviews/SR-013-make-execution-control-measurement.md) and the
owner decision is tracked in
[issue #11](https://github.com/agent-ix/tl-syntax/issues/11).

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
corpus/                        # pinned formula schemas, fixtures, traces, and oracles
assurance/                     # the change declaration and the adopted release pins
spec/                          # requirements, plans, reviews, and the test matrix
scripts/                       # domain gates and the shared-assurance driver
```
