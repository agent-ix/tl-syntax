---
id: VO-002
title: "Authority artifact"
type: value_object
relationships:
  - { target: ix://agent-ix/quire-contract-ir/FR-028, type: references }
---
# [VO-002] Authority artifact

## Properties

- **contract_selection** — VO-001 selection admitted before decoding.
- **bytes** — one complete immutable canonical UTF-8 document within the selected byte/depth/count/string limits.
- **artifact_ref** — nonempty owner-defined identity whose domain is fixed by the selected contract.
- **revision** — positive owner revision; equality and ordering semantics come from the owner contract.
- **digest** — lowercase SHA-256 over the exact bytes unless the owner contract selects another explicitly named digest domain.
- **validated_view** — constructor-private value returned only by the selected public strict reader after canonical re-encoding and every semantic invariant succeeds.

The bytes, identity, revision and digest are an inseparable value. Changing any
one requires reader rejection or a new authority artifact. Consumers may retain
or reference the tuple but may not restamp it in a consumer digest domain.
Strict reading rejects invalid UTF-8/JSON, duplicate or unknown members,
trailing data, unknown versions, noncanonical bytes, graph/state violations and
every declared resource overrun without returning a usable partial view.
