---
id: FR-362
title: Admit tl-solve as the fifth crate of the coordinated release graph
type: FR
relationships:
  - target: ix://agent-ix/tl-syntax/FR-029
    type: depends_on
  - target: ix://agent-ix/tl-syntax/FR-033
    type: depends_on
  - target: ix://agent-ix/tl-solve/ADR-001
    type: depends_on
  - target: ix://agent-ix/tl-solve/ADR-004
    type: depends_on
---

# FR-362: Admit tl-solve as the fifth crate of the coordinated release graph

## Description

When a coordinated TL release is proposed, the release gate shall treat exactly
five crates as the candidate set: tl-syntax, tl-parse, tl-mltl, tl-solve and
tl-rewrite, and shall fail when any one of them is missing or when any other
crate is presented as a member.

## Inputs

- The candidate commits and proposed tags of the five crates (FR-029).
- The dependency edges read from each candidate's manifests, including the
  optional edge from tl-rewrite to tl-solve and the dev edges.

## Behavior

- `release-gate/src/lib.rs` names the five crates in one constant (`CRATES`);
  every check that counted four (the candidate-set size, the consumer's
  candidate count, the decision's revision count, the corpus and API entries
  that name a repository) counts five and reads the length from the constant,
  not from a second literal.
- The release-gate checks the edges against the allowed set recorded in
  tl-solve ADR-004: tl-solve depends on tl-syntax and tl-mltl only; tl-rewrite
  may depend on tl-solve through its `solver` feature; no edge exists from
  tl-solve to tl-rewrite, from tl-mltl or tl-syntax to tl-solve, and the graph
  has no cycle.
- Every production dependency edge between the five crates resolves to the one
  selected revision of the target (FR-029), including the tl-solve edges, so
  that tl-solve and tl-rewrite resolve one tl-syntax and one tl-mltl.
- The dependency-ordered tag plan (FR-033) lists tl-syntax, tl-parse, tl-mltl,
  tl-solve, tl-rewrite, and a plan that places tl-solve after tl-rewrite is
  refused.
- tl-oracle is unpublished and dev-only and is not a member; a dev edge to it
  seen by the graph check is not a sixth crate.
- Scope: this requirement and the four-to-five edits of FR-029 through FR-033
  change counts only. Replaying owner corpora through tl-solve and a
  satisfiability step in the consumer smoke are not added here; they would be
  new behavior of FR-030 and FR-032 and need their own requirement and ticket
  (tl-solve FR-019 is an oracle cross-check on tl-solve's own corpus, not a
  release-gate replay).
- This requirement adds no human-decision step; FR-033 still requires one
  attributed decision for the exact five-crate candidate.

## Acceptance Criteria

| ID | Criteria | Verification |
|---|---|---|
| FR-362-AC-1 | A synthetic candidate set of the five crates with consistent pins passes the gate, and the produced tag plan is in the stated dependency order. | integration-testing (TC-250) |
| FR-362-AC-2 | For each of the five crates in turn, a candidate set without that crate fails with an error naming it, and a set with a sixth unknown crate fails. | integration-testing (TC-251) |
| FR-362-AC-3 | A candidate graph with a tl-solve to tl-rewrite edge, a tl-mltl to tl-solve edge, a cycle, or two revisions of tl-syntax or tl-mltl across the tl-solve and tl-rewrite edges fails, and the consistent graph passes. | integration-testing (TC-252) |

## Dependencies

Depends on FR-029 for pins and MSRV and FR-033 for the tag plan. tl-solve
FR-017 is the repository-side requirement that the same gate must pass.
