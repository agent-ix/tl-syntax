---
id: FR-014
title: Bind source-readiness facts to one candidate and configuration
type: FR
relationships:
  - target: ix://agent-ix/tl-syntax/StR-004
    type: implements
  - target: ix://agent-ix/tl-syntax/NFR-003
    type: depends_on
  - target: ix://agent-ix/tl-syntax/FR-018
    type: depends_on
---

# FR-014: Bind source-readiness facts to one candidate and configuration

## Description

When source-readiness facts are prepared, tl-syntax shall bind them to one exact
candidate revision, declared complete live-source population and build/evaluation
configuration through released shared-assurance contracts.

## Inputs

- Canonical repository identity, exact Git commit/tree identity and clean
  declared materialized source population selected for review, including
  tracked regular-file bytes, symlink targets and any submodule, Git LFS,
  generated or build-time fetched input disposition.
- The ordered canonical admitted-root set and each root's immutable filesystem/
  mount identity; AP-002 admits only the isolated candidate root.
- Crate name/version plus `Cargo.toml`, `Cargo.lock`, Rust toolchain and source
  artifact digests.
- Selected features, target triples, build flags, corpus/profile revisions and
  domain-suite identities.
- Exact Engineering Assurance, Quire, Quoin and workflow package identities,
  versions and consumed artifact digests.
- For every executed producer: executable bytes, invocation identity/run nonce,
  arguments/environment, start and completion facts, process outcome and fresh
  output identity.

## Outputs

- Source-grounded shared-contract inputs for one exact candidate/configuration,
  or the exact MRS-004 `ReadinessState` produced by a non-success condition,
  including `unavailable` for an absent/incompatible required shared capability,
  `incomplete` for missing package/artifact integrity and `refused` for a
  missing, stale or ambiguous candidate identity.

## Behavior

- tl-syntax shall treat a changed candidate revision, tracked source path or
  byte, feature set, target, toolchain, build flag, corpus/profile revision,
  shared-contract release or consumed artifact digest as a different readiness
  subject.
- tl-syntax shall refuse a dirty or incompletely enumerated live source
  population instead of describing it as the reviewed candidate.
- tl-syntax shall execute against immutable isolated candidate/configuration
  bytes or revalidate every bound identity before and after each producer and
  shared handoff.
- If any bound identity changes during execution, then the tl-syntax Rust
  readiness projection shall produce `refused`.
- tl-syntax shall quarantine or remove prior producer outputs before invocation.
- The tl-syntax Rust readiness projection shall accept only output freshly
  created by the successful bound invocation.
- The tl-syntax Rust readiness projection shall record the exact
  `ExecutionOutcome` without reusing a prior output.
- tl-syntax shall retain unsupported and unevaluated configurations as explicit
  states instead of copying a result from a nearby configuration.
- While compatible released capabilities are available, tl-syntax shall use
  the source-grounded Quire export and Quoin source connections.
- A packaged compatibility matrix shall be bound through the immutable released
  package/version and package-integrity mechanism that supplies it.
- Every artifact consumed outside the matrix's package shall retain its own
  digest.
- If package integrity or an external-artifact digest is missing, then the
  tl-syntax Rust readiness projection shall classify the shared artifact set as
  `incomplete`.
- tl-syntax shall preserve symlink link bytes.
- When the build consumes a symlink referent, tl-syntax shall bind its resolved
  bytes and identity.
- tl-syntax shall constrain every resolved symlink referent to a profile-
  admitted root.
- tl-syntax shall constrain every resolved symlink referent to an ordinary file
  type.
- The tl-syntax source walk shall detect resolution cycles.
- The tl-syntax source walk shall enforce the AP-002 symlink-chain bound.
- The tl-syntax source walk shall normalize and resolve every path segment
  beneath the bound admitted root without crossing a descendant mount.
- The tl-syntax source walk shall bind the admitted-root set, canonical paths
  and filesystem/mount identities before and after resolution.
- If root/path/mount identity changes during resolution, then the tl-syntax Rust
  readiness projection shall produce `refused`.
- If a symlink is dangling, cyclic, out-of-root, special-file targeting or over
  the declared chain bound, then the tl-syntax Rust readiness projection shall
  produce `refused`.
- If a required released source-grounding capability is unavailable, then
  tl-syntax shall report the readiness request unavailable.
- tl-syntax shall not parse Markdown, copy a branch-head contract or introduce
  a local source-identity schema as a substitute.

## Acceptance Criteria

| ID | Criteria | Verification |
|---|---|---|
| FR-014-AC-1 | Every healthy readiness input names one exact candidate revision, complete tracked-source set, crate/package bytes, feature/target/build configuration, corpus/profile revision, suite set and compatible shared-contract artifact set. | Test (TC-059) |
| FR-014-AC-2 | Mutating any identity or digest changes the readiness subject; omitting a required candidate identity is refused, while absence/incompatibility of a required shared capability is unavailable, and no prior result is reused. | Test (TC-060) |
| FR-014-AC-3 | Dirty, untracked-live, mutable-ignore, incomplete-census and unavailable-enumeration cases cannot be represented as a clean reviewed candidate. | Test (TC-060) |
| FR-014-AC-4 | Absence of a compatible released source-grounding contract remains unavailable and creates no local parser, schema, matrix or branch-head substitute. | Test (TC-066) |
| FR-014-AC-5 | Every `ExecutionOutcome` value in MRS-004 is exercised with a valid stale output present; no non-`succeeded` value yields a current successful domain output, every accepted output binds fresh invocation/executable/termination facts, and a mid-run source/configuration mutation is refused. | Test (TC-068) |
| FR-014-AC-6 | Candidate identity includes repository/Git/materialized-source/admitted-root identities; material or root/path/mount substitution/race is detected or unavailable, symlink chains below/at AP-002's bound terminate, and dangling/cyclic/out-of-root/special/mount-crossing/over-bound paths are refused. | Test (TC-069) |

## Dependencies

- [FR-006](./FR-006-shared-assurance-intake.md) owns current shared intake.
- [NFR-003](./NFR-003-qualification-integrity.md) owns existing v0.1
  qualification meaning and the pre-stable limitations.
- `agent-ix/tl-syntax#16` owns adoption of a released source-grounded Quire and
  Engineering Assurance compatibility set; this requirement does not bypass
  its resume condition.
