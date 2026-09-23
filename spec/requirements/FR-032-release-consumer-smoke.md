---
id: FR-032
title: Build a downstream consumer against all release tags
type: FR
relationships:
  - target: ix://agent-ix/tl-syntax/FR-029
    type: depends_on
  - target: ix://agent-ix/tl-syntax/FR-030
    type: depends_on
---

# FR-032: Build a downstream consumer against all release tags

## Description

Before tagging a coordinated TL release, a non-workspace `release-smoke/`
consumer shall depend on the exact four proposed tags and exercise the public
integration path at the shared MSRV and current stable Rust.

## Inputs

- Exact candidate tags and commits for tl-syntax, tl-parse, tl-mltl and
  tl-rewrite, with one locked dependency resolution.
- A representative formula, signal catalog, trace and C2PO mapping request.

## Outputs

- Build and behavioral smoke outcomes at both toolchains, bound to the
  dependency graph and candidate commits.

## Behavior

The consumer parses and round-trips the formula, rewrites it, binds the signal
catalog, evaluates the trace and maps an applicable formula to C2PO through
public APIs. It lives outside the tl-syntax workspace and has no reverse
dependency on the repository being released. Its lockfile must resolve the
four proposed revisions, with no local path substitution. A compile-only
smoke is insufficient; each named path yields an asserted result.

## Acceptance Criteria

| ID | Criteria | Verification |
|---|---|---|
| FR-032-AC-1 | The consumer uses all four exact proposed tag revisions and executes parsing, rewriting, evaluation, signal binding and C2PO mapping with asserted results. | Test (TC-177) |
| FR-032-AC-2 | The same locked consumer builds and runs at the shared MSRV and current stable; a wrong pin, absent operation or local path override fails. | Test (TC-178) |

## Dependencies

The four crate APIs supply the integration path; FR-029 fixes its exact pins.
