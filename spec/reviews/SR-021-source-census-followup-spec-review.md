---
id: SR-021
title: Composite review of source-census reproducibility follow-ups
type: SpecReview
analysis: base
scope: "agent-ix/tl-syntax#20 author specification candidate; TS18B-01 through TS18B-05; FR-006-AC-7/AC-8; TC-034/TC-035"
review_set: all
relationships:
  - target: ix://agent-ix/tl-syntax/FR-006
    type: reviews
  - target: ix://agent-ix/tl-syntax/PLAN-005
    type: references
---

# SR-021: Composite review of source-census reproducibility follow-ups

## Summary

This author-performed pre-implementation composite review translates the five
nonblocking findings from the independent PR #18 re-review into explicit
source-census behavior. It grants no closure to those findings; only a later
independent exact-head review may do that. The change remains inside the
existing Rust integration test and adds no runner, collector, Make parser,
evidence envelope, identity registry, retention layer, or shared-contract copy.

## Findings

| ID | Severity | Summary | Refs | Escape Cause |
|---|---|---|---|---|
| FND-2101 | low | TS18B-01: exact path equality aborts before the per-area assertion, making the latter unable to supply its claimed coarse diagnostic. | FR-006-AC-7, TC-034 | implementation-bug-despite-evidence |
| FND-2102 | low | TS18B-02: UTF-8 decoding turns a tracked binary input into a read failure before the forbidden-byte scan can evaluate it. | FR-006-AC-8, TC-035 | missing-requirement |
| FND-2103 | low | TS18B-03: the gate's clean-untracked requirement is stronger than the criterion's former “reported separately” wording. | FR-006-AC-7, TC-034 | wrong-requirement |
| FND-2104 | low | TS18B-04: `--exclude-standard` imports mutable `core.excludesFile` and `.git/info/exclude` state. | FR-006-AC-8, TC-035 | missing-requirement |
| FND-2105 | low | TS18B-05: expected `catch_unwind` controls print panic-hook noise on passing runs. | FR-006-AC-7, TC-034 | implementation-bug-despite-evidence |

## Dispositions

| Finding | Disposition | Evidence |
|---|---|---|
| FND-2101 | **SPECIFIED; EXTERNAL CLEARANCE REQUIRED** | TC-034 checks the independently authored area map first, then exact paths. |
| FND-2102 | **SPECIFIED; EXTERNAL CLEARANCE REQUIRED** | FR-006-AC-8 requires byte-level search with a benign binary control and an embedded-identity refusal. |
| FND-2103 | **SPECIFIED; EXTERNAL CLEARANCE REQUIRED** | FR-006-AC-7 now states scan, report, and refusal explicitly and explains why. |
| FND-2104 | **SPECIFIED; EXTERNAL CLEARANCE REQUIRED** | Only index-matching tracked `.gitignore` rules define generated exclusions; untracked/modified policy and both external sources receive falsifying controls. |
| FND-2105 | **SPECIFIED; EXTERNAL CLEARANCE REQUIRED** | Fallible helpers return explicit errors, so controls inspect refusals without intercepting panics. |

## Composite analysis

- **Dependency:** The scope depends only on landed PR #18 and does not alter the
  public tl-syntax API proposed separately by issue #15.
- **Risk and failure domain:** The principal risk is false omission: a machine
  ignore rule or binary decode failure can prevent a forbidden identity from
  reaching its consumer. The specification therefore constrains both the
  producer set and the byte-level read site.
- **Falsifiability:** A scratch repository supplies a tracked non-UTF-8 file,
  toggles an embedded forbidden byte sequence, and names ordinary untracked
  paths through both `core.excludesFile` and `.git/info/exclude`, then mutates
  tracked and untracked `.gitignore` policy. Each case has an accepted
  neighboring control.
- **Integrity:** Exact paths remain the final authority. The area map is checked
  first only to make its diagnostic reachable; it cannot replace exact
  equality or admit a within-area substitution.
- **Evidence boundary:** TC-034 and TC-035 are local native tests. They produce
  no evidence envelope and do not claim that Quoin attests their execution.
- **EARS and scope:** The new obligations name observable inclusion, byte-search,
  refusal, and diagnostic ordering behavior. They do not qualify Git or expand
  FR-006 into a test-runner requirement.

## Conclusion

The specification is bounded and falsifiable enough to plan. This author review
grants no merge authority; independent exact-head clearance remains required.
