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
making TC-034 enumerate every non-archival tracked path through Git, report
non-ignored untracked paths separately, and bind the exact live path set plus
per-area populations. Keep TC-026 focused on absence of deleted local machinery.
Correct the retained statements identified by the same review without changing
the shared assurance architecture.

## Base and scope

The branch starts from merged PR #14 revision
`a6d58aa4df3ade8964b3e1223666983aa5e89910`. It changes one existing Rust
integration test, its owning requirement and matrix row, the prior measurement
clarification, this plan bundle, and closing reviews. It adds no production
dependency, command runner, Make target, hosted workflow, evidence envelope, or
retention path.

## Dependency order

```text
reviewed FR-006/NFR-002/TC-026 and TC-034 split
  -> Git tracked/untracked helper with unable-to-enumerate refusal
    -> ignored/generated and non-ignored/untracked controls
      -> required-root and per-area population controls
        -> retained statement corrections
          -> local full gate
            -> code review + gap analysis
              -> reviewed merge before tl-syntax#15
```

## Verification

TC-034 exercises a scratch Git repository containing tracked source and ignore
configuration paths, one
non-ignored untracked source, and one ignored generated source. The tracked item
must enter the population and scan; the untracked item must enter only the scan;
the ignored item must enter neither. The untracked item also carries a forbidden
name so the scan consumer, not only its producer, is falsified. The live
repository then checks all 66 non-archival tracked paths exactly, checks ten
per-area cardinalities, and reports any ordinary untracked path. TC-026 scans
that complete live set and rejects a higher-precedence Make input.

The exact candidate runs focused TC-026 and TC-034, strict Quire validation and coverage,
then full local `make ci CARGO_TARGET_DIR=target/cargo-review`. Hosted CI is not
dispatched.

## Exit criteria

1. Ignored generated files cannot change the reviewed population or scan.
2. A Git enumeration failure is a named failure, never an empty successful set.
3. Missing, new, renamed, extensionless, and within- or cross-area path changes
   fail with the exact path-set or area delta.
4. NFR-002 and SR-013 describe the live criterion and scenario-label boundaries.
5. Closing reviews have no unresolved high or medium finding and the exact-head
   full local gate passes.
