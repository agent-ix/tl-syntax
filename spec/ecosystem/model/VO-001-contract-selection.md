---
id: VO-001
title: "Contract selection"
type: value_object
relationships:
  - { target: ix://agent-ix/quire-contract-ir/FR-028, type: implemented_by }
---
# [VO-001] Contract selection

## Properties

- **contract** — nonempty versioned semantic contract identity.
- **package_version** — exact released package/contract version, never a range or moving alias.
- **repository** — canonical `owner/repository` identity of the contract authority.
- **revision** — immutable 40-character lowercase Git object identity containing the accepted contract and schema.
- **schema_digest** — lowercase SHA-256 of the exact canonical schema bytes selected by the other four properties.

The value is immutable and equality requires all five properties to match
byte-for-byte. An unknown but well-formed tuple is `unsupported`; a selected
tuple whose accepted reader cannot be reached is `unavailable`; different
schema bytes under an equal tuple are `conflict`; malformed fields are refused
before a semantic decision. A branch, tag that can move, copied schema, local
Rust type, or semver range is not a ContractSelection.
