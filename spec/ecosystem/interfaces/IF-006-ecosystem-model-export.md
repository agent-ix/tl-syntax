---
id: IF-006
title: "Non-authoritative ecosystem model export"
type: interface
relationships:
  - { target: ix://agent-ix/quire-contract-ir/FR-027, type: implemented_by }
---
# [IF-006] Non-authoritative ecosystem model export

## Contract

```yaml
name: TemporalEcosystemModelExport
associated_types: [ComponentNode, ContractNode, ObjectNode, EvidenceNode, DependencyEdge, ModelDocument, ModelReadError]
operations:
  - name: export_model
    inputs: [checked repository manifests and exact merged revisions, limits]
    output: canonical bounded ModelDocument or ModelReadError
    semantics: describe components objects interfaces versions dependency pins tests and review evidence without running semantic producers
  - name: read_model
    inputs: [exact bytes, model ContractSelection, limits]
    output: validated immutable ModelDocument or ModelReadError
    semantics: strict canonical read with referential integrity and acyclic runtime dependency checks
invariants:
  - model output cannot authorize a contract revision or mark its own evidence accepted
  - model output cannot alter owner artifacts or become an evaluator oracle
  - unknown or cyclic ownership and runtime dependency edges refuse
  - improvement output is a proposal requiring the ordinary external owner review and merge path
dispatch: exact model ContractSelection
```
