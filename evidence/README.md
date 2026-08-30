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
results. Missing PGM-01 gates are recorded as `skipped-unavailable`, never as
passes.

The sibling `.sha256` file uses repository-relative paths and covers every file
in the record without self-reference. Verify it from the repository root with
`sha256sum --check evidence/<record>.sha256`.

Candidate output informs the human decision described by MP-001. It cannot
approve, publish, validate, accredit, or certify a release or consuming project.
