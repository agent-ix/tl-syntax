# tl-syntax

A no_std syntax tree and semantic profile model for Mission-time Linear Temporal Logic.

## Commands

```bash
make fmt              # format with rustfmt
make fmt-check        # verify formatting (CI gate)
make lint             # clippy with -D warnings
make test             # cargo test
make build            # release build
make clean            # cargo clean
make deny             # cargo-deny advisories, bans, licenses, and sources
make audit-unsafe     # enforce the unsafe-code policy guard
make check-corpus     # corpus digests, schemas, derived oracles, and their mutation probe
make conformance      # replay the shared temporal corpus through the crate
make spec             # validate the specification with Quire
make msrv             # test every target and feature at Rust 1.98.1
make ci               # complete local gate set (hosted CI is manual-only)
```

## Specification workflow

All new or changed work must be specified before it is implemented: use
`quoin write` to obtain the current authoritative artifact contracts, then add
or update the relevant requirements, plans, tasks, matrix rows, and evidence
links. Before requesting review, run `quoin review` over the affected scope and
validate the resulting artifacts with Quire. Record the selected analyses and
their findings; do not advance Quoin's final `validated` to `accepted` gate,
which remains a human decision.

## Assurance

This repository retains no evidence. The 23 `quire.derivation-evidence/v1`
records its pre-migration collector wrote, the two schemas frozen because those
records named them by digest, and the read-only compatibility view over them
were all deleted under
[issue #12](https://github.com/agent-ix/tl-syntax/issues/12), on the
preservation constraint `agent-ix/engineering-assurance#7` released for the
pre-stable phase. Deleted, not rewritten — no claim that historical evidence
still verifies survives them. The constraint re-applies at the move toward
stable releases.

## Safety scaffolding

Backported from `agent-ix/ecaz`:

- `clippy.toml` pins MSRV to `1.98.1` and caps cognitive complexity / arg count
- `deny.toml` allow-lists licenses and denies unknown registries/git sources
- `scripts/check_unsafe_comments.sh` runs in CI and locally via `make audit-unsafe`. Every `unsafe {` block must have a `// SAFETY:` comment within the 3 preceding lines, or be listed in `scripts/unsafe_comment_baseline.txt`. Update the baseline with `bash scripts/check_unsafe_comments.sh --update-baseline`.
- `rustfmt.toml` uses only stable 100-character-width settings.
- `Cargo.toml` declares Rust 1.98.1 as the MSRV; `make msrv` checks every target
  and feature at that version while `rust-toolchain.toml` pins the same exact
  version for rustfmt and clippy.

## Layout

```
src/lib.rs                     # crate root
src/document.rs                # bounded owned/wire documents (alloc + serde)
src/future.rs                  # allocation-free W/M admission, lowering, and reports
src/syntax.rs                  # intervals, spans, nodes, profiles, borrowed validation
examples/corpus_conformance.rs # the domain conformance runner over the shared corpus
tests/integration.rs           # end-to-end domain tests
tests/future_lowering.rs       # FR-008 traced lowering and refusal tests
tests/future_operator_corpus.rs # TC-074 replay of the paired W/M corpus
corpus/                        # pinned formula schemas, fixtures, traces, and oracles
corpus/future-operators/       # digest-pinned paired W/M source and canonical-graph corpus
spec/                          # requirements, plans, reviews, and the test matrix
scripts/                       # domain gates
```
