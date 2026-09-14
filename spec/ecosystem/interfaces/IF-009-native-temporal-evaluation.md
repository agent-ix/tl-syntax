---
id: IF-009
title: "Canonical native temporal evaluation interface"
type: interface
relationships:
  - { target: ix://agent-ix/quire-spec-language/FR-052, type: implemented_by }
---
# [IF-009] Canonical native temporal evaluation interface

## Contract

```yaml
name: NativeTemporalEvaluationAuthority
associated_types: [NativeTemporalRequest, ValidatedNativeTemporalRequest, NativeTemporalResult, ValidatedNativeTemporalResult, NativeResultRelation, NativeOwnerError, NativeOwnerLimits]
operations:
  - name: produce_request
    inputs: [validated native temporal subject, one obligation instance, exact position and valuation population, activation and capture input, four progress/closure owner references, completeness, correspondence, limits]
    output: canonical native evaluation request or NativeOwnerError
    semantics: bind one formula-wide evaluation input without accepting truth settlement or support
  - name: read_request
    inputs: [exact bytes, validated native temporal subject, limits]
    output: constructor-private ValidatedNativeTemporalRequest or NativeOwnerError
    semantics: strict canonical read and complete subject graph profile position valuation axis and identity revalidation
  - name: evaluate
    inputs: [ValidatedNativeTemporalRequest, original or validated direct predecessor relation, limits]
    output: canonical formula-wide native result or NativeOwnerError
    semantics: invoke only the owner FR-043 evaluator and retain its activation execution truth non-value settlement support premises and correction
  - name: read_result
    inputs: [exact bytes, ValidatedNativeTemporalRequest, exact relation, limits]
    output: constructor-private ValidatedNativeTemporalResult or NativeOwnerError
    semantics: re-evaluate the exact request and require byte equality before admitting the result
invariants:
  - no caller-supplied truth settlement support or self-asserted trust field enters result construction
  - one result is formula-wide and no checked-leaf result can substitute for it
  - every external owner reference remains opaque and is cross-checked by Contract IR rather than re-authored here
  - correction preserves immutable predecessor bytes and an exact direct edge
  - exact limits pass and every one-over failure returns no partial request result or Boolean
dispatch: exact native request and result ContractSelection values
```
