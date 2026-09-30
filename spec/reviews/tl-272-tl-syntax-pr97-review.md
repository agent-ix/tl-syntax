---
id: SR-114
title: "tl-syntax#97 drop dangling PGM-01 citations review"
type: SpecReview
analysis: base
scope: "assurance/README.md; requirements-assurance.txt; spec/assurance/AD-002-source-readiness-boundary.md; spec/assurance/AP-002-progressive-source-readiness.md; spec/evidence/suites.md; spec/requirements/FR-018-require-human-source-release-decision.md; spec/requirements/StR-004-progressive-source-readiness.md; spec/reviews/SR-004-pgm01-reconciliation.md (deleted); spec/source-readiness.md; spec/spec.md; tests/shared_assurance.rs"
review_set: subset
---

# tl-syntax#97 drop dangling PGM-01 citations review

## Summary

Ticket: TL-272. PR: agent-ix/tl-syntax#97, branch `chore/drop-pgm01-citations`.
Methods run in this one document, scoped to the diff: spec-review
(base checklist over the changed spec text) and gap-analysis (planless: AC
meaning, trace, and code with no owning requirement). Rust review was not run as
a separate lane because the only `.rs` change is one comment line in
`tests/shared_assurance.rs`.

PGM-01 was a governance and evidence-policy standard in
`agent-ix/quire-contract-ir`. It was deleted there as tracking ceremony, and the
owner's rule is that tracking ceremony is deleted. This review checks that the
citations are gone, that the prose still reads correctly without them, and that
validation is unchanged.

## Verdict

**PASS with one low finding.** It can merge as is. FND-001 is an optional
wording fix.

- **No dangling live references.** No `ix://agent-ix/quire-contract-ir/PGM-01`
  edge is left anywhere in the tree. The PR removed all four. The remaining `PGM-01` and `pgm01` text
  is of two kinds:
  - archival records that the repo's own census treats as archival
    (`spec/plans/**`, `spec/reviews/**`);
  - literal identifiers of a removed reader: `map_pgm01_bytes`, and the quoted
    refusal string in `assurance/README.md:55`.
- **The SR-004 deletion was ceremony only.** The file held:
  - a mapping of tl-syntax onto PGM-01-R01..R10;
  - one medium "finding" that restated the open human-review and release gates;
  - no FR, AC or test.

  Nothing live links to it.
- **No AC lost meaning.** Every spec edit is in a Dependencies section, a
  frontmatter edge, a context diagram, or narrative. No AC row, TC, or test
  matrix row changed.
- **Dropped clause.** The rewrite drops a PGM-01 obligation along with the
  citation:
  - `spec/source-readiness.md:159` drops "in the order governed by
    quire-contract-ir#4/PGM-01", which was PGM-01-R03 release order.

  These are the ceremony the owner rule deletes, not a loss of behaviour. The
  remaining prerequisites (candidate, reviews, human decision, signed
  tag/checksum) are still listed.
- **Validation matches origin/main.** The Makefile's `quire validate` output,
  with paths normalised, is identical on `origin/main` and the PR head: the same
  28 pre-existing failing documents. The only change is the document count,
  273 to 272, from the deleted SR-004. `scripts/check_spec_id_uniqueness.py`
  passes.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | The retained-evidence statement paraphrases the recorded refusal as "an unknown schema version". The mapper's literal reason was "unknown PGM-01 schema version", which `assurance/README.md:55` still quotes. Fix: quote the literal reason, e.g. "refused with the reason 'unknown PGM-01 schema version'". A tool's output string is not a citation. | assurance/pins.json:30 |

## Scope examined

| Unit | Role | Result |
| --- | --- | --- |
| spec/spec.md (MRS-001 frontmatter, Purpose, References) | examined | clean: edge, governing-policy paragraph and reference entry removed; no orphaned wording |
| spec/assurance/AP-002-progressive-source-readiness.md (relationships) | examined | clean |
| spec/source-readiness.md (relationships, prerequisite DAG, sequencing bullet) | examined | clean |
| spec/assurance/AD-002-source-readiness-boundary.md (context diagram, dependency row) | examined | clean |
| spec/requirements/FR-018 Dependencies | examined | clean: authority now rests on the human-release-owner policy alone |
| spec/requirements/StR-004 Dependencies | examined | clean |
| spec/evidence/suites.md SUITE-006/007 note | examined | clean |
| assurance/README.md:56 | examined | clean |
| requirements-assurance.txt:5 | examined | clean |
| tests/shared_assurance.rs:808 (comment) | examined | clean; the file is itself archival in `is_archival_record` |
| spec/reviews/SR-004-pgm01-reconciliation.md (deleted) | examined | ceremony only; no inbound links |

## Gap analysis

Plan completion: not assessed.

- No requirement, AC, or TC was added, removed, or reworded. The coverage and
  trace surface is unchanged.
- No production code changed.
- The source census in `tests/shared_assurance.rs` excludes `spec/reviews/**`
  from its reviewed path set, so deleting SR-004 does not disturb the expected
  live path set.
