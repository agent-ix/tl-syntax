---
id: IF-004
title: "Canonical TL core interface"
type: interface
relationships:
  - { target: ix://agent-ix/tl-syntax/FR-014, type: implemented_by }
  - { target: ix://agent-ix/tl-parse/FR-009, type: implemented_by }
  - { target: ix://agent-ix/tl-mltl/FR-018, type: implemented_by }
  - { target: ix://agent-ix/tl-rewrite/FR-010, type: implemented_by }
---
# [IF-004] Canonical TL core interface

## Contract

```yaml
name: CanonicalTemporalLogicCore
associated_types: [Formula, SignalCatalog, PropositionMap, PositionHistory, Trace, EvaluationRequest, EvaluationReport, MappedEvaluationView, ResultMappingSelection, RewriteReport, TlError]
operations:
  - name: read_formula
    inputs: [exact bytes, formula ContractSelection, semantic profile, limits]
    output: validated canonical Formula or TlError
    semantics: strict version profile graph and resource admission
  - name: read_signal_binding
    inputs: [exact catalog bytes, exact proposition-map bytes, target ContractSelections, limits]
    output: validated bijective Boolean signal binding or TlError
    semantics: strict sibling-artifact join without private wire imports
  - name: read_history
    inputs: [exact bytes, selected observation assertions clock and pure-past profile, limits]
    output: validated PositionHistory or TlError
    semantics: preserve origin anchor zero-based order completeness and clock mapping
  - name: read_trace
    inputs: [exact bytes, selected observation assertions clock and future profile, limits]
    output: validated Trace or TlError
    semantics: preserve the complete position-bound Boolean valuation rectangle
  - name: read_evaluation_request
    inputs: [exact bytes, validated formula history-or-trace and exact selections, limits]
    output: validated EvaluationRequest or TlError
    semantics: reject every stale or cross-wired formula valuation clock profile and owner identity
  - name: read_evaluation_report
    inputs: [exact bytes, validated request and exact selections, limits]
    output: validated EvaluationReport or TlError
    semantics: authenticate canonical evaluator output and every independent result axis
  - name: evaluate
    inputs: [validated Formula, validated PositionHistory or Trace, exact EvaluationRequest, limits]
    output: EvaluationReport
    semantics: bounded future or origin-complete past evaluation under one selected closed profile
  - name: map_evaluation_report
    inputs: [validated EvaluationReport, expected formula history-or-trace request correspondence and observation identities, ResultMappingSelection, limits]
    output: constructor-private MappedEvaluationView or TlError
    semantics: total deterministic extraction of independent evaluator axes without adopting Contract-IR wire vocabulary
  - name: rewrite
    inputs: [validated Formula, selected rule catalog, limits]
    output: rewritten Formula plus RewriteReport or TlError
    semantics: apply only declared profile-preserving equivalences and retain replay identity
invariants:
  - native Quire remains the only editable formal language
  - formula v1 and established future profiles retain bytes and semantics
  - no mixed profile is admitted unless a successor profile explicitly specifies it
  - every accepted wire family has canonical schema bytes and a bounded public strict reader
dispatch: exact formula semantic evaluator and rewrite ContractSelections
```
