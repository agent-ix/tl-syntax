---
id: IF-001
title: "Native definition authority interface"
type: interface
relationships:
  - { target: ix://agent-ix/quire-spec-language/FR-051, type: implemented_by }
---
# [IF-001] Native definition authority interface

## Contract

```yaml
name: NativeDefinitionAuthority
associated_types: [AdmittedCompiledPackage, CheckedPredicate, NativeTemporalSubject, OwnerReadError]
operations:
  - name: checked_predicate
    inputs: [AdmittedCompiledPackage, declaration index, optional temporal node index, limits]
    output: checked predicate authority artifact or OwnerReadError
    semantics: derive one whole-clause Boolean or holds-expression leaf from the admitted owner graph
  - name: temporal_subject
    inputs: [AdmittedCompiledPackage, declaration index, limits]
    output: complete source-bound native temporal subject authority artifact or OwnerReadError
    semantics: derive the exact typed tree, leaf population, profiles, clock, activation and capture requirements
  - name: read_checked_predicate
    inputs: [exact bytes, independent package selection, limits]
    output: constructor-private CheckedPredicate or OwnerReadError
    semantics: strict canonical read and complete re-comparison with the admitted package
  - name: read_temporal_subject
    inputs: [exact bytes, independent package selection, limits]
    output: constructor-private NativeTemporalSubject or OwnerReadError
    semantics: strict canonical read and complete re-comparison with the admitted package
invariants:
  - no source text or caller-built AST is accepted as authority
  - every leaf belongs to one verified parent subject and exact source locus
  - every exported expression is Boolean under the selected native profiles
  - unknown fields versions profiles graph edges and resource overruns return no validated view
dispatch: exact ContractSelection only
```
