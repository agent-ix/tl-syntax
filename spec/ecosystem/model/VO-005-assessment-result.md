---
id: VO-005
title: "Canonical assessment result"
type: value_object
relationships:
  - { target: ix://agent-ix/quire-protocol/FR-042, type: implemented_by }
---
# [VO-005] Canonical assessment result

## Properties

- **identity** — the derived `sha256-jcs` result identity over the complete closed canonical result with only `resultId` omitted; an enclosing artifact digest is computed separately over the complete canonical bytes.
- **subject_and_correspondence** — exact native subject, formula, trace, evaluator request, observation and correspondence identities applicable to the producer.
- **assessment_execution** — `completed`, `resource-incomplete`, `unsupported`, `failed`, or `refused`.
- **decision_scope_progress** — owner assertion selection/ref/revision/digest and independently supplied `open` or `closed` state.
- **decision_scope_closure** — exact scope/closure-authority/boundary identities and independently supplied `open` or `closed` state.
- **surrounding_execution_progress** — a second independently selected owner assertion and `open` or `closed` state.
- **surrounding_execution_closure** — a second exact scope/closure-authority/boundary fact and `open` or `closed` state.
- **complete_global_conformance_closure** — the fifth independent closure fact with its distinct `closed`, `open`, `incomplete`, `contradicted`, or `not-required` vocabulary and exact premise set.
- **truth** — `satisfied`, `violated`, `pending`, or `unavailable`, never inferred from execution or completeness.
- **settlement_basis** — `closed-scope`, `decisive-witness`, `decisive-counterexample`, `unsettled`, or `unavailable`.
- **activation** — exactly `triggered`, `inactive`, or `activation-unknown` when an obligation instance exists.
- **participation** — one closed `known` or `unknown` value object, never a downstream status label.
- **decision_support** — sorted distinct exact identities of facts that decide truth.
- **completeness** — embedded owner contract selection and exact assertion ref/revision/digest/state/fact population.
- **relation** — `original`, `superseding`, or `invalidating`, with the exact direct predecessor, corrected input and contradicted premise fields required by that alternative.
- **observation_and_protocol_adequacy** — two independent closed domains that cannot establish one another.
- **claim** — exact claim kind, one source-bound claim strength, admitted fragment and analysis backend identities.
- **provenance_and_limits** — exact producer/profile/model/config/source identities and effective bounded-work record.

The result is immutable canonical owner output with a public strict reader.
Only a valid completed/final combination may project to a Boolean. Open settled
truth requires its exact decisive support; non-completed execution uses
unavailable truth/basis; a completed closed decision scope cannot be pending. Embedded
progress/completeness assertions remain owner artifacts and are never
restamped by the result producer or a downstream join.
