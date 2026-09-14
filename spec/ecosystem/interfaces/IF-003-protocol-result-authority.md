---
id: IF-003
title: "Canonical protocol result authority interface"
type: interface
relationships:
  - { target: ix://agent-ix/quire-protocol/FR-042, type: implemented_by }
---
# [IF-003] Canonical protocol result authority interface

## Contract

```yaml
name: ProtocolResultAuthority
associated_types: [ProtocolAssessment, CanonicalResult, ValidatedResult, MappedResultView, ResultMappingSelection, ResultRefusal, ResultLimits]
operations:
  - name: produce_result
    inputs: [admitted protocol assessment, selected owner assertions, optional direct predecessor, ResultLimits]
    output: exact canonical result bytes and ValidatedResult or ResultRefusal
    semantics: encode every orthogonal execution truth settlement progress closure decision-premise completeness provenance and correction field
  - name: read_result
    inputs: [exact bytes, independent ContractSelection and expected identities, ResultLimits]
    output: constructor-private ValidatedResult or ResultRefusal
    semantics: strict canonical read and complete identity premise state and direct-predecessor validation
  - name: validate_lineage
    inputs: [finite complete result population, ResultLimits]
    output: valid immutable lineage or ResultRefusal
    semantics: reject missing predecessor self-reference cycles branches invented independence and revision regression
  - name: map_contract_ir_result
    inputs: [ValidatedResult, expected native subject correspondence and observation identities, ResultMappingSelection, ResultLimits]
    output: constructor-private MappedResultView or ResultRefusal
    semantics: total deterministic extraction of all independent result axes decision premises and predecessor fields under the selected mapping revision without adopting bridge wire vocabulary
invariants:
  - producer-owned source model configuration and assertion identities are retained in their original domains
  - decision-scope and surrounding-execution progress and closure remain independent
  - only a valid completed final result can carry a Boolean
  - refusal identity is independent of rendering and no failed operation returns partial canonical bytes
dispatch: exact result ContractSelection
```
