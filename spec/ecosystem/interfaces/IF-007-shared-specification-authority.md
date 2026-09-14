---
id: IF-007
title: "Shared specification authority interface"
type: interface
relationships:
  - { target: ix://agent-ix/quire-specification/FR-200, type: implemented_by }
---
# [IF-007] Shared specification authority interface

## Contract

```yaml
name: SharedSpecificationAuthority
associated_types: [SemanticObjectRef, AcceptedRevision, IdentityRule, ClosedVocabulary, BoundaryRule, SpecificationGap]
operations:
  - name: select_semantic_object
    inputs: [exact ix object identity, immutable merged revision]
    output: one accepted semantic object or SpecificationGap
    semantics: resolve shared object meaning without importing a consumer wire shape
  - name: verify_owner_allocation
    inputs: [semantic object, executable owner contract, mapping contracts]
    output: unique owner and total explicit cross-owner mappings or SpecificationGap
    semantics: detect duplicate authorities missing keys open member sets and unstated boundary conversions
invariants:
  - one semantic noun has one shared meaning and one executable wire owner per artifact family
  - shared specifications never become a runtime parser evaluator or wire crate
  - unresolved keys vocabularies or boundary conversions fail closed in every dependent plan
  - accepted owner-specific spellings remain distinct until an explicit total mapping relates them
dispatch: exact ix identity and immutable accepted revision
```
