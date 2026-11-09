---
id: TM-006
title: "Robustness test matrix"
type: TestMatrix
relationships:
  - target: ix://agent-ix/tl-syntax/MRS-001
    type: covers
---

# Robustness test matrix

## Functional Requirement Coverage

| Functional Req | Acceptance Criteria | Test Cases | Status |
|---|---|---|---|
| FR-014 | FR-014-AC-2 (limits, through NFR-006-AC-3) | TC-165 | 🚧 planned |

## Non-Functional Requirement Coverage

| Non-Functional Req | Verification Method | Evidence/Test Cases | Status |
|---|---|---|---|
| NFR-006 | NFR-006-AC-1, NFR-006-AC-2, NFR-006-AC-3, NFR-006-AC-4 | TC-163, TC-164, TC-165, TC-166 | 🚧 planned |

A row becomes implemented only when its assertion executes against the real
public API and passes. TC-163 can fail: the existing `tests/feature_boundary.rs`
test runs the `check-features` target, which builds the embedded
configurations. TC-164 is an Analysis of the gate completion record: the `miri`
target runs inside `make ci` and the analysis reads the gate's exit status and
the target's presence in the composite; it is not a Rust test.

## Test Case Summary

| Test ID | Title | Type | Priority | Traces To | Status |
|---|---|---|---|---|---|
| TC-163 | Build the library for `thumbv7em-none-eabi` with no default features, `alloc`, and `alloc,serde` | Compile | P0 | NFR-006-AC-1 | 🚧 planned |
| TC-164 | Analyse the gate completion record for a clean Miri run of the unit and property suites | Analysis | P0 | NFR-006-AC-2 | 🚧 planned |
| TC-165 | For every row of the NFR-006 limit contract table, a value equal to the limit is accepted; a value one past the limit is refused with the typed error that names that limit | Property | P0 | NFR-006-AC-3 | 🚧 planned |
| TC-166 | Produce and assert every variant of every public error type in the NFR-006 error type table, or remove the variant | Unit | P0 | NFR-006-AC-4 | 🚧 planned |

## Matrix Design Coverage

| Required design rule | Coverage |
|---|---|
| Every acceptance criterion mapped | NFR-006-AC-1 through AC-4 map to TC-163 through TC-166. |
| Boundary conditions | TC-165 covers the 24 limit rows, accept at the limit and refuse one past it. |
| Error conditions | TC-166 covers every public error variant. |
