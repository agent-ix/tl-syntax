---
id: ADR-019
title: "Prefer structural and version identity to digests and pins across the numeric specifications"
type: ADR
status: proposed
owner: tl-syntax-maintainer
relationships:
  - target: ix://agent-ix/tl-syntax/ADR-014
    type: relates_to
  - target: ix://agent-ix/tl-syntax/FR-303
    type: realizes_requirement
  - target: ix://agent-ix/tl-syntax/FR-305
    type: realizes_requirement
  - target: ix://agent-ix/tl-syntax/FR-309
    type: realizes_requirement
---

# ADR-019: Prefer structural and version identity to digests and pins across the numeric specifications

## Status

Proposed (TL-419). Ready for human acceptance. The exemption question (Open
decision 3 of [ADR-017](./ADR-017-assume-guarantee-contracts.md)) is Peter's; this ADR
only removes what can be removed and states why each remaining item stays.

## Context

A digest or pin that has to be recomputed whenever a legitimate input changes is a
churn source, and family precedent ("the fairness-premises document does it") is not
a reason to repeat it. The first draft of P-TL-150 added schema digests, a corpus
manifest hash per file, a catalog and trace digest, a graph digest, retained-exchange
digests and a pinned target profile. This register lists each, the repository that
owns it, and its disposition. "Structural identity" means an identity string such as
`tl-syntax.predicate-table/v1` plus embedded schema bytes compared byte for byte, a
revision integer, or a count that can be re-derived; "version identity" means a recorded
version string. Dispositions: REMOVED (the item is deleted), REPLACED (a structural
or version identity takes over), RETAINED (kept, with a one-line reason).

## Decision

1. No new digest constant is added to any document defined by P-TL-150, and
   a corpus or document is identified by its version string and structure.
2. The register below is the closed list of digests and pins in P-TL-150. A digest or
   pin not in it is a defect.

| # | Item | Repo and requirement | Disposition | One-line reason |
|---|---|---|---|---|
| 1 | Schema SHA-256 constants of predicate-table, aggregation and contract-set | tl-syntax FR-303, FR-306, FR-309 | REMOVED | Identity is the version string plus the embedded schema bytes compared with the checked-in file. |
| 2 | Per-file SHA-256 in the numeric corpus manifest | tl-syntax FR-305 | REPLACED | Path, class, per-class case count and a semantic replay of each case against its declared outcome; immutability by `corpus/v1` versus `v2`. |
| 3 | `graph_identity` of the contract set (and its use in tl-mltl FR-059) | tl-syntax FR-309; tl-mltl FR-059 | RETAINED | A formula graph has no structural identity that distinguishes two different graphs of one shape; this is the existing `CONTENT_IDENTITY_DOMAIN_V1` binding that `fairness-premises/v1` already uses, and it stops a contract set being applied to the wrong graph. Subject to the human exemption decision. |
| 4 | Existing schema digest constants (`FORMULA_V1_SCHEMA_SHA256` and peers) asserted unchanged | tl-syntax FR-303-AC-3, FR-304-AC-3, FR-309-AC-3 | UNCHANGED | Pre-existing; the specified work only asserts they do not move. |
| 5 | Dialect v5 digest record | tl-parse FR-028 | REPLACED | `DIALECT_V5_REVISION` (an integer) and the identity `tl-parse.clean-ascii/v5`; no digest record. |
| 6 | Catalog digest in the numeric parse report | tl-parse FR-028 | REPLACED | The report echoes the catalog schema identity, signal count and binding count; consumers receive the catalog itself. |
| 7 | Numeric-trace schema SHA-256 constant | tl-mltl FR-053 | REMOVED | As item 1. |
| 8 | `catalogIdentity` (hex SHA-256) in `tl-mltl.numeric-trace/v1` | tl-mltl FR-053 | REMOVED | Every observation is already admitted against the supplied catalog (signal, kind, scale, bounds); a trace admitted against a different but compatible catalog means the same thing. |
| 9 | SHA-256 of the derived Boolean trace in the numeric evaluation report | tl-mltl FR-054 | REMOVED | The report carries the derived per-position truth values themselves. |
| 10 | Digests of the retained R2U2 exchange files and the one-byte-alteration test | tl-mltl FR-058, NFR-004 pattern | RETAINED | Bytes produced by an external tool have no structural identity and a digest is the only detection of silent alteration of evidence; also DEFERRED, since no such bytes exist. Subject to the human exemption decision. |
| 11 | Recorded R2U2 revision (`campaign/tool-versions.json`) | tl-mltl FR-058 | RETAINED | A version string identifying the external tool that produced the evidence, not a computed digest. |
| 12 | Pinned `TargetNumericProfile` | tl-syntax ADR-015; tl-mltl FR-057, FR-058 | RETAINED | It is measured target behaviour (data), not an identity; unpinned means refusal, so it cannot silently drift. |
| 13 | Content identities of the predicate table and catalog in request and result | tl-solve FR-205 | REPLACED | The result records the effective numeric domains (signal id, kind, scale, bounds) and the predicate-table identity string and predicate count it answered for. |

## Alternatives rejected

- Keep every digest because the family already uses them: rejected; precedent does
  not make a churn source necessary.
- Remove every digest including items 3 and 10: rejected for now; without them a
  contract set can be applied to a different graph and altered evidence passes replay.
  This is flagged, not hidden, for the human exemption decision.

## Consequences

Four repositories lose their numeric digest constants. Items 3, 10, 11 and 12 remain
and are the only ones whose change needs a deliberate human act.
