# Retained evidence

Run `scripts/collect_evidence.sh` from a clean repository root. The collector
creates a revision-and-UTC-time-scoped directory and refuses to overwrite an
existing record. It preserves command stdout and stderr separately, tool and
source identities, outcomes, limitations, and SHA-256 digests.

The collector emits a canonical `quire.derivation-evidence/v1` record plus
separately versioned collection-input and manifest records. Set `PGM01_SCHEMA`
to the reviewed PGM-01 Draft 7 schema and `PGM01_VALIDATOR` to its
`scripts/validate_governance.py`; collection then retains both conformance
results. Missing PGM-01 gates are recorded as `skipped-unavailable`, never as
passes.

Candidate output informs the human decision described by MP-001. It cannot
approve, publish, validate, accredit, or certify a release or consuming project.
