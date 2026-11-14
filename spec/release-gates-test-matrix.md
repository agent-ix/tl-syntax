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
exercises its criterion, and every row that needs the real four-crate candidate
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

## Functional Requirement Coverage

| Functional Req | Acceptance Criteria | Test Cases | Status |
|---|---|---|---|
| FR-029 | FR-029-AC-1 through FR-029-AC-3 | TC-170 through TC-172 | ✅ checker logic implemented and mutation-tested; first run on a real candidate graph pending |
| FR-030 | FR-030-AC-1 through FR-030-AC-2 | TC-173, TC-174 | 🚧 planned (release-time replay lane; only the empty-population refusal is unit-tested) |
| FR-031 | FR-031-AC-1 through FR-031-AC-2 | TC-175, TC-176 | 🚧 planned (blocked at release time on cargo-semver-checks and the candidate tags; byte comparison with declared exceptions and migration reconciliation are unit-tested) |
| FR-032 | FR-032-AC-1 through FR-032-AC-2 | TC-177, TC-178 | 🚧 planned (blocked on four candidate commits; manifest and lock binding is unit-tested) |
| FR-033 | FR-033-AC-1 through FR-033-AC-2 | TC-179 | ✅ tag-plan refusal and ordering implemented and tested; the human decision itself is external |
| NFR-007 | NFR-007-AC-1 | TC-179 | 🚧 planned (report identity binding is exercised only by a real run) |

## Test Case Summary

| Test ID | Title | Type | Priority | Traces To | Status |
|---|---|---|---|---|---|
| TC-170 | Resolve one production revision and matching exact version per TL edge | Integration | P0 | FR-029-AC-1 | ✅ `exact_candidate_graph_and_wrong_pin_refusal` |
| TC-171 | Build at common MSRV and check tag reachability under one-axis mutations | Integration | P0 | FR-029-AC-2 | ✅ `msrv_and_tag_mutations_fail` |
| TC-172 | Keep declared historical test pins isolated from production | Integration | P0 | FR-029-AC-3 | ✅ `historical_test_lane_is_explicit_and_cannot_enter_production` |
| TC-173 | Replay all applicable owner corpus cases across four candidate crates | Integration | P0 | FR-030-AC-1 | 🚧 planned (release-time lane) |
| TC-174 | Reject missing, changed, skipped or disagreeing corpus cases | Integration | P0 | FR-030-AC-2 | 🚧 planned (`corpus_target_cannot_pass_with_an_empty_or_ignored_population` covers the empty-population refusal only) |
| TC-175 | Compare public APIs to preceding tags and reconcile breaks to migration notes | Integration | P0 | FR-031-AC-1 | 🚧 planned (blocked at release time on cargo-semver-checks; reconciliation is unit-tested) |
| TC-176 | Compare legacy wire goldens byte for byte, allowing only declared reasoned exceptions (undeclared and stale ones fail), and reject changed decoders | Integration | P0 | FR-031-AC-2 | 🚧 planned (byte comparison and the declared-exception rule (`golden_differences_need_matching_declared_exceptions`) unit-tested; decoder replay runs against the preceding tags at release time) |
| TC-177 | Exercise all public downstream consumer operations at exact pins | Integration | P0 | FR-032-AC-1 | 🚧 planned (release-time lane) |
| TC-178 | Build and run one locked consumer at MSRV and stable; reject wrong pins | Integration | P0 | FR-032-AC-2 | 🚧 planned (manifest and lock binding unit-tested; builds are release-time) |
| TC-179 | Refuse tags absent exact human decision and reproducible bound evidence | Integration | P0 | FR-033-AC-1, FR-033-AC-2, NFR-007-AC-1 | ✅ for FR-033-AC-1 and FR-033-AC-2 (`exact_human_decision_only_yields_dependency_ordered_tag_plan`); NFR-007-AC-1 planned |

## Integration Test Matrix

| Purpose | Target | Type | Test Cases |
|---|---|---|---|
| Verify exact release graph and MSRV | all four TL crates | workspace | TC-170 through TC-172 |
| Replay syntax-owned corpora | parse, rewrite, mltl | workspace | TC-173, TC-174 |
| Compare preceding releases | all four TL crates | workspace | TC-175, TC-176 |
| Consume proposed tags and decide release | scratch consumer generated by `release-gate/` and human release owner | service | TC-177 through TC-179 |
