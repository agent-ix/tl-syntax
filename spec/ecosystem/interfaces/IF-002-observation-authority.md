---
id: IF-002
title: "Observation authority interface"
type: interface
relationships:
  - { target: ix://agent-ix/quire-observation/FR-004, type: implemented_by }
---
# [IF-002] Observation authority interface

## Contract

```yaml
name: ObservationAuthority
associated_types: [ObservationRecord, Population, PositionLedger, ClockBinding, CaptureEnvironment, ProgressAssertion, ClosureAssertion, CompletenessAssertion, ResultAvailabilityAssertion, OwnerReadError]
operations:
  - name: derive_context
    inputs: [qualified immutable observation state, selected authority contracts, limits]
    output: immutable observation-context artifact set or OwnerReadError
    semantics: derive observation population position clock capture progress closure completeness and result-availability documents without deriving protocol activation or truth
  - name: read_observation
    inputs: [exact bytes, expected producer source subject and predecessor selections, limits]
    output: validated ObservationRecord or OwnerReadError
    semantics: strict read preserving the FR-260 and FR-288 correction preimage
  - name: read_population
    inputs: [exact bytes, expected membership selection sources configuration closure and dependencies, limits]
    output: validated Population or OwnerReadError
    semantics: rederive the FR-263 population and FR-264 explicit-members identities
  - name: read_position_ledger
    inputs: [exact bytes, ContractSelection, limits]
    output: validated PositionLedger or OwnerReadError
    semantics: strict read preserving zero-based order and anchor snapshot invocation identities
  - name: read_clock_binding
    inputs: [exact bytes, expected clock family and parameters, limits]
    output: validated ClockBinding or OwnerReadError
    semantics: preserve event-position fixed-sample or timestamped-event selection without using timestamp order as causality
  - name: read_capture_environment
    inputs: [exact bytes, expected native subject trigger anchor and bindings, limits]
    output: validated CaptureEnvironment or OwnerReadError
    semantics: preserve the complete sorted typed value population without deriving activation
  - name: read_progress
    inputs: [exact bytes, ContractSelection, limits]
    output: validated ProgressAssertion or OwnerReadError
    semantics: keep decision scope and surrounding execution distinct
  - name: read_closure
    inputs: [exact bytes, ContractSelection, limits]
    output: validated ClosureAssertion or OwnerReadError
    semantics: authenticate scope closure without deriving it from progress or completeness
  - name: read_completeness
    inputs: [exact bytes, ContractSelection, limits]
    output: validated CompletenessAssertion or OwnerReadError
    semantics: validate complete fact population and contradiction state
  - name: read_result_availability
    inputs: [exact bytes, ContractSelection, limits]
    output: validated ResultAvailabilityAssertion or OwnerReadError
    semantics: authenticate each required producer and contract availability classification
invariants:
  - member object identities remain opaque while owner population identities are rederived
  - no state is inferred from another authority axis
  - all positions and fact populations are complete sorted distinct and bounded
  - corrections create new immutable revisions and retain predecessor identities
  - strict-reader refusal returns no trusted partial assertion
dispatch: exact ContractSelection for each of the nine authority families
```
