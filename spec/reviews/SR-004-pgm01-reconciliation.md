---
id: SR-004
title: tl-syntax PGM-01 candidate reconciliation
type: SpecReview
analysis: base
scope: PGM-01 requirements and the tl-syntax v0.1 candidate
review_set: all
---

# tl-syntax PGM-01 candidate reconciliation

## Summary

PGM-01 candidate: `agent-ix/quire-contract-ir#12` at
`7f8130d3fdb160a98a7a7f445cc1eb7419a3c179`.

Envelope schema: `quire.derivation-evidence/v1`, SHA-256
`0946e235e9e4b0fa79e9b9ec27ae157b303c17de0a9408d3cc04968fb7152256`.

This is a provisional reconciliation against a review candidate. The exact
identity and every conclusion below must be checked again after PGM-01 merges.

The collector architecture was adapted for this repository from the
same-program `quire-contract-runtime` collector at immutable revision
`534691f5c8f21fd2457118a83add96cc2e265b49`: source paths
`scripts/build_evidence_envelope.py`, `scripts/collect_evidence.sh`, and
`scripts/validate_json_schema.py`; retrieved SHA-256 values respectively
`59b9f1d4e64885166b462f405955aa54eb0b093307aae79327228399e90d9885`,
`ccc81989405ffe9dc6663b8dd9332d1c76adbc47abc4ce48abbc7ad0e40777a3`,
and `b52c451f7d9611b7faf41cc5cd53c1bcb8a3e2b0415a3bf50d222e44a4046a86`.
The MIT OR Apache-2.0 design was independently adapted to tl-syntax commands,
corpus identities, complete failure retention, external checksum placement,
and strict date-time validation. Human review remains pending.

## Findings

| ID | Severity | Summary | Refs |
|---|---|---|---|
| FND-401 | medium | PGM-01 is still in review; the candidate policy and envelope digest are provisionally reconciled, but the exact merged identities and final human gates remain open. | PGM-01, AP-001, AA-001 |

## Policy mapping

| Policy requirement | tl-syntax disposition | Evidence or remaining gate |
|---|---|---|
| PGM-01-R01 schema compatibility | Formula, proposition-map, corpus, semantic-profile, local evidence-input, and evidence-manifest boundaries carry explicit v1 identities; unknown formula document versions are rejected. | FR-003, FR-004, FR-005; corpus schemas; local evidence schemas |
| PGM-01-R02 exact pins | Candidate evidence records source, policy candidate, schema, toolchain, dependency, parameter, input, and output digests. | Collection input, manifest, and canonical envelope |
| PGM-01-R03 release order | `tl-syntax` is an independent initial tag root; temporal consumers must pin its eventual source tag, commit, schema/corpus digests, and checksums. | PGM-01 candidate; FR-005; human tag decision remains open |
| PGM-01-R04 licensing and provenance | Crate, local schemas, and corpus are `MIT OR Apache-2.0`; publication remains disabled; no copied third-party syntax material is present. | Cargo.toml, schema notices, CONTRIBUTING.md, license audit |
| PGM-01-R05 clean-room grammar | No text grammar, parser table, or imported grammar fixture is implemented in this repository. | MRS-001 out-of-scope boundary and repository inspection |
| PGM-01-R06 human authority | Agent-assisted provenance and `@kreneskyp` reviewer identity are recorded; automation leaves approval and release pending. | Envelope provenance, CODEOWNERS, AP-001, AA-001 |
| PGM-01-R07 classification | `tl-syntax` is linked runtime and requires consuming-project verification. | AP-001 and README |
| PGM-01-R08 common envelope | Revision-scoped evidence emits every canonical core field and is gated by the pinned PGM-01 Draft 7 schema and validator. | Evidence collector and retained PGM-01 validation outputs |
| PGM-01-R09 retention and decision | New runs refuse overwrite, retain stdout/stderr and SHA-256s, and preserve limitations; no automated release decision is recorded. | Evidence collector, external checksum file, AA-001 |
| PGM-01-R10 qualification boundary | Source and evidence provide reusable support only and confer no consuming-project validation, accreditation, or certification. | AP-001 and README |

The governance candidate requires no public semantic API change. PGM-01 merge,
final identity reconciliation, protected-branch checks, independent CODEOWNER
review, and the human source-release decision remain external workflow gates.
