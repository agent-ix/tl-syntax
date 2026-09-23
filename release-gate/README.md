# TL candidate graph checker

This standalone Rust tool checks the exact four-crate candidate graph before a
coordinated release. It reads each manifest, lockfile and toolchain declaration
from the named Git commit. It verifies canonical Git sources, exact package
versions and revisions on every TL edge, one resolved production revision per
crate, the common MSRV, and separately declared historical dev lanes. A clean
worktree at each commit is required before it runs all-target, all-feature
tests at the MSRV. The JSON report names every static inconsistent edge and
marks builds `not-run` when the graph is already invalid.

The syntax owner also compares every retained legacy corpus golden byte for
byte with the `previous_tag` (initially `v0.3.0`). New wire editions may add
new paths; old paths may neither disappear nor change. This is the syntax
portion of TC-176, not the full four-crate API/wire gate.

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
<candidate-set.json>`. Exit 0 means this graph/MSRV/syntax-wire checker
passed. It does not claim the corpus, API compatibility, consumer smoke, or
human release-decision gates passed. The optional `require_tags: true` plus
per-candidate `proposed_tag` enables a later read-only tag-target check; the
tool never creates or changes a tag.

A historical lane declaration names `owner`, `alias`, `package`, `revision`,
`version`, and `test_target`. It is admitted only for a matching dev edge and
the named test target is run at the MSRV. The old revision's package version is
read from that exact commit.
