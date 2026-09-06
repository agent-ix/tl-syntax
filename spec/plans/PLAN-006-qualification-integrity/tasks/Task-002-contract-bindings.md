---
id: Task-002
title: Bind qualification declarations and tests
type: Task
status: done
relationships:
  - target: ix://agent-ix/tl-syntax/PLAN-006
    type: part_of
---

# Task-002: Bind qualification declarations and tests

Add NFR-003 to the shared change declaration, implement TC-036 over the local
suite/proof boundary, extend existing test traces, and update the exact source
census without introducing new assurance tooling.

## Completion evidence

The declaration binds all five NFR-003 criteria and a structured local-suite
boundary, retains separate open/accepted unknowns for the stable qualified
record and measured Make limitation, and makes no SUITE-008 proof claim.
TC-036 checks those boundaries. Existing TC-021, TC-022, TC-025, TC-026,
TC-034, and TC-035 carry the corresponding NFR-003 traces, and the exact census
contains the new requirement path.
