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
rewriting prior-version bytes; this is the default for any change not declared below. The gate compares every retained golden file
against the previous release tag. A changed or deleted file passes only when it
is declared in the checked-in exceptions file `release-gate/golden-exceptions.toml`
with its repo, path, kind (changed or deleted), the tag it is measured against
and a one-sentence reason. An undeclared difference, an exception of the wrong
kind, or a declared exception whose file no longer differs from that tag (stale)
fails the gate; an exception measured against an older tag is ignored. A
wire-bearing difference is never excused as bookkeeping. A declared exception may
record an in-place break of an existing schema identity instead of a new
identity, provided its reason names the break and the migration, and the release
CHANGELOG of the owning crate carries a matching entry with a migration note. Tool failure or an absent prior tag is a failed
gate, not a pass.

## Acceptance Criteria

| ID | Criteria | Verification |
|---|---|---|
| FR-031-AC-1 | `cargo-semver-checks` compares each candidate to the preceding immutable release tag, and every break maps to one CHANGELOG break and migration note. | Test (TC-175) |
| FR-031-AC-2 | Every retained golden file is byte identical to the previous tag unless declared in the exceptions file with a reason; an undeclared or stale difference, an absent schema, or a changed legacy decoder fails. A declared exception may record an in-place schema break when its reason names the break and the release CHANGELOG carries a matching migration note. | Test (TC-176) |

## Dependencies

FR-004 owns tl-syntax versioned wire compatibility; each downstream crate owns
its own public API and wire formats.
