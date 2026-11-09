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
public API and passes. TC-163 and TC-164 are discharged by the local gate
(`make ci`) together with a Rust wiring test that fails when the gate stops
naming the embedded builds or the Miri run; the wiring test is not evidence
that the builds or Miri passed.

## Test Case Summary

| Test ID | Title | Type | Priority | Traces To | Status |
|---|---|---|---|---|---|
| TC-163 | Build the library for `thumbv7em-none-eabi` with no default features, `alloc`, and `alloc` plus `serde` | Compile | P0 | NFR-006-AC-1 | 🚧 planned |
| TC-164 | Run the unit and property suites under Miri and keep the run in the local gate | Unit | P0 | NFR-006-AC-2 | 🚧 planned |
| TC-165 | Accept each declared limit at the limit and refuse it with the exact typed error one past the limit, on every enforcing path | Property | P0 | NFR-006-AC-3 | 🚧 planned |
| TC-166 | Produce and assert every variant of every public error enum | Unit | P0 | NFR-006-AC-4 | 🚧 planned |

## Matrix Design Coverage

| Required design rule | Coverage |
|---|---|
| Every acceptance criterion mapped | NFR-006-AC-1 through AC-4 map to TC-163 through TC-166. |
| Boundary conditions | TC-165 covers the limit and limit plus one for every declared limit. |
| Error conditions | TC-166 covers every public error variant. |
| Edge cases | TC-164 covers undefined behavior in unsafe-adjacent and pointer-sized arithmetic paths on the host. |
