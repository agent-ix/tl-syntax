---
id: SM-001
title: "Temporal assessment artifact lifecycle"
type: state_machine
---
# [SM-001] Temporal assessment artifact lifecycle

## States & Transitions

```mermaid
stateDiagram-v2
    [*] --> ContractAdmission
    ContractAdmission --> BaseNonValue: unknown unavailable or conflicting contract
    ContractAdmission --> OwnerRead: all selections admitted
    OwnerRead --> InvalidInput: strict reader refusal
    OwnerRead --> StaticProjection: owner artifacts valid
    StaticProjection --> ProjectionNonValue: unsupported incomplete unavailable failed refused or conflict
    StaticProjection --> EvaluationReady: exact formula valuation history or trace and request constructed
    EvaluationReady --> ProducerResult: bounded evaluator or producer executes
    ProducerResult --> ResultNonValue: noncompleted or unavailable result
    ProducerResult --> ResultJoin: both result artifacts available and valid
    ResultJoin --> Agreement: equal final or equal pending premises
    ResultJoin --> ResultNonValue: mismatch or invalid correction relation
    Agreement --> CorrectedRevision: later admissible corrected input
    ResultNonValue --> CorrectedRevision: later admissible corrected input
    CorrectedRevision --> ContractAdmission: new immutable request retaining direct predecessor
    BaseNonValue --> [*]
    InvalidInput --> [*]
    ProjectionNonValue --> [*]
    Agreement --> [*]
    ResultNonValue --> [*]
```

Every terminal transition returns one complete operation-specific decision or
one operation diagnostic and no usable partial artifact. A correction starts a
new immutable revision; it never mutates or reclassifies the predecessor in
place. Contract admission always precedes interpretation of bytes governed by
that contract.
