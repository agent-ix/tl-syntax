---
id: PLAN-005
title: Source-census reproducibility plan
type: Plan
status: in_progress
relationships:
  - target: ix://agent-ix/tl-syntax/FR-006
    type: references
---

# PLAN-005: Source-census reproducibility plan

## Objective

Close `agent-ix/tl-syntax#20` by making the landed source census reproducible
across workstations, byte-safe for arbitrary tracked inputs, and quiet when its
expected negative controls run. Preserve the exact tracked path set as the
final authority and keep the shared Engineering Assurance, Quire, and Quoin
boundary unchanged.

## Base and scope

The branch starts from landed PR #18 revision
`ba292615479024e271595457798789634b686a15`. It changes FR-006, the test matrix,
one existing Rust integration test, this plan bundle, and review records. It
adds no production dependency, command runner, Make target, hosted workflow,
evidence envelope, or retention path.

## Dependency order

```text
reviewed FR-006-AC-7/AC-8 and TC-034/TC-035
  -> fallible Git enumeration and byte-scan helpers
    -> repository-authored ignore semantics
      -> binary and machine-ignore falsifying controls
        -> reachable area diagnostic followed by exact path equality
          -> focused and full local gates
            -> code review + gap analysis
              -> independent exact-head review
```

## Verification

TC-034 retains the tracked, ordinary-untracked, generated-ignore, archival, and
non-repository cases. It checks the area population first, the exact path set
second, and explicitly refuses any ordinary-untracked delta after scanning it.
TC-035 extends the scratch repository with a tracked non-UTF-8 input and two
ordinary-untracked paths named by mutable Git excludes. Benign arbitrary bytes
must scan successfully; embedding a forbidden ASCII identity must return a
path-specific refusal. Both locally excluded paths must remain visible.

The exact candidate runs focused TC-034/TC-035, strict Quire validation and
coverage, then full local `make ci CARGO_TARGET_DIR=target/cargo-review`.
Hosted CI is not dispatched.

## Exit criteria

1. Workstation and `.git/info` excludes cannot narrow the ordinary-untracked
   source set; repository `.gitignore` rules still exclude generated paths.
2. Arbitrary bytes are scanned without UTF-8 decoding, and forbidden ASCII
   identities remain detectable with the source path named.
3. Expected refusal controls use explicit errors and produce no panic-hook
   noise.
4. Area and exact-path diagnostics each have an observable failure role, with
   exact equality retaining final authority.
5. Closing reviews have no unresolved high or medium finding and the exact-head
   full local gate passes.
