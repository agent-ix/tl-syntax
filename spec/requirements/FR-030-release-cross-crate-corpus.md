---
id: FR-030
title: Replay owner corpora across release pins
type: FR
relationships:
  - target: ix://agent-ix/tl-syntax/FR-005
    type: depends_on
  - target: ix://agent-ix/tl-syntax/FR-024
    type: depends_on
  - target: ix://agent-ix/tl-syntax/FR-029
    type: depends_on
---

# FR-030: Replay owner corpora across release pins

## Description

When preparing a coordinated TL release, the release gate shall replay the
digest-pinned tl-syntax owner corpora through tl-parse, tl-rewrite and tl-mltl
at exactly the candidate revisions selected by FR-029.

## Inputs

- The owner corpus manifests, bytes, schema identities and digests.
- The four exact candidate revisions and their built binaries.

## Outputs

- A case-by-case report of parse round-trip, rewrite equivalence and
  evaluation, including typed refusal results.

## Behavior

The gate reads each corpus from `tl_syntax::CORPUS_DIR` at the selected
tl-syntax revision. Every applicable case is replayed through all available
layers; an unsupported case is recorded as unsupported with its typed reason
and cannot count as a successful semantic replay. The oracle value comes from
the owner corpus, never from production evaluation. A digest mismatch, skipped
case, altered expected result or disagreement blocks the release. Applicability
and explicitly unsupported mappings are listed per case, so a newly supported
feature cannot be silently skipped.

## Acceptance Criteria

| ID | Criteria | Verification |
|---|---|---|
| FR-030-AC-1 | Every applicable owner case at the exact release pins is replayed through parse, rewrite and evaluation against its independent expected result. | Test (TC-173) |
| FR-030-AC-2 | A missing case, digest change, wrong pin, layer disagreement or unsupported-to-success promotion fails with a case and layer identity. | Test (TC-174) |

## Dependencies

FR-005 and FR-024 own corpus bytes and oracle derivations; the downstream
crates own their respective replay adapters.
