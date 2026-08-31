# tl-syntax

A no_std syntax tree and semantic profile model for Mission-time Linear Temporal Logic.

## Commands

```bash
make fmt            # format with rustfmt
make fmt-check      # verify formatting (CI gate)
make lint           # clippy with -D warnings
make test           # cargo test
make build          # release build
make clean          # cargo clean
make deny           # cargo deny check licenses
make audit-unsafe   # check that every unsafe block has a // SAFETY: comment
make check-corpus   # validate corpus schemas, semantics, and checksums
make spec           # validate specification structure and traceability
make verify-evidence # verify every retained record and committed anchor
make ci             # complete local gate set (hosted CI is manual-only)
```

## Safety scaffolding

Backported from `agent-ix/ecaz`:

- `clippy.toml` pins MSRV to `1.75` and caps cognitive complexity / arg count
- `deny.toml` allow-lists licenses and denies unknown registries/git sources
- `scripts/check_unsafe_comments.sh` runs in CI and locally via `make audit-unsafe`. Every `unsafe {` block must have a `// SAFETY:` comment within the 3 preceding lines, or be listed in `scripts/unsafe_comment_baseline.txt`. Update the baseline with `bash scripts/check_unsafe_comments.sh --update-baseline`.
- `rustfmt.toml` uses only stable 100-character-width settings. CI fails on drift.
- `rust-toolchain.toml` pins to stable + rustfmt + clippy.

## Layout

```
src/lib.rs             # crate root
src/document.rs        # bounded owned/wire documents (alloc + serde)
tests/integration.rs   # end-to-end tests
corpus/                # pinned formula schemas, fixtures, traces, and oracles
evidence/              # retained records and Git/PR-review integrity anchors
schemas/               # evidence input and manifest schemas
spec/                  # requirements, plans, reviews, assurance, and test matrix
scripts/               # local tooling
```
