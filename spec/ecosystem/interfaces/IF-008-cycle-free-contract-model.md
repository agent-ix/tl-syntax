---
id: IF-008
title: "Cycle-free Contract IR model foundation"
type: interface
relationships:
  - { target: ix://agent-ix/quire-contract-ir/FR-028, type: implemented_by }
---
# [IF-008] Cycle-free Contract IR model foundation

## Contract

```yaml
name: ContractModelFoundation
associated_types: [PackageModel, RequirementModel, ClauseModel, TypeModel, ExpressionModel, BindingModel, CanonicalIdentity, Diagnostic, ConformanceReport]
operations:
  - name: admit_model
    inputs: [exact contract bytes, contract selection, limits]
    output: constructor-private semantic model or Diagnostic
    semantics: preserve the existing Contract IR substrate without reaching any owner or bridge crate
  - name: canonicalize
    inputs: [validated semantic model, limits]
    output: existing canonical bytes and identity or Diagnostic
    semantics: preserve every accepted schema byte identity domain and refusal
invariants:
  - the package has no production or optional dependency on QSL observation protocol or TL crates
  - quire-contract-ir publicly re-exports the full existing model API
  - QSL may select this package under its existing dependency key without source import changes
  - owner readers flow toward the bridge and no reverse production edge remains
dispatch: exact existing Contract IR substrate selection
```
