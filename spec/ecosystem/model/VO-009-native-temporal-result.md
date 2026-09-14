---
id: VO-009
title: "Native temporal evaluation request and result"
type: value_object
relationships:
  - { target: ix://agent-ix/quire-spec-language/FR-052, type: implemented_by }
---
# [VO-009] Native temporal evaluation request and result

## Properties

- **request_identity** — owner-derived identity over the selected checked temporal subject, obligation instance, bridge correspondence, profile, clock, position/valuation population, activation/captures, four progress/closure references, completeness and effective limits.
- **formula_subject** — exact FR-051 checked temporal subject identity/digest and semantic obligation instance; never one checked-leaf result.
- **positions_and_valuations** — sorted semantic positions with admitted order, opaque observation identities and an explicit Boolean for every reachable leaf at every required position.
- **orthogonal_context** — independent decision-scope progress, decision-scope closure, surrounding-execution progress, surrounding-execution closure and completeness references/states.
- **result_identity** — owner-derived identity over the exact request, formula-wide native evaluator output, effective limits and correction relation.
- **outcome** — activation, execution, typed truth or non-value, settlement and exact decision-support position identities produced only by native evaluation.
- **relation** — original, superseding or invalidating with exact direct predecessor identity/digest and corrected request identity.

The request and result are immutable canonical QSL owner artifacts with public
strict readers. Contract IR proves that their opaque observation references
equal the separately admitted Quire Observation views; QSL owns their native
semantic evaluation and does not claim observation authority.
