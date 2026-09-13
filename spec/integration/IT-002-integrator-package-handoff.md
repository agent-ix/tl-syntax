---
id: IT-002
title: Preserve source facts through the real integrator-package contract
type: IT
relationships:
  - target: ix://agent-ix/tl-syntax/FR-016
    type: verifies
  - target: ix://agent-ix/tl-syntax/FR-017
    type: verifies
---

# IT-002: Preserve source facts through the real integrator-package contract

## Objective

Verify that the future real shared integrator-package writer/reader preserves
tl-syntax source-release facts and open adopter obligations without creating a
qualification, certification or acceptance claim.

## Target Integration

The system under test is the tl-syntax adapter to a compatible released
Engineering Assurance integrator-package contract. The test invokes the real
shared writer and strict reader. No repository-local format or mock reader is
admitted.

## Preconditions

Engineering Assurance has released an accepted Rust-consumable package
contract, and the compatibility matrix selects its exact identity/version/
digest. A human source-release disposition and its source facts exist for one
exact tl-syntax candidate. Until those conditions hold, this integration is
blocked and package emission remains unavailable.

## Inputs

One complete source-release fact set; two different adopter intended-use and
deployment configurations; open and completed adopter fields; and mutations of
each assumption, limitation, exception, negative result, review, decision,
profile, corpus, configuration and supersession identity.
Applicable license and reuse-right identities are included in the valid set and
mutated independently.

Completed adopter fields are preserved only as separately attributed adopter-
stage inputs. Package readability and the source-release disposition neither
validate those fields nor convert them into accepted adopter obligations.

## Test Procedure

1. Emit and read the valid package through the real shared contract.
   - IT-002-SC-01: every source-release and open adopter fact survives exactly with its stage and authority.
2. Pair the same source release with two intended-use/configuration subjects.
   - IT-002-SC-02: the reader returns two distinct open adopter subjects and no inherited acceptance.
3. Delete or alter each required fact independently and submit the bytes to the
   strict reader.
   - IT-002-SC-03: the reader refuses the mutation or retains the named incomplete/unrepresented obligation without improving disposition; license and reuse-right loss is not silent.
4. Remove the compatible shared contract selection.
   - IT-002-SC-04: tl-syntax reports unavailable and no local writer, schema, reader or approval path runs.
5. Force writer/reader spawn, timeout, signal, nonzero, truncation, partial-write
   and schema/version failures while a prior package exists.
   - IT-002-SC-05: no partial or prior package is exposed as new; only a candidate-bound temporary object that passes strict read and integrity verification becomes visible.
6. Supply conditional, concurrent and contradictory decision successors.
   - IT-002-SC-06: the package preserves condition/conflict state and cannot present unconditional or adopter acceptance.
7. Reconcile every material against authoritative license/reuse-right sources,
   then remove, conflict or substitute each authority independently.
   - IT-002-SC-07: one normalized SPDX expression and complete source set represents valid `AND`/`OR`/nested licensing; missing resolution is incomplete, conflicting resolution is conflict, and unverifiable authority is refused.
8. Retry identical package attempts, reuse one attempt identity with changed
   inputs, and race non-identical writers.
   - IT-002-SC-08: identical retries resolve to one verified object; identity reuse and non-identical races produce conflict without exposing a timing-selected winner.

## Expected Results

The shared package is lossless for source facts and explicit about every open
adopter obligation. Mutations fail closed. Neither source-release acceptance nor
package readability establishes publication, certification, monitor
qualification or adopter acceptance.

## License and Rights Analysis Procedure

TC-072 binds its analysis population to the exact candidate and includes every
tracked source file, emitted package member, `Cargo.lock` dependency, and
generated or embedded third-party material. The authoritative evidence for each
material is the repository license/notice set, the exact dependency package or
release metadata, and any immutable registry or upstream notice selected by the
review policy; every source is retained with its issuer, applicability basis,
identity and digest.

SUITE-013 supplies dependency-license and dependency-source facts for the Rust
closure. An independent rights reviewer reconciles those facts and the remaining
material population against the authoritative evidence, records the exact
candidate, tool/configuration identity, reviewer identity and policy, limitations
and complete source set, and assigns one normalized SPDX expression per
material. Missing disposition produces `incomplete`, incompatible resolutions
produce `conflict`, and an unverifiable authority produces `refused`. The output
is attributable engineering analysis; neither the automated check nor the
review is represented as legal advice, certification or a transfer of release
authority.

## Metadata

- Priority: High
- Target Integration: future released Engineering Assurance integrator package
- Automation: Planned real Rust/shared-contract integration test

## Dependencies

Engineering Assurance owns the shared package and use-specific qualification
contract. Quoin owns the retention/receipt interface, while the selected
external backend operator/custodian owns stored bytes. tl-syntax implements only
the reviewed consumer and handle verification after exact compatible releases
and authorities exist.

## Notes

This planned test is deliberately blocked on a real contract. A mock would hide
the central compatibility and losslessness boundary.

## Traceability

Planned TC-061, TC-062, TC-063, TC-066, TC-068 and TC-070 through TC-072 cover
its acceptance dimensions.
