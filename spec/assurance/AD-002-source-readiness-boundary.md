---
id: AD-002
title: tl-syntax source-readiness and integrator boundary
type: ArchitectureDescription
status: proposed
owner: tl-syntax-maintainer
system: tl-syntax source-readiness preparation and future integrator handoff
relationships:
  - target: ix://agent-ix/tl-syntax/MRS-004
    type: realizes
  - target: ix://agent-ix/tl-syntax/AP-002
    type: references
---

# tl-syntax source-readiness and integrator boundary

## System Boundary

tl-syntax owns its Rust domain producers, exact candidate/configuration inputs,
component-specific assumptions/limitations and adapter to accepted shared
contracts. Engineering Assurance owns compatibility, future reusable producer-
execution and integrator/use-specific qualification contracts. Quire owns
source-grounded static export. Quoin owns the shared evidence-retention and
orchestration contract; current workspace bytes remain transient until an exact
backend/handle lifecycle passes retrieval.
GitHub branch protection/review history and the human release owner remain
external authorities. Native Quire is the only editable formal-clause language;
tl-syntax is internal representation infrastructure.

## Views

The source-readiness flow is:

```text
exact Git source + Cargo/toolchain/config/profile/corpus identities
  -> tl-syntax Rust domain producers
  -> compatible released Quire source export
  -> compatible released Engineering Assurance classification/contracts
  -> Quoin record/attestation/intake/receipt and conditional retained handle
  -> independent spec/code/gap reviews
  -> attributed human source-release decision
  -> future shared integrator package with adopter fields still open
```

The external context is:

```text
PGM-01 + native-language ruling + LR08 policy
                    |
Git/repository -> tl-syntax Rust readiness projection/admission adapter
                    |                 |                 |
        released Quire/EA/Quoin   GitHub review     human release owner
                    |                                   |
          selected retention backend/operator      downstream integrator
```

Failure remains visible at the stage where it occurs. No later stage repairs a
missing producer input, identity, review, limitation or decision. Historical
facts remain immutable and are linked by supersession/invalidation.

The current executable-path inventory for review distinguishes:

| Class | Current path | Disposition boundary |
|---|---|---|
| Rust domain/library logic | `src/`, Rust examples/tests/fuzz targets | Retain as tl-syntax-owned; every new first-party path is Rust. |
| Existing Python domain/assurance helpers | corpus oracle, feature gate, assurance adapter | Explicit legacy inventory; replace with reviewed Rust/shared capabilities or obtain bounded owner disposition with parity/failure evidence. |
| Existing shell/Make/workflow orchestration | Makefile, shell audits, hosted YAML/inline commands | Not a trust root; inventory and migrate reusable executable semantics under LR08. |
| Existing Quoin Node/TypeScript | installed shared Quoin release | Sole recorded accommodation; contain rather than copy or broaden. |
| Quire and Engineering Assurance | released shared tools/contracts | Consume exact compatible artifacts; do not fork or reproduce locally. |

## External Dependencies and Trust

`Assumed` means tl-syntax preserves an attributable input without proving the
external authority's semantic correctness. `Guaranteed` means a named planned
contract test verifies this consumer boundary; where the required release is
missing, the guarantee remains unavailable rather than falling back locally.

| Dependency | Trust mode | Contract and current state |
|---|---|---|
| PGM-01 and native-language ruling | Assumed | Governance/source-language authority; bind exact immutable identity before implementation. |
| Git repository/object/materialization | Guaranteed boundary; Git correctness/provenance assumed | TC-059/060/069 bind/refuse identities; Git implementation and repository provenance remain external assumptions. |
| Rust/Cargo toolchain and public package delivery | Guaranteed identity; semantics/provenance assumed | FR-014 identity mutations; no compiler or registry certification claim. |
| Released Quire source export and Engineering Assurance classifier | Guaranteed when available | IT-001; unavailable until the `tl-syntax#16` accepted release set exists. |
| Released Quoin record/attestation/intake/receipt | Guaranteed when available | IT-001 real interfaces; Quoin does not execute producers or decide sufficiency. |
| Retention backend/operator | Guaranteed when selected | TC-067 verifies handle/content/lifecycle after workspace deletion; currently unselected/unavailable. |
| GitHub review/event facts | Assumed input, guaranteed binding | TC-073 checks exact policy/actor/subject binding; authenticated event semantics remain external. |
| Reviewer-independence and human-release policy | Assumed authority, guaranteed admission | FR-017/TC-073; currently unavailable until an immutable policy/event source and actor set are selected. |
| LR08 cross-repository language policy | Assumed | `quire-research#64`; tl-syntax guarantees only its local census/classification under TC-064/065. |
| Future Engineering Assurance integrator package | Guaranteed when available | IT-002; currently no accepted compatible contract. |
| Downstream intended-use/deployment facts | Assumed input, guaranteed separation | IT-002 preserves adopter attribution and non-transfer of acceptance. |
| License/reuse-right authority | Assumed authority, guaranteed mapping | TC-072 checks material mapping; it does not establish legal validity. |
| Hosted CI provider/run event | Assumed optional observation | Only a human-dispatched exact run may be cited; no dispatch is required here. |

## Decisions

1. Source-readiness identity is the product of exact source and declared
   configuration, not a branch name, command name or latest compatible version.
2. Developer observation, retained evidence, review and human decision are
   orthogonal facts. No aggregate score carries release authority.
3. The future integrator package retains source facts but leaves intended-use,
   deployment and validation obligations open for each adopter.
4. Missing shared capabilities return unsupported/unavailable. Local schema,
   parser, runner, store, compatibility map and approval substitutes are
   prohibited.
5. “Source” in this architecture means the tl-syntax Rust repository candidate,
   never an alternative user-authored temporal source language.
6. The `readiness projection` and `admission adapter` are tl-syntax Rust
   component projections/checks over released shared types. They are not a new
   generic evidence schema, aggregate qualification interpreter or approval
   workflow.

## Risks

- `agent-ix/tl-syntax#16` cannot resume until an immutable Engineering Assurance
  release accepts the source-grounded Quire release and artifact shape.
- The latest immutable Engineering Assurance release does not yet carry its
  corrected human-acceptance predicate or accept Quire 0.32; branch-head state
  and a repository-local compatibility fixture are not substitutes.
- The general shared integrator-package contract is not yet selected; IT-002
  remains blocked on a real accepted Rust-consumable release.
- `agent-ix/engineering-assurance#11/#34` must reconcile use-specific
  qualification and bounded Rust producer execution; tl-syntax must not fill
  that gap locally.
- Existing Python/shell/Make/inline-workflow paths require the LR08 inventory,
  Rust parity or explicit owner disposition before stable qualification use.
- Quire status-column classification and Quoin binary-attachment/non-release-
  build profile gaps remain explicit prerequisites when the selected evidence
  depends on them; local parsers, archives or false `release` labels are
  prohibited.
- The measured Make false-success class remains an open limitation; a Quoin
  record constrains produced bytes but cannot prove an omitted command ran.
- Existing retention prose conflicts over ignored `target/` state versus a
  repository evidence area. No durable-retention claim is admitted until an
  exact released shared backend, immutable handle, retrieval result and
  lifecycle rule resolve that conflict.
