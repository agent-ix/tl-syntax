---
id: FR-018
title: Classify every readiness execution path
type: FR
relationships:
  - target: ix://agent-ix/tl-syntax/StR-004
    type: implements
  - target: ix://agent-ix/tl-syntax/FR-014
    type: references
  - target: ix://agent-ix/tl-syntax/NFR-003
    type: depends_on
  - target: ix://agent-ix/tl-syntax/FR-006
    type: depends_on
---

# FR-018: Classify every readiness execution path

## Description

The maintainer shall complete classification of every executable production,
test, fixture-audit,
adapter, orchestration and qualification path as retained shared capability,
domain-specific Rust logic, missing reusable capability or explicitly
owner-dispositioned legacy logic before implementing source readiness or
deciding a stable candidate.

## Inputs

- Exact candidate tracked-source census and executable-path inventory.
- Implementation-language owner policy.
- Released shared-capability inventory and open upstream tickets.
- Existing behavior, failure-path evidence and proposed replacement ownership.

## Outputs

- A reviewed disposition for every executable path, including language,
  purpose, authority, owner, replacement/retention decision, parity obligation,
  failure-path evidence and resume condition.

## Behavior

- New first-party libraries, CLI/tools, generators, validators,
  canonicalization, adapters, test assertions and fixture/CI audits shall be
  Rust.
- Existing Quoin shall remain the only permanent shared-runtime non-Rust
  accommodation.
- The Quoin accommodation shall not authorize new TypeScript or Node semantics.
- Existing Python, shell, Make, inline workflow and other executable paths shall
  remain classified as temporary pre-stable legacy paths.
- The tl-syntax executable census shall keep every temporary legacy path in the
  inventory.
- Stable qualification shall exclude every temporary legacy path.
- A bounded owner disposition shall authorize only its named pre-stable use.
- Stable qualification shall require a Rust/shared replacement with positive
  and failure-path parity for every required legacy behavior.
- The inventory shall distinguish data schemas and foreign-language fixture
  samples from executable logic.
- The census unit shall be one tracked executable entry point or interpreter/
  runtime invocation.
- The tl-syntax executable census shall apply this exhaustive candidate-class
  set: released shared tool, tl-syntax-owned Rust, owner-dispositioned legacy
  executable and missing reusable capability.
- If zero or multiple candidate classes match, then the tl-syntax executable
  census shall produce an ambiguity error rather than choosing by order.
- The tl-syntax executable census shall include tracked scripts/binaries, build scripts, generated
  workflow steps, inline/nested interpreter or runtime invocations, dynamically
  assembled commands and executable generators.
- If an executable construction cannot be resolved, then the tl-syntax
  executable census shall keep it unclassified in the population.
- A missing shared runner, source export, integrator format, evidence store or
  qualification contract shall remain an upstream dependency.
- tl-syntax shall not re-create a missing shared capability in this repository.

## Acceptance Criteria

| ID | Criteria | Verification |
|---|---|---|
| FR-018-AC-1 | Every tracked executable entry point/runtime invocation is present exactly once with language, purpose, authority, owner, disposition, parity/failure evidence and resume condition; adding an entry point or producing zero/multiple class matches makes the census fail until classified. | Test (TC-064) |
| FR-018-AC-2 | Every new first-party executable readiness path is Rust; Quoin's retained accommodation does not admit another Node/TypeScript path. | Test (TC-064) |
| FR-018-AC-3 | Each replacement demonstrates positive and forced-failure parity before the legacy path is removed; missing parity leaves the replacement incomplete. | Test (TC-065) |
| FR-018-AC-4 | No missing reusable assurance capability is replaced by a local runner, parser, schema, evidence store, compatibility map or approval workflow. | Test (TC-066) |

## Dependencies

- `agent-ix/quire-research#64` owns the cross-repository policy/catalog;
  tl-syntax owns exhaustive local enumeration, classification, refusal and
  bounded legacy disposition.
- `agent-ix/engineering-assurance#34` owns the planned reusable Rust producer-
  execution boundary.
- `agent-ix/tl-syntax#16` owns the source-grounded shared-contract migration.
