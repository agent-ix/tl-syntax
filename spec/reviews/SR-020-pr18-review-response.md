---
id: SR-020
title: Author response to PR 18 independent review
type: SpecReview
analysis: code-review
scope: "agent-ix/tl-syntax#17 remediation candidate 400bba5; independent PR #18 review at 99dde202; FR-006-AC-6/AC-7; TC-026/TC-034"
review_set: all
relationships:
  - target: ix://agent-ix/tl-syntax/FR-006
    type: reviews
  - target: ix://agent-ix/tl-syntax/PLAN-004
    type: references
---

# SR-020: Author response to PR #18 independent review

## Summary

The independent review at `99dde202` reported one high, eight medium, and eleven
low findings. Candidate `400bba5` remediates every high and medium finding,
remediates nine low findings, and routes the two shared-contract lows to #16.
The review also identified the missing qualification-integrity ownership
requirement, now tracked as #19. This author response grants no closure or merge
authority; a new independent exact-head review remains mandatory.

## Findings

| ID | Severity | Summary | Refs | Escape Cause |
|---|---|---|---|---|
| FND-2001 | high | TS18-01: the 42-path allowlist excluded live corpus, configuration, root, workflow, and extensionless paths. | FR-006-AC-7, TC-034 | wrong-requirement |
| FND-2002 | medium | TS18-02: the Git ceiling had no control that needed it. | TC-034 | correct-requirement-no-evidence |
| FND-2003 | medium | TS18-03: the untracked scan producer was tested but its forbidden-reference consumer was not. | TC-026, TC-034 | correct-requirement-no-evidence |
| FND-2004 | medium | TS18-04: name and extension filters omitted valid live paths and higher-precedence Make inputs. | TC-026, TC-034 | implementation-bug-despite-evidence |
| FND-2005 | low | TS18-05: the aggregate population assertion could fire only when two literals disagreed and its diagnostic blamed the tree. | TC-034 | implementation-bug-despite-evidence |
| FND-2006 | low | TS18-06: the root conjunction could not distinguish missing from untracked. | TC-034 | implementation-bug-despite-evidence |
| FND-2007 | low | TS18-07: any nonzero Git failure satisfied the non-repository control. | TC-034 | correct-requirement-no-evidence |
| FND-2008 | low | TS18-08: scratch controls were not process-scoped and leaked on assertion failure. | TC-034 | implementation-bug-despite-evidence |
| FND-2009 | low | TS18-09: within-area path substitution was not detected or disclosed. | TC-034 | missing-requirement |
| FND-2010 | medium | TS18-R01: the bundle implied final-head verification remained while the PR record reported it complete. | PLAN-004 | correct-requirement-no-evidence |
| FND-2011 | medium | TS18-R02: NFR-002 retained a blanket succession claim and left three retired clauses undescribed. | NFR-002 | wrong-requirement |
| FND-2012 | medium | TS18-R03: FR-006-AC-6 mirrored one test mechanism and bundled unrelated absence and census obligations. | FR-006-AC-6, FR-006-AC-7 | wrong-requirement |
| FND-2013 | medium | TS18-R04: a sealed preservation statement claimed a local test had run without an attestation or declared suite. | SUR-001 | correct-requirement-no-evidence |
| FND-2014 | medium | TS18-R05: SR-017 renumbered external findings and self-granted their closure without attribution. | SR-017 | implementation-bug-despite-evidence |
| FND-2015 | low | TS18-R06: author reviews did not make their reviewed revisions and exact-head authority boundary fully explicit. | SR-017, SR-018, SR-019 | missing-requirement |
| FND-2016 | low | TS18-R07: AA-001 and SR-013 remain outside the authoritative sealed source set. | agent-ix/tl-syntax#16 | missing-requirement |
| FND-2017 | low | TS18-R08: Git was required by TC-026 but named by no verification suite. | SUR-001 | missing-requirement |
| FND-2018 | low | TS18-R09: duplicated declaration statements have no shared-contract equality validation. | agent-ix/tl-syntax#16 | correct-requirement-no-evidence |
| FND-2019 | low | TS18-R10: FR-006-AC-6 claimed wider source coverage than the allowlist implemented. | FR-006-AC-6, FR-006-AC-7 | wrong-requirement |
| FND-2020 | low | TS18-R11: the main-branch SR and PLAN number gaps did not explain their concurrent-branch reservation. | SR-017 | missing-requirement |

## Dispositions

| Finding | Disposition | Evidence |
|---|---|---|
| FND-2001 | **AUTHOR REMEDIATED** | All 66 non-archival tracked paths form the exact reviewed set. The only exclusions are review/plan records, this declaring test, and `spec/.gitkeep`; any other new path is live regardless of name, extension, or area. |
| FND-2002 | **AUTHOR REMEDIATED** | The negative directory is now inside the real repository. Removing the ceiling lets Git ascend and turns the refusal control red. |
| FND-2003 | **AUTHOR REMEDIATED** | The ordinary untracked fixture carries a forbidden name and the production scan consumer must reject that exact path/name. The live-tree untracked delta must be empty and names every offender. |
| FND-2004 | **AUTHOR REMEDIATED** | There is no name/extension allowlist. Corpus, `.agent`, CODEOWNERS, locks, licenses, and gate configurations are in the exact set; `GNUmakefile` and `makefile` are explicitly refused as precedence overrides. |
| FND-2005 | **AUTHOR REMEDIATED** | The redundant total literal was removed. Exact expected/observed path sets name the tree delta directly. |
| FND-2006 | **AUTHOR REMEDIATED** | Every required root is an exact expected tracked path, so the set comparison distinguishes its full path without a disk/tracked conjunction. |
| FND-2007 | **AUTHOR REMEDIATED** | The control requires both the census refusal and Git's `not a git repository` diagnosis. |
| FND-2008 | **AUTHOR REMEDIATED** | Process-scoped scratch directories remove themselves through `Drop`, including unwind paths. |
| FND-2009 | **AUTHOR REMEDIATED** | Exact path-set equality detects same-area substitutions; the independent area map still identifies cross-area changes. |
| FND-2010 | **AUTHOR REMEDIATED** | PLAN-004 now states that exact final-head execution belongs to the immutable PR record because a commit cannot contain its own final digest. The implementation-candidate run is retained below. |
| FND-2011 | **AUTHOR REMEDIATED** | NFR-002 now enumerates all five retired clauses, their narrow successors or explicit absence, and removes the Dependencies blanket claim. |
| FND-2012 | **AUTHOR REMEDIATED** | AC-6 owns deleted-machinery absence through TC-026. New AC-7 owns behaviorally stated source-partition integrity through TC-034 without prescribing a CLI implementation. |
| FND-2013 | **AUTHOR REMEDIATED** | The unattested preservation assertion was removed. SUITE-008 names the local test command and states explicitly that its result is not a Quoin attestation. |
| FND-2014 | **AUTHOR REMEDIATED** | SR-017 maps every TS14R3 identifier, labels itself an author review at `130c521`, replaces self-granted `FIXED` with author-remediation states, and reserves closure for an independent reviewer. |
| FND-2015 | **AUTHOR REMEDIATED** | SR-017 names `130c521`; SR-018/SR-019 already name `1a00573`; this response names `400bba5`; only the PR review may clear the final head. |
| FND-2016 | **DEFERRED** | Existing #16 owns authoritative source/path/scope binding. The PR #18 case was added there; no local envelope or parser is introduced here. |
| FND-2017 | **AUTHOR REMEDIATED** | SUITE-008 names the exact test command and Git dependency, including refusal outside a repository boundary. |
| FND-2018 | **DEFERRED** | Changed AC-6/AC-7 declaration text matches the specification text exactly; generic correspondence enforcement was added to #16 for shared-contract implementation. |
| FND-2019 | **AUTHOR REMEDIATED** | AC-6 is narrowed and the complete name-independent path partition is separately specified by AC-7 and exercised by TC-034. |
| FND-2020 | **AUTHOR REMEDIATED** | SR-017 records that SR-014 through SR-016 and PLAN-003 were already reserved on concurrent issue #15. |

## Verification observed

Focused TC-026 and TC-034 pass. Strict Quire validation reports 57/57 documents
grammar-clean at the implementation candidate, with all six FR-006 criteria and
all 25 matrix cases backed. Full local
`make ci CARGO_TARGET_DIR=target/cargo-review` passed outside the known Node
`execFileSync` process sandbox at exact candidate
`400bba5e4548be75426cb295f58b447fcf3ceb80`: 7/7 shared-assurance tests, MSRV,
corpus, supply-chain, rustdoc, four mutation probes, and the complete Quoin chain
all passed. Hosted CI was not dispatched.

## Residual gaps

- #16 owns authoritative declaration source/path/scope binding and generic
  requirement-statement correspondence.
- #19 owns a dedicated qualification-integrity requirement so surviving and
  intentionally unclaimed shared-assurance obligations have one explicit home.
- The initial sandboxed full gate failed only because Quoin's Node child could
  not spawn the installed Quire CLI. The identical local gate passed outside
  that process sandbox; no project assertion was weakened to hide it.

## Conclusion

No high or medium review finding remains open in the author remediation. The
new exact head still requires a full local gate and independent review before
merge. Hosted CI remains manual-only.
