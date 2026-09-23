# TL candidate graph checker

This standalone Rust tool checks the exact four-crate candidate graph before a
coordinated release. It reads each manifest, lockfile and toolchain declaration
from the named Git commit. It verifies canonical Git sources, exact package
versions and revisions on every TL edge, one resolved production revision per
crate, the common MSRV, and separately declared historical dev lanes. A clean
worktree at each commit is required before it runs all-target, all-feature
tests at the MSRV. The JSON report names every static inconsistent edge and
marks builds `not-run` when the graph is already invalid.

Every crate compares retained legacy corpus bytes with its `previous_tag`
(initially `v0.3.0`). New wire editions may add paths; old paths may neither
disappear nor change; tl-mltl's retained `schemas/` are included. A separate
external replay produces formula, parser report, trace and rewrite report bytes
using the four preceding tagged crates, then admits those same bytes through
the four candidate public decoders. The checker also refuses byte-identical vendored copies
of the syntax owner's infinite corpus in any downstream repository and runs
the three owner-corpus test targets at the exact candidate revisions.

After the graph and MSRV lanes pass, a separate scratch consumer is generated
with four exact Git commit dependencies and no path overrides. It asserts
public parsing, rewriting, signal binding, trace evaluation, and C2PO mapping,
then runs with the same lockfile at the MSRV and installed `stable` toolchain.
The scratch consumer's lock is checked against all four candidate commits.
The proposed tag names are checked only after an attributed human decision;
the checker does not create or push tags.

The API lane requires `cargo-semver-checks` 0.50.0 (set `TL_SEMVER_CHECKS` to
its binary path if it is not on `PATH`). It checks every crate against its
`previous_tag` with all features and a minor-release diagnostic scan, which
exposes breaking findings even for a planned 0.3-to-0.4 release. Every unique
lint and affected symbol must have exactly one line in the candidate version's
`### API migration inventory` CHANGELOG section:
`- \`lint_id\` \`Affected::Symbol\`: Migration: actionable guidance.`
Stale entries, tool errors and a breaking finding without a breaking version
bump fail. A 0.x minor bump is a breaking version bump.

Create a JSON input outside the source tree:

```json
{
  "candidates": [
    {"name":"tl-syntax","path":"/path/to/tl-syntax","commit":"<40-hex-sha>","previous_tag":"v0.3.0"},
    {"name":"tl-parse","path":"/path/to/tl-parse","commit":"<40-hex-sha>","previous_tag":"v0.3.0"},
    {"name":"tl-mltl","path":"/path/to/tl-mltl","commit":"<40-hex-sha>","previous_tag":"v0.3.0"},
    {"name":"tl-rewrite","path":"/path/to/tl-rewrite","commit":"<40-hex-sha>","previous_tag":"v0.3.0"}
  ],
  "historical_lanes": []
}
```

Run `cargo run --manifest-path release-gate/Cargo.toml --locked --
<candidate-set.json>`. Exit 0 means the graph, API, MSRV, corpus, legacy wire
decoder and consumer lanes passed for the exact candidate commits. It does not
claim a human release decision. The optional `require_tags: true` plus
per-candidate `proposed_tag` enables a later read-only tag-target check; the
tool never creates or changes a tag.

A historical lane declaration names `owner`, `alias`, `package`, `revision`,
`version`, and `test_target`. It is admitted only for a matching dev edge and
the named test target is run at the MSRV. The old revision's package version is
read from that exact commit.
