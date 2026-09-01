# Retained evidence

Run `scripts/collect_evidence.sh` from a clean repository root. The collector
creates a revision-and-UTC-time-scoped directory and refuses to overwrite an
existing record. It preserves command stdout and stderr separately, tool and
source identities, outcomes, limitations, and SHA-256 digests. Captured text
normalizes repeated terminal newlines to one newline so the retained PR range
remains whitespace-clean; no other output bytes are changed.

The executable lock is deliberately scoped to the qualification host. The
collector runs `make ci-for-evidence` in a closed environment and checks both
launcher bytes and the dispatched Cargo/rustc version identities. Ordinary
`make ci` remains portable: it verifies retained identities against their bound
source revision but does not require another operator to possess this host's
absolute paths. The collector clears the dedicated, repository-local ignored
qualification target before running, so shared ambient Cargo artifacts are not
inputs to the claim. This qualification PR must land with a true merge commit so its
bound source revisions remain ancestors of `main`; squash and rebase merges do
not satisfy the retained-history contract. The source branch remains retained
until all downstream pins and records have moved to reachable main revisions.

To qualify on a second host, update the review-visible `tools.lock` in the
candidate revision: set `environment.home` to that host's absolute home, set
`environment.cargoTargetDir` to that checkout's `target/qualification-v1`, and
replace every tool path, executable SHA-256, and dispatched Cargo/rustc verbose-output
SHA-256 with observations from that host. Review that source diff before
running `make ci-for-evidence`; no hidden or ambient host profile is accepted.

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
