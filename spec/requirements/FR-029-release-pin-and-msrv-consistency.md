---
id: FR-029
title: Check coordinated release pins and MSRV
type: FR
relationships:
  - target: ix://agent-ix/tl-syntax/FR-018
    type: depends_on
---

# FR-029: Check coordinated release pins and MSRV

## Description

When tagging is proposed for a coordinated TL release after 0.3.0, a Rust
checker shall verify the exact release candidate graph for tl-syntax,
tl-parse, tl-mltl and tl-rewrite.

## Inputs

- Immutable candidate commits and proposed tags for the four crates.
- Each crate's Cargo manifests, resolved dependency graph, toolchain and
  declared minimum supported Rust version (MSRV).
- Explicitly declared historical compatibility lanes, if any.

## Outputs

- A deterministic per-edge pin and MSRV report bound to the four candidate
  commits, or a refusal naming every inconsistent edge.

## Behavior

Every production dependency on a TL crate resolves to the one selected
candidate revision for that crate. Each `version = "=..."` agrees with the
package version at its pinned revision; no branch, floating tag, or unstated
path override is a release pin. All four crates declare the same MSRV, and the
candidate builds at that MSRV. Each dependency revision is reachable from the
corresponding proposed release tag's commit. The checker does not infer
reachability from a version string alone.

A historical-revision test lane is allowed only when its aliases and revisions
are enumerated in the release record, isolated from the production resolution,
and separately tested. It cannot satisfy or weaken the single-revision
production rule. The 0.3.0 release is the fixed previous-release baseline;
its documented light-gate exception does not apply to a later tag.

## Acceptance Criteria

| ID | Criteria | Verification |
|---|---|---|
| FR-029-AC-1 | At the candidate commits, each production TL dependency resolves to exactly one selected revision and its exact version matches that revision's package version. | Test (TC-170) |
| FR-029-AC-2 | Four identical declared MSRVs, successful MSRV builds and reachability from the proposed tags are checked from the candidate graph; a one-axis mutation fails. | Test (TC-171) |
| FR-029-AC-3 | A declared historical test lane stays isolated and cannot mask a duplicate production revision; undeclared or leaking historical pins fail. | Test (TC-172) |

## Dependencies

FR-018 owns the human source-release decision; this gate supplies facts to it.
