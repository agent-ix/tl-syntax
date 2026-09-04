---
id: PLAN-004
title: Tracked source census hardening plan
type: Plan
status: in_progress
relationships:
  - target: ix://agent-ix/tl-syntax/FR-006
    type: references
---

# PLAN-004: Tracked source census hardening plan

## Objective

Close `agent-ix/tl-syntax#17` before the next census-changing feature lands by
making TC-026 enumerate the reviewed tracked source set through Git, scan
non-ignored untracked paths separately, require declared roots, and bind exact
total and per-area populations. Correct the two retained statements identified
by the same review without changing the shared assurance architecture.

## Base and scope

The branch starts from merged PR #14 revision
`a6d58aa4df3ade8964b3e1223666983aa5e89910`. It changes one existing Rust
integration test, its owning requirement and matrix row, the prior measurement
clarification, this plan bundle, and closing reviews. It adds no production
dependency, command runner, Make target, hosted workflow, evidence envelope, or
retention path.

## Dependency order

```text
reviewed FR-006/NFR-002/TC-026 extension
  -> Git tracked/untracked helper with unable-to-enumerate refusal
    -> ignored/generated and non-ignored/untracked controls
      -> required-root and per-area population controls
        -> retained statement corrections
          -> local full gate
            -> code review + gap analysis
              -> reviewed merge before tl-syntax#15
```

## Verification

TC-026 exercises a scratch Git repository containing one tracked source, one
non-ignored untracked source, and one ignored generated source. The tracked item
must enter the population and scan; the untracked item must enter only the scan;
the ignored item must enter neither. The live repository then checks its
required roots, exact tracked total, exact per-area cardinalities, readable
selected paths, and forbidden-name absence.

The exact candidate runs focused TC-026, strict Quire validation and coverage,
then full local `make ci CARGO_TARGET_DIR=target/cargo-review`. Hosted CI is not
dispatched.

## Exit criteria

1. Ignored generated files cannot change the reviewed population or scan.
2. A Git enumeration failure is a named failure, never an empty successful set.
3. Missing required roots and compensating cross-area changes fail distinctly.
4. NFR-002 and SR-013 describe the live criterion and scenario-label boundaries.
5. Closing reviews have no unresolved high or medium finding and the exact-head
   full local gate passes.
