---
id: FR-031
title: Check API and wire compatibility against the previous release
type: FR
relationships:
  - target: ix://agent-ix/tl-syntax/FR-004
    type: depends_on
  - target: ix://agent-ix/tl-syntax/FR-029
    type: depends_on
---

# FR-031: Check API and wire compatibility against the previous release

## Description

When tagging is proposed for a coordinated TL release after 0.3.0, the
release gate shall compare each crate's public Rust API and retained wire
formats against its previous release tag.

## Inputs

- The exact previous and proposed release tag commits for every crate.
- `cargo-semver-checks` results and each crate's CHANGELOG.
- Versioned owner golden files and their recorded digests.

## Outputs

- A per-crate API compatibility report, migration-note reconciliation and
  byte-level wire compatibility report.

## Behavior

Each reported API break has a corresponding CHANGELOG entry identifying the
break and a usable migration note. An unreported breaking change or an
unmatched tool finding blocks the release. A declared breaking release may
contain documented breaks; the gate does not misstate them as compatible.
Existing versioned wire formats retain byte-identical golden files and
decoding semantics. New versions use new schema identities rather than
rewriting prior-version bytes. Tool failure or an absent prior tag is a failed
gate, not a pass.

## Acceptance Criteria

| ID | Criteria | Verification |
|---|---|---|
| FR-031-AC-1 | `cargo-semver-checks` compares each candidate to the preceding immutable release tag, and every break maps to one CHANGELOG break and migration note. | Test (TC-175) |
| FR-031-AC-2 | Existing golden wire files are byte identical to the previous tag; altered bytes, absent schema, or a changed legacy decoder fail. | Test (TC-176) |

## Dependencies

FR-004 owns tl-syntax versioned wire compatibility; each downstream crate owns
its own public API and wire formats.
