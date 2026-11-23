---
id: Plan-022
title: "Unbuilt release train deferred ledger"
type: Plan
status: draft
relationships:
  - target: ix://agent-ix/tl-syntax/FR-362
    type: references
  - target: ix://agent-ix/tl-syntax/FR-034
    type: references
  - target: ix://agent-ix/tl-syntax/FR-302
    type: references
  - target: ix://agent-ix/tl-syntax/FR-306
    type: references
  - target: ix://agent-ix/tl-syntax/FR-309
    type: references
  - target: ix://agent-ix/tl-syntax/FR-332
    type: references
---
# Implementation Plan: unbuilt release train deferred ledger

Branch `TL-code-unbuilt` implements TL-394, TL-414, TL-420, TL-424, TL-425 and
TL-428 without any build (decision rules of 2026-11-23). The owning plans
(PLAN-021 on TL-150, PLAN-051 on TL-151) live on spec branches that this
branch does not carry, so their deferred-verification rows are collected
here until those plans merge; the spec owner then moves each row.

## Deferred verification

| Item | Why deferred | Evidence needed to close |
|---|---|---|
| Every test added by this branch (TC-180..184, TC-220..231, TC-250..252, TC-1500..1539) | No cargo, rustc or make may run | `make ci` and `make release-gate-test` at the branch head |
| `make fmt-check` | rustfmt may not run; lines were wrapped by hand | `make fmt` then `make fmt-check` |
| `thumbv7em-none-eabi` build of the numeric, aggregation, contract and LTLf cores (TC-1530, TC-1519, TC-231) | No build | `make check-features` |
| Cross-target byte replay of the numeric corpus (TC-1532) and expansion goldens (TC-1516) | No build | `make msrv`, `make miri` |
| `examples/numeric_corpus.rs` reproduces `corpus/numeric/` byte for byte | No build; the Python original it replaces did | `cargo run --example numeric_corpus --features serde` exits 0 |
| Downstream compile of tl-parse, tl-mltl, tl-rewrite and tl-solve | `NodeKind` gains six variants and `SemanticProfile` a fifth (source-breaking) | `make ci` in each dependent repository |

## Release-time blockers

| Item | Why it blocks | What unblocks it |
|---|---|---|
| tl-solve has no preceding release tag | The golden, API (`cargo-semver-checks`) and legacy-decoder lanes diff each member against `previous_tag`; `check` now refuses an empty one (`a_member_without_a_preceding_tag_is_refused`). The lanes are not weakened. | A specified first-release rule for a new member (FR-362 amendment), or a baseline tag for tl-solve, decided by the release owner |
| Version bump and API migration inventory | New variants on the exhaustive public enums `NodeKind` and `SemanticProfile` are breaking; `Cargo.toml` stays `0.4.0` on this branch | Bump to `0.5.0` in the release-candidate commit and write the `### API migration inventory` from the `cargo-semver-checks` run (FR-031, FR-033) |
| TL-423 AC1, TL-425 AC2 (R2U2 / C2PO numeric and contract export) | Decision 5: refused until a numeric target profile is pinned | A retained R2U2 exchange |

## Guesses about external APIs

None: this branch adds no dependency. `sha2` (already a `serde` dependency)
is used by `examples/numeric_corpus.rs`.
