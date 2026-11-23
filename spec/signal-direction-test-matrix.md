---
id: TM-008
title: Signal direction test matrix
type: TestMatrix
relationships:
  - target: ix://agent-ix/tl-syntax/FR-034
    type: covers
---

# Signal direction test matrix

Rows for FR-034, the input/output direction on the signal catalog (TL-414 in
project P-TL-149). No row is implemented at the time of writing; every status
is planned. Coverage of requirement-to-test is read from `quire matrix`, not
authored here.

## Test Case Summary

| Test ID | Title | Type | Priority | Traces To | Status |
|---|---|---|---|---|---|
| TC-180 | Every existing v1 catalog fixture validates, round-trips byte-identically with unchanged identity, and reads every signal as undeclared | Snapshot | P0 | FR-034-AC-1 | 🚧 planned, not yet written |
| TC-181 | v2 documents round-trip with mixed and absent directions; invalid, duplicate, literal-undeclared and v1-with-direction forms are refused; the v1 constructor refuses a declared direction | Property | P0 | FR-034-AC-2 | 🚧 planned, not yet written |
| TC-182 | Partition of generated catalogs and formulas is disjoint, increasing, complete and direction-faithful; partition_all equals the union partition | Property | P0 | FR-034-AC-3 | 🚧 planned, not yet written |
| TC-183 | Partition refuses the first undeclared referenced signal in node order with no partial output, ignores unreferenced undeclared signals, and preserves the FR-007 binding errors | Property | P0 | FR-034-AC-4 | 🚧 planned, not yet written |
| TC-184 | v2 schema bytes and digest are embedded, v1 schema bytes and digest are unchanged, and schema acceptance matches the semantic reader | Snapshot | P1 | FR-034-AC-5 | 🚧 planned, not yet written |
