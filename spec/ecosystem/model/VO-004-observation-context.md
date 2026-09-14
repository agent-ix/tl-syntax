---
id: VO-004
title: "Immutable observation context"
type: value_object
relationships:
  - { target: ix://agent-ix/quire-observation/FR-004, type: implemented_by }
---
# [VO-004] Immutable observation context

## Properties

- **observation** — owner identity, positive revision and exact bytes/digest; any observation-document lifecycle state is distinct from every assessed scope axis.
- **positions** — ordered distinct zero-based position records, each with an owner position identity plus exact anchor, snapshot and invocation identities.
- **clock** — exact event-position or fixed-sample binding; fixed sample retains normalized epoch, positive rational period, opaque unit and position mapping.
- **activation requirements** — exact trigger, guard, scope and capture identities
  required by the native subject. Observation authority records the admitted
  trigger/capture facts but does not derive a protocol activation state.
- **capture** — immutable capture-environment identity, positive revision, exact bytes/digest and complete value bindings.
- **progress** — separate decision-scope and surrounding-execution assertions with scope, authority, boundary, clock, interval/history boundary, sorted source identities and independently supplied state `open` or `closed`.
- **closure** — separate decision-scope and surrounding-execution assertions with their exact scope, closure-authority and boundary identities and independently supplied state `open` or `closed`.
- **completeness** — assertion, boundary and population identities plus state `complete`, `incomplete`, or `contradicted` and complete fact population; completeness never supplies a progress or closure state.
- **availability** — assertion identity/revision/digest and independently classified required results: `available`, `not-yet-observed`, `producer-unavailable`, or `contract-unavailable`.

Every component is an authority artifact with its own selected contract and
reader. No activation, closure, progress, completeness or availability state is
inferred from another axis. Protocol activation is derived only by the protocol
owner from its obligation and these exact owner facts. A correction creates a
greater revision and new digest; it does not reopen or rewrite prior
observation bytes.
