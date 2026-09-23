---
id: TM-006
title: "Coordinated TL release-gates test matrix"
type: TestMatrix
relationships:
  - target: ix://agent-ix/tl-syntax/FR-018
    type: covers
---

# Coordinated TL release-gates test matrix

These gates apply after the released 0.3.0 baseline. TC-170 through TC-179
are planned until their assertions execute on a candidate using the real
four-crate graph. Ignored test-first stubs are not implementation coverage.

## Functional Requirement Coverage

| Functional Req | Acceptance Criteria | Test Cases | Coverage Status |
|---|---|---|---|
| FR-029 | FR-029-AC-1 through FR-029-AC-3 | TC-170 through TC-172 | 🚧 planned |
| FR-030 | FR-030-AC-1 through FR-030-AC-2 | TC-173, TC-174 | 🚧 planned |
| FR-031 | FR-031-AC-1 through FR-031-AC-2 | TC-175, TC-176 | 🚧 planned |
| FR-032 | FR-032-AC-1 through FR-032-AC-2 | TC-177, TC-178 | 🚧 planned |
| FR-033 | FR-033-AC-1 through FR-033-AC-2 | TC-179 | 🚧 planned |
| NFR-007 | NFR-007-AC-1 | TC-179 | 🚧 planned |

## Test Case Summary

| Test ID | Title | Type | Priority | Traces To | Status |
|---|---|---|---|---|---|
| TC-170 | Resolve one production revision and matching exact version per TL edge | Integration | P0 | FR-029-AC-1 | 🚧 planned |
| TC-171 | Build at common MSRV and check tag reachability under one-axis mutations | Integration | P0 | FR-029-AC-2 | 🚧 planned |
| TC-172 | Keep declared historical test pins isolated from production | Integration | P0 | FR-029-AC-3 | 🚧 planned |
| TC-173 | Replay all applicable owner corpus cases across four candidate crates | Integration | P0 | FR-030-AC-1 | 🚧 planned |
| TC-174 | Reject missing, changed, skipped or disagreeing corpus cases | Integration | P0 | FR-030-AC-2 | 🚧 planned |
| TC-175 | Compare public APIs to preceding tags and reconcile breaks to migration notes | Integration | P0 | FR-031-AC-1 | 🚧 planned |
| TC-176 | Compare legacy wire goldens byte for byte and reject changed decoders | Integration | P0 | FR-031-AC-2 | 🚧 planned |
| TC-177 | Exercise all public downstream consumer operations at exact pins | Integration | P0 | FR-032-AC-1 | 🚧 planned |
| TC-178 | Build and run one locked consumer at MSRV and stable; reject wrong pins | Integration | P0 | FR-032-AC-2 | 🚧 planned |
| TC-179 | Refuse tags absent exact human decision and reproducible bound evidence | Integration | P0 | FR-033-AC-1, FR-033-AC-2, NFR-007-AC-1 | 🚧 planned |

## Integration Test Matrix

| Purpose | Target | Type | Test Cases |
|---|---|---|---|
| Verify exact release graph and MSRV | all four TL crates | workspace | TC-170 through TC-172 |
| Replay syntax-owned corpora | parse, rewrite, mltl | workspace | TC-173, TC-174 |
| Compare preceding releases | all four TL crates | workspace | TC-175, TC-176 |
| Consume proposed tags and decide release | non-workspace `release-smoke/` and human release owner | service | TC-177 through TC-179 |
