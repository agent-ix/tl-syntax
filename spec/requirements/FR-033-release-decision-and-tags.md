---
id: FR-033
title: Require an attributed release decision before permanent tags
type: FR
relationships:
  - target: ix://agent-ix/tl-syntax/FR-018
    type: depends_on
  - target: ix://agent-ix/tl-syntax/FR-029
    type: depends_on
  - target: ix://agent-ix/tl-syntax/FR-030
    type: depends_on
  - target: ix://agent-ix/tl-syntax/FR-031
    type: depends_on
  - target: ix://agent-ix/tl-syntax/FR-032
    type: depends_on
---

# FR-033: Require an attributed release decision before permanent tags

## Description

When a coordinated TL release after 0.3.0 is proposed, the release process shall
require an attributed human decision for the exact four-crate candidate after
all required gates and reviews, before any tag is pushed.

## Inputs

- Exact candidate commits, configurations and proposed tag names.
- Complete FR-029 through FR-032 gate reports and required review results.
- The FR-018 decision event, accepted limitations and scoped exceptions.

## Outputs

- A release-ready or open/conditional/rejected disposition bound to the exact
  candidate set, and a one-at-a-time tag sequence when authorized.

## Behavior

Green automation never substitutes for an attributed human decision. An
exception names the specific failed gate, candidate, owner, rationale,
counterevidence and expiry, and remains conditional until an authorized
decision explicitly permits the exact release. The 0.3.0 light-gate exception
is historical only. Material changes after a decision reopen admission under
FR-018. Before pushing each tag, the process verifies that the name is unused,
the target is the accepted candidate commit, and predecessor tags and pins are
the accepted ones. A pushed tag is permanent and cannot be retargeted as a
recovery method.

## Acceptance Criteria

| ID | Criteria | Verification |
|---|---|---|
| FR-033-AC-1 | Without a current attributed FR-018 decision for the exact candidate and required gates, no tag is pushed; an expired or changed decision reopens the release. | Test (TC-179) |
| FR-033-AC-2 | Existing or mismatched tags refuse, and authorized tags follow syntax, parse, mltl, rewrite dependency order without retargeting. | Test (TC-179) |

## Dependencies

FR-018 owns decision authenticity and independence. The release gate consumes
that disposition and does not create a second approval authority.
