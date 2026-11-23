---
id: TM-007
title: "Coordinated TL release-gates test matrix"
type: TestMatrix
relationships:
  - target: ix://agent-ix/tl-syntax/FR-018
    type: covers
---

# Coordinated TL release-gates test matrix

These gates apply after the released 0.3.0 baseline. The standalone
`release-gate/` checker (`make release-gate`) implements them; its tests drive
the checking logic with synthetic candidate graphs and mutations. That is
component evidence for the checker, not evidence about a real release: a row is
marked implemented only when a tagged test in `release-gate/tests/` fully
exercises its criterion, and every row that needs the real five-crate candidate
graph, a cargo-semver-checks run, or a human decision stays planned or blocked
until a release candidate is presented to the gate. Ignored test-first stubs are
not implementation coverage.

The release-time lanes (corpus replay against owner corpora at the candidate
revisions, cargo-semver-checks against the preceding tags with a per-finding
CHANGELOG migration inventory, replay of wire records produced by the preceding
tagged crates through the candidate decoders, and the external consumer at the
MSRV and stable) run only through `make release-gate` on exact candidate
commits. They are not part of `make ci`. The gate fails, rather than skips,
when `cargo-semver-checks` is absent. The human decision and the permanent tags
remain outside automation.

Revision note (TL-393): FR-029 through FR-033 and NFR-007 now name five crates (tl-solve joins; see FR-362). `release-gate/src/lib.rs` still counts four until TL-394 lands, so rows whose criteria changed from four to five are marked planned below and are to be re-verified against five; they were implemented and tested against the four-crate set.

## Functional Requirement Coverage

| Functional Req | Acceptance Criteria | Test Cases | Status |
|---|---|---|---|
| FR-029 | FR-029-AC-1 through FR-029-AC-3 | TC-170 through TC-172 | 🚧 planned for the five-crate clause (checker logic was implemented and tested against the four-crate set, with one-axis input mutations; TC-171 is partial, see its row; first run on a real candidate graph pending) |
| FR-030 | FR-030-AC-1 through FR-030-AC-2 | TC-173, TC-174 | 🚧 planned (release-time replay lane; only the empty-population refusal is unit-tested) |
| FR-031 | FR-031-AC-1 through FR-031-AC-2 | TC-175, TC-176 | 🚧 planned (blocked at release time on cargo-semver-checks and the candidate tags; byte comparison with declared exceptions and migration reconciliation are unit-tested) |
| FR-032 | FR-032-AC-1 through FR-032-AC-2 | TC-177, TC-178 | 🚧 planned (blocked on five candidate commits; manifest and lock binding is unit-tested) |
| FR-362 | FR-362-AC-1 through FR-362-AC-3 | TC-250 through TC-252 | 🚧 planned (needs a release-gate build and test run) |
| FR-033 | FR-033-AC-1 through FR-033-AC-2 | TC-179 | 🚧 planned for the five-crate clause (tag-plan refusal and ordering were implemented and tested against four crates; the human decision itself is external) |
| NFR-007 | NFR-007-AC-1 | TC-179 | 🚧 planned (report identity binding is exercised only by a real run) |

## Test Case Summary

| Test ID | Title | Type | Priority | Traces To | Status |
|---|---|---|---|---|---|
| TC-170 | Resolve one production revision and matching exact version per TL edge | Integration | P0 | FR-029-AC-1 | 🚧 planned for five crates (`exact_candidate_graph_and_wrong_pin_refusal` passes at four; `CRATES` has four until TL-394) |
| TC-171 | Build at common MSRV and check tag reachability under one-axis mutations | Integration | P0 | FR-029-AC-2 | 🚧 partial: the declared-MSRV and tag-reachability half is tested by `msrv_and_tag_mutations_fail`; the "successful MSRV builds" half (`msrv_builds` in `release-gate/src/main.rs`) runs only at release time on a real candidate graph |
| TC-172 | Keep declared historical test pins isolated from production | Integration | P0 | FR-029-AC-3 | 🚧 planned for five crates (`historical_test_lane_is_explicit_and_cannot_enter_production` passes at four) |
| TC-173 | Replay all applicable owner corpus cases across five candidate crates | Integration | P0 | FR-030-AC-1 | 🚧 planned (release-time lane) |
| TC-174 | Reject missing, changed, skipped or disagreeing corpus cases | Integration | P0 | FR-030-AC-2 | 🚧 planned (`corpus_target_cannot_pass_with_an_empty_or_ignored_population` covers the empty-population refusal only) |
| TC-175 | Compare public APIs to preceding tags and reconcile breaks to migration notes | Integration | P0 | FR-031-AC-1 | 🚧 planned (blocked at release time on cargo-semver-checks; reconciliation is unit-tested; item-kind line shapes (variant, function, field, method) by `api_finding_symbols_drop_the_item_kind`) |
| TC-176 | Compare legacy wire goldens byte for byte, allowing only declared reasoned exceptions (undeclared and stale ones fail), and reject changed decoders | Integration | P0 | FR-031-AC-2 | 🚧 planned (unit-tested: byte comparison with declared, undeclared, wrong-kind and stale exceptions by `golden_differences_need_matching_declared_exceptions`; the CHANGELOG-with-Migration requirement for `breaking = true` exceptions by `breaking_exceptions_need_a_changelog_migration_entry`. Not automated: detecting an unflagged in-place schema break. Declared decoder breaks: parsing, applicability and the CHANGELOG-with-Migration requirement are unit-tested by `declared_decoder_breaks_are_replayed_and_changelog_enforced`; identity-to-fixture, fixture-to-repo and removed-field validation are unit-tested too; that the candidate replay program implements the refusal and the key-order-preserving stripped decode for the parse, rewrite and conformance fixtures and keeps `replay.json` required-accepted is only asserted on its source by `candidate_replay_enforces_the_declared_refusal`, and the actual refusal runs at release time; the replay proves only those fixtures, other identities are reviewer-checked Decoder replay runs against the preceding tags at release time) |
| TC-177 | Exercise all public downstream consumer operations at exact pins | Integration | P0 | FR-032-AC-1 | 🚧 planned (release-time lane) |
| TC-178 | Build and run one locked consumer at MSRV and stable; reject wrong pins | Integration | P0 | FR-032-AC-2 | 🚧 planned (manifest and lock binding unit-tested; builds are release-time) |
| TC-179 | Refuse tags absent exact human decision and reproducible bound evidence | Integration | P0 | FR-033-AC-1, FR-033-AC-2, NFR-007-AC-1 | 🚧 planned for five crates (`exact_human_decision_only_yields_dependency_ordered_tag_plan` passes at four); NFR-007-AC-1 planned |
| TC-250 | Pass the five-crate candidate set and emit the tag plan in dependency order | Integration | P0 | FR-362-AC-1 | 🚧 planned, deferred verification (needs a release-gate test run) |
| TC-251 | Fail when any one of the five crates is missing or a sixth crate is present | Integration | P0 | FR-362-AC-2 | 🚧 planned, deferred verification (needs a release-gate test run) |
| TC-252 | Refuse forbidden edges, cycles and split tl-syntax or tl-mltl revisions across the tl-solve and tl-rewrite edges | Integration | P0 | FR-362-AC-3 | 🚧 planned, deferred verification (needs a release-gate test run) |

## Integration Test Matrix

| Purpose | Target | Type | Test Cases |
|---|---|---|---|
| Verify exact release graph and MSRV | all five TL crates | workspace | TC-170 through TC-172 |
| Replay syntax-owned corpora | parse, rewrite, mltl | workspace | TC-173, TC-174 |
| Compare preceding releases | all five TL crates | workspace | TC-175, TC-176 |
| Admit the fifth crate and its edges | tl-solve with the other four TL crates | workspace | TC-250 through TC-252 |
| Consume proposed tags and decide release | scratch consumer generated by `release-gate/` and human release owner | service | TC-177 through TC-179 |
