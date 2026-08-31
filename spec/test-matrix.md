---
id: TM-001
title: tl-syntax v0.1 test matrix
type: TestMatrix
relationships:
  - target: ix://agent-ix/tl-syntax/MRS-001
    type: covers
---

# tl-syntax v0.1 Test Matrix

## Functional Requirement Coverage

| Functional Req | Acceptance Criteria | Test Cases | Coverage Status |
|---|---|---|---|
| FR-001 | FR-001-AC-1, FR-001-AC-2 | TC-001, TC-002 | ✅ covered |
| FR-002 | FR-002-AC-1, FR-002-AC-2, FR-002-AC-3 | TC-003, TC-004, TC-005 | ✅ covered |
| FR-003 | FR-003-AC-1, FR-003-AC-2, FR-003-AC-3 | TC-006, TC-007, TC-008 | ✅ covered |
| FR-004 | FR-004-AC-1, FR-004-AC-2, FR-004-AC-3, FR-004-AC-4 | TC-009, TC-010, TC-011, TC-017, TC-020 | ✅ covered |
| FR-005 | FR-005-AC-1, FR-005-AC-2, FR-005-AC-3 | TC-012, TC-013, TC-014 | ✅ covered |

## Stakeholder Requirement Coverage

| Stakeholder Req | Trace to US/FR | Test/Validation | Coverage Status |
|---|---|---|---|
| StR-001 | NFR-001 | TC-015 | ✅ covered |
| StR-002 | FR-003, FR-004, FR-005 | TC-007, TC-009, TC-012 | ✅ covered |

## Non-Functional Requirement Coverage

| Non-Functional Req | Verification Method | Evidence/Test Cases | Status |
|---|---|---|---|
| NFR-001 | Compile and inspection | TC-015, TC-019 | ✅ covered |
| NFR-002 | Tests and inspection | TC-005, TC-014, TC-016, TC-018 | ✅ covered |

## Test Case Summary

| Test ID | Title | Type | Priority | Traces To | Status |
|---|---|---|---|---|---|
| TC-001 | Accept valid inclusive intervals | Unit | P0 | FR-001-AC-1 | ✅ implemented |
| TC-002 | Reject inverted intervals | Unit | P0 | FR-001-AC-2 | ✅ implemented |
| TC-003 | Validate complete node vocabulary | Unit | P0 | FR-002-AC-1 | ✅ implemented |
| TC-004 | Reject invalid graph references | Unit | P0 | FR-002-AC-2 | ✅ implemented |
| TC-005 | Preserve deterministic ordering | Unit | P1 | FR-002-AC-3, NFR-002-AC-1 | ✅ implemented |
| TC-006 | Validate and order identities and spans | Unit | P1 | FR-003-AC-1 | ✅ implemented |
| TC-007 | Preserve stable profile wire names | Unit | P0 | FR-003-AC-2 | ✅ implemented |
| TC-008 | Require formula profile field | Integration | P0 | FR-003-AC-3 | ✅ implemented |
| TC-009 | Round-trip owned documents | Integration | P0 | FR-004-AC-1 | ✅ implemented |
| TC-010 | Reject unknown wire versions | Integration | P0 | FR-004-AC-2 | ✅ implemented |
| TC-011 | Reject malformed owned documents | Integration | P0 | FR-004-AC-3 | ✅ implemented |
| TC-012 | Cover required corpus classes | Integration | P0 | FR-005-AC-1 | ✅ implemented |
| TC-013 | Enforce declared fixture validity | Integration | P0 | FR-005-AC-2 | ✅ implemented |
| TC-014 | Verify checked-in corpus determinism | Integration | P1 | FR-005-AC-3, NFR-002-AC-2 | ✅ implemented |
| TC-015 | Compile the allocation-free API | Compile | P0 | NFR-001-AC-1, NFR-001-AC-2 | ✅ implemented |
| TC-016 | Reject false-success evidence classifications | Integration | P0 | NFR-002-AC-3 | ✅ implemented |
| TC-017 | Round-trip every supported node wire variant and tag | Integration | P0 | FR-004-AC-1 | ✅ implemented |
| TC-018 | Reject swallowed command failures and incomplete traceability reports | Integration | P0 | NFR-002-AC-4 | ✅ implemented |
| TC-019 | Execute the no-std feature matrix and empty-default-dependency gate | Integration | P0 | NFR-001-AC-1, NFR-001-AC-2 | ✅ implemented |
| TC-020 | Bound formula node allocation during wire decoding and owned construction | Integration | P0 | FR-004-AC-1, FR-004-AC-4 | ✅ implemented |
