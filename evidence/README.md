# Retained evidence

Run `scripts/collect_evidence.sh` from a clean repository root. The collector
creates a revision-and-UTC-time-scoped directory and refuses to overwrite an
existing record. It preserves command stdout and stderr separately, tool and
source identities, outcomes, limitations, and SHA-256 digests. Captured text
normalizes repeated terminal newlines to one newline so the retained PR range
remains whitespace-clean; no other output bytes are changed.

The collector emits a canonical `quire.derivation-evidence/v1` record plus
separately versioned collection-input and manifest records. Set `PGM01_SCHEMA`
to the reviewed PGM-01 Draft 7 schema and `PGM01_VALIDATOR` to its
`scripts/validate_governance.py`; collection then retains both conformance
results plus sealed validation of the exact final envelope. Because an envelope
cannot validate and digest itself without changing bytes, its own result stays
inconclusive; `collection-summary.json` records the post-seal outcome and exact
envelope digest. Missing PGM-01 gates are recorded as `skipped-unavailable`,
never as passes.

The sibling `.sha256` file uses repository-relative paths and covers every file
in the record without self-reference. Verify it from the repository root with
`sha256sum --check evidence/<record>.sha256`.

`STATIC.sha256` covers every retained evidence document outside the immutable
record directories, including review and run notes. `ANCHORS` exactly covers
that static manifest and every record manifest, so no evidence entry can be
added, removed, or changed outside the verified integrity boundary.

Candidate output informs the human decision described by MP-001. It cannot
approve, publish, validate, accredit, or certify a release or consuming project.
