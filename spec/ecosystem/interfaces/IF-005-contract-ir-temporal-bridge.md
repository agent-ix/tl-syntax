---
id: IF-005
title: "Contract IR temporal bridge interface"
type: interface
relationships:
  - { target: ix://agent-ix/quire-contract-ir/FR-025, type: implemented_by }
  - { target: ix://agent-ix/quire-contract-ir/FR-026, type: implemented_by }
---
# [IF-005] Contract IR temporal bridge interface

## Contract

```yaml
name: ContractIrTemporalBridge
associated_types: [PredicateProjectionDecision, PredicateValuationDecision, TemporalProjectionDecision, TemporalResultJoinDecision, BridgeDiagnostic]
operations:
  - name: project_predicate_set
    inputs: [validated native checked predicates, exact native and target selections, limits]
    output: predicate projection decision and admitted sibling artifacts when applicable
    semantics: deterministic sorted bijection to Boolean signals and propositions under FR-025
  - name: value_predicate
    inputs: [admitted correspondence, validated availability and protocol predicate-result mapping views, exact mapping selection, limits]
    output: predicate valuation decision
    semantics: total mapping preserving every non-value and correction state under FR-025
  - name: project_temporal_subject
    inputs: [validated native temporal subject, admitted predicate artifacts and valuations, validated observation context, exact TL selections, limits]
    output: temporal projection decision and evaluator artifacts when admitted
    semantics: deterministic node formula valuation history or trace and request construction under FR-026
  - name: join_results
    inputs: [admitted correspondence, validated availability native-result and TL-result mapping views, exact owner result and mapping selections, limits]
    output: temporal result join decision
    semantics: compare independent normalized axes and direct predecessor relations under FR-026
invariants:
  - contract admission precedes owner byte interpretation
  - no parser evaluator callback plugin network or ambient lookup is reachable
  - no owner identity is copied into a bridge authority domain
  - bridge-normalized labels are derived by exact owner mapping contracts and are never required as owner wire labels
  - every non-admitted decision has ordered nonempty typed causes and no usable partial artifact
dispatch: exact bridge profile and complete ContractSelection set
```
