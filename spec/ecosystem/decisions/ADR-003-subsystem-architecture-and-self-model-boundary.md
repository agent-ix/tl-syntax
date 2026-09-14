---
id: ADR-003
title: "Organize the temporal ecosystem by semantic subsystem and keep self-modeling observational"
type: ADR
status: proposed
owner: tl-syntax-maintainer
relationships:
  - target: ix://agent-ix/tl-syntax/MRS-003
    type: depends_on
---
# ADR-003: Organize the temporal ecosystem by semantic subsystem and keep self-modeling observational

## Context

Epic #52 was delivered initially as seven repository tickets. The semantic
ownership is mostly correct, but specifications and several implementations
grew as flat feature files: Contract IR has multi-thousand-line identity,
expression, conformance and candidate predicate modules; TL syntax combines
graph/profile/error concerns in large peers; result and observation owner
contracts are incomplete. Continuing record-by-record would lock cross-owner
shapes before the complete object/state/dependency model exists.

The intended product is one ecosystem that will later model and improve itself.
That goal needs stable inspectable metadata, but permitting generated models or
their analyses to become their own authority would create a circular proof and
an unsafe self-modifying contract path.

## Decision

Use DOM-001 as the application bounded context and organize normative artifacts
under `spec/ecosystem/{domain,model,interfaces,process,decisions,reviews}`.
Owner repositories retain their own normative implementation FRs and schemas,
but those artifacts implement the shared objects/interfaces rather than
redeclaring cross-owner payloads.

The runtime dependency direction is:

```text
tl-syntax <- tl-parse
tl-syntax <- tl-mltl <- tl-rewrite
quire-contract-model -> quire-spec-language -> quire-protocol
quire-spec-language -----\
quire-observation --------+-> quire-contract-ir -> consumers
quire-protocol -----------/
tl-syntax + tl-mltl -----/
```

The normative dependency direction is `quire-specification -> every owner and
bridge specification`. It is a specification/reference edge, not a Cargo edge:
the shared repository fixes meaning, identity composition and closed
vocabularies, while each executable owner keeps the one canonical wire type and
strict reader for its own artifact. This distinction prevents both semantic
restatement and a shared runtime-wire dependency cycle.

An arrow points from owner to consumer. Test-only compatibility edges may point
back to a consumer only when Cargo keeps them outside production code and the
dependency manifest names their exact historical purpose. No production cycle
is admitted.

Apply this target source topology while preserving public re-exports and every
accepted wire/profile identity:

| Repository | Target semantic modules |
| --- | --- |
| `quire-specification` | shared `objects::{foundation,temporal,observation,protocol}` rulings, with no executable consumer wire crate |
| `tl-syntax` | `formula::{graph,profile,document}`, `signal::{domain,catalog,binding,document}`, `contracts::{limits,manifest}` |
| `tl-parse` | `dialect::{v1,v2,v3}`, `lexer`, `parser`, `formatter`, `diagnostic`, with shared traversal separated from dialect policy |
| `tl-mltl` | `future`, `past::{history,requirement,evaluate,result}`, `wire::{trace,request,report}`, `clock`, `mapping` |
| `tl-rewrite` | `catalog`, `engine::{future,past}`, `equivalence`, `report`, `replay` |
| `quire-spec-language` | existing compiler/runtime subsystems plus `protocol_artifact::{checked_predicate,temporal_subject,native_temporal::{request,result}}`; checked definitions derive only from admitted v2 packages and native result truth derives only from its evaluator |
| `quire-observation` | `authority::{observation,population,position,clock,capture,progress,closure,completeness,availability}` over qualified immutable observation state |
| `quire-protocol` | `result::{record,reader,global,lineage,contract_ir}` plus the existing separate refusal subsystem |
| `quire-contract-ir` repository | cycle-free `quire-contract-model` package containing the existing shared substrate; compatibility-reexporting `quire-contract-ir` package with `predicate::{admission,definition,artifacts,valuation,decision,reader}`, `temporal::{admission,formula,valuation,request,correspondence,join,decision,reader}`, and observational `ecosystem_model::{manifest,graph,document,reader}` |

The package split is required by the real graph: QSL already consumes Contract
IR's foundational types, while the new bridge must consume QSL's
constructor-private owner views. QSL therefore pins `quire-contract-model`
under its existing dependency key, preserving Rust imports, and the bridge
package depends on both. Keeping the foundation and bridge in one package would
form a Cargo cycle; copying either side's wire types would form a second
authority.

A source move alone does not change a public contract. Existing public Rust
paths remain available through deliberate re-exports for the current major
version. A semantic or wire change requires a successor identity and migration
rule; a source-only reorganization requires exact regression tests proving
unchanged canonical bytes, errors and outcomes.

Publish one bounded `TemporalEcosystemModel` describing repositories,
components, objects, interfaces, contracts, revisions, dependencies, tests and
reviews. Its producer reads checked manifests/spec metadata only. The model is
never an authorization input to its own revision, never executes semantics,
never records acceptance, and emits only proposals that traverse the ordinary
owner review/merge path.

## Alternatives rejected

- Continue one FR and one source file per incoming ticket: rejected because it
  hides shared identities/state transitions and produces cross-repository
  semantic drift.
- Combine all crates into one monolith: rejected because owner boundaries,
  `no_std` support, release cadence and explicit contract selection are useful
  architecture, not accidental packaging.
- Introduce a shared wire crate containing every owner type: rejected because
  it would become a second authority and create dependency cycles. Shared
  conceptual objects live in the spec; executable types stay with one owner.
- Let Contract IR accept reader callbacks or trait objects from arbitrary
  plugins: rejected because FR-025/FR-026 require supplied validated immutable
  values and a closed selected contract set.
- Let the exported ecosystem model authorize or verify itself: rejected as
  circular evidence and uncontrolled self-modification.

## Consequences

Tasks 1–5 remain valid delivered baselines but must be reconciled against this
topology and formal model. Modules shown to mix responsibilities are split with
compatibility re-exports and full corpus replay. Missing owner contracts are
implemented before Contract IR removes its temporary data-only seams. The
predicate and temporal bridges become sibling subsystems over shared contract,
canonicalization, limit and diagnostic foundations.

Where valid owner result vocabularies differ, Contract IR consumes
owner-published constructor-private result/mapping views selected by exact
contract. The QProtocol mapping consumed by FR-025 is predicate-leaf-bound and
supplies explicit valuation cells. It is not a native temporal result and is
never compared to a TL formula result. QSL therefore owns a canonical
formula-wide native request/result sibling to the TL request/result; Contract IR
constructs both inputs from the same admitted observations and compares the two
formula-wide results without invoking either evaluator. It does not require an
owner to adopt Contract-IR labels and does not normalize by display text.

This reorganization increases short-term cross-repository work but makes
ownership, versioning, generated models and future self-analysis explicit. It
does not authorize external qualification, production monitoring, alternate
source languages, or automatic acceptance of generated changes.
