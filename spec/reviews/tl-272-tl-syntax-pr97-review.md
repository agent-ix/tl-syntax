---
id: SR-114
title: "tl-syntax#97 drop dangling PGM-01 citations review"
type: SpecReview
analysis: base
scope: "agent-ix/tl-syntax@fa2f6aa5c17825474b9ee822fad8e3d83b3037a9; diff against origin/main 6aa9b11; assurance/README.md; assurance/pins.json; requirements-assurance.txt; spec/assurance/AD-002-source-readiness-boundary.md; spec/assurance/AP-001.md; spec/assurance/AP-002-progressive-source-readiness.md; spec/evidence/suites.md; spec/requirements/FR-018-require-human-source-release-decision.md; spec/requirements/StR-004-progressive-source-readiness.md; spec/reviews/SR-004-pgm01-reconciliation.md (deleted); spec/source-readiness.md; spec/spec.md; tests/shared_assurance.rs"
review_set: subset
---

# tl-syntax#97 drop dangling PGM-01 citations review

## Summary

Ticket: TL-272. PR: agent-ix/tl-syntax#97, branch `chore/drop-pgm01-citations`,
reviewed at `fa2f6aa`. Methods run in this one document, scoped to the diff
against `origin/main` (`6aa9b11`, which is also the merge base): spec-review
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
  edge is left anywhere in the tree. The PR removed all four: MRS-001,
  source-readiness, AP-001 and AP-002. The remaining `PGM-01` and `pgm01` text
  is of two kinds:
  - archival records that the repo's own census treats as archival
    (`spec/plans/**`, `spec/reviews/**`);
  - literal identifiers of a removed reader: `map_pgm01_bytes`, and the quoted
    refusal string in `assurance/README.md:55`.
- **The SR-004 deletion was ceremony only.** The file held:
  - a mapping of tl-syntax onto PGM-01-R01..R10;
  - one medium "finding" that restated the open human-review and release gates,
    which AP-001 and AA-001 already own;
  - no FR, AC or test.

  Nothing live links to it. The only mention is prose in
  `spec/reviews/SR-011-drop-legacy-evidence-code-review.md:92`, an archival
  record that lists SR-004 among the closed reviews. By that record's own rule
  it is not edited.
- **No AC lost meaning.** Every spec edit is in a Dependencies section, a
  frontmatter edge, a context diagram, or narrative. No AC row, TC, or test
  matrix row changed.
- **Two dropped clauses.** Both rewrites drop a PGM-01 obligation along with the
  citation:
  - `spec/source-readiness.md:159` drops "in the order governed by
    quire-contract-ir#4/PGM-01", which was PGM-01-R03 release order;
  - `AP-001` drops "PGM-01 classifies", which was PGM-01-R07 classification.

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
| spec/assurance/AP-001.md (relationships, classification sentence) | examined | clean |
| spec/assurance/AP-002-progressive-source-readiness.md (relationships) | examined | clean |
| spec/source-readiness.md (relationships, prerequisite DAG, sequencing bullet) | examined | clean |
| spec/assurance/AD-002-source-readiness-boundary.md (context diagram, dependency row) | examined | clean |
| spec/requirements/FR-018 Dependencies | examined | clean: authority now rests on the human-release-owner policy alone |
| spec/requirements/StR-004 Dependencies | examined | clean |
| spec/evidence/suites.md SUITE-006/007 note | examined | clean |
| assurance/README.md:56 | examined | clean |
| assurance/pins.json:22,30 | examined | FND-001 on :30 |
| requirements-assurance.txt:5 | examined | clean |
| tests/shared_assurance.rs:808 (comment) | examined | clean; the file is itself archival in `is_archival_record` |
| spec/reviews/SR-004-pgm01-reconciliation.md (deleted) | examined | ceremony only; no inbound links |
| spec/reviews/SR-011-drop-legacy-evidence-code-review.md:92 | context_only | archival prose mention of SR-004 |

## Gap analysis

Plan completion: not assessed.

- No requirement, AC, or TC was added, removed, or reworded. The coverage and
  trace surface is unchanged.
- No production code changed.
- The source census in `tests/shared_assurance.rs` excludes `spec/reviews/**`
  from its reviewed path set, so deleting SR-004 does not disturb the expected
  live path set.

## Remaining pin/digest machinery (out of this PR's scope: blocked on permission)

The PR does not touch this machinery, and it is not a must-fix for this PR:

- `assurance/pins.json` holds the release pins, a digest-pinned
  `compatibility.py`, and the known_drift and retained_evidence records.
- `assurance/change-assurance.json`
- `assurance/README.md`, the pin and assurance narrative.
- `scripts/check_shared_pins.py`
- `scripts/assurance_chain.py`
- `requirements-assurance.txt`, the separate assurance virtual environment.
- `Makefile` targets `assurance-env`, `assurance-inputs`, `pins`,
  `mutation-probes`, `assurance-chain`, `assurance` and `assurance-record`.
  `test` depends on `assurance-inputs`.
- `.github/workflows/ci.yml` uses the pins and assurance steps.
- `tests/shared_assurance.rs`, the census and the shared-assurance adapter
  tests.
- Corpus checksum files:
  - `corpus/SHA256SUMS`
  - `corpus/future-operators/SHA256SUMS`
  - `corpus/infinite-trace/SHA256SUMS`
  - `corpus/past-history/SHA256SUMS`
  - `fuzz/corpus/infinite_wire_decode/SHA256SUMS`

  These are read by `tests/future_operator_corpus.rs`,
  `tests/infinite_trace_corpus.rs` and `tests/infinite_fuzz_seeds.rs`. They are
  content digests over this repo's own corpus, so they may be load-bearing
  rather than ceremony. Decide that before removing them.
