# TL V1 production feature increment, 2026-09-29

Implementation report for draft [PR #96](https://github.com/agent-ix/tl-syntax/pull/96),
code revision `dfbec80480453e0fe14bb29f5216453e8957409b`. This report is not a
retained evidence record or a human acceptance decision.

## TL-63 criterion disposition

| Criterion | Implemented behavior | Remaining boundary |
| --- | --- | --- |
| Seal each digest with its source identity/path | Rust projects repository-relative path-shaped Quoin source connections; aliases, traversal, symlinks, non-files, missing files, duplicated connections and substituted lookups refuse. Requirement and preservation source references must name sealed connections. | Complete Git/materialized candidate and immutable input consumption belong to TL-22; this projection does not establish them. |
| State metadata authority | Only the declared record body is projected. Top-level metadata remains authorial. AA-001 and SR-013 are source-connected; Makefile/CLAUDE descriptions are mirrors. | No local SUITE-008 observation is promoted to a Quoin attestation. |
| Check scope or state its authorial boundary | The scope list is sealed as declared, while the impact snapshot remains incomplete. Existing unfavorable gap disclosures are preserved. A caller's complete claim is overwritten with incomplete, including a nonexistent scope that omits configuration/corpus. | The admitted Quire export supplies specification locators, not a complete candidate change-footprint check. A future shared capability would belong to Quire/Engineering Assurance; no local completeness checker is substituted. |
| Consume released shared contracts | Quire CLI 0.33.0 assurance-v1, released process-module schema premises, and Quoin 0.24.1 sealing are used by `make source-grounding-record`. | Runtime package-integrity admission and the legacy default-chain migration remain open. |
| Refuse honest adverse cases | Focused Rust checks cover source substitution, aliases, missing/duplicate references, statement drift, changed specification bytes, incompatible module premise, false scope and metadata authority. | This is feature evidence only. TL-63 is open and draft; the full census/parity obligation is not completed. |

At the code revision above, the real Quire export and Make/Rust/Quoin handoff
succeeded. Quoin retained record digest
`aacdb013bb7fbc9240bb93edef11bc973e3e38de853210aaab3c1ad5c3809ba8`
in the disposable local Quoin store under `target/assurance-store`. It contains
12 source connections and an incomplete scope. Focused Rust tests and Clippy
passed. Those observations do not establish freshness or durable retention.

## Production FR status

| Requirement | Implementation status |
| --- | --- |
| tl-syntax FR-006-AC-9 | Draft path-binding behavior implemented in PR #96; FR-006 as a whole is not reaccepted by this increment. |
| tl-syntax FR-006-AC-2 | The native corpus-result transcription now has a Rust implementation used by `assurance-record`; the legacy full chain and other producer paths remain. All 22 entries from a real corpus replay matched the Python adapter, and Quoin accepted the Rust entries with no unmatched or suspect bindings. |
| tl-syntax FR-019 / TL-23 | Unimplemented complete census/classifier and Rust/shared parity. Legacy executable paths remain. |
| tl-syntax FR-015 / TL-22 | Unimplemented complete immutable candidate/configuration binding and fresh producer-result admission. TL-63 is a partial source projection only. |
| tl-syntax FR-016 / TL-21 | Unimplemented lifecycle stages, durable retention and supersession adapter. |
| tl-syntax FR-018 / TL-21 | Unimplemented authoritative human-decision admission adapter. |
| IT-001 / TL-20 | Unimplemented complete real source-readiness handoff; sealing one record is only a partial seam. |
| tl-syntax FR-017 / TL-19 | Unimplemented integrator-readiness package. |

The supplied audited baseline already contains production implementations for
front-end profiles, infinite evaluation, rewrite and R2U2. In particular,
[syntax PR #93](https://github.com/agent-ix/tl-syntax/pull/93),
[parser PR #53](https://github.com/agent-ix/tl-parse/pull/53) (FR-015..FR-017),
and [evaluator PR #99](https://github.com/agent-ix/tl-mltl/pull/99)
(FR-027..FR-034) are merged feature deliveries with their own evidence.
TL-255, TL-256 and TL-257 remain acceptance gaps. Their planned rows alone do
not establish a missing production behavior; no new independent missing feature
was established by this increment. The live FR crosswalk remains the authority
for the full multi-repository mapping.

## Dependency and ownership disposition

| Next step | Exact blocker or required work | Owner |
| --- | --- | --- |
| TL-24 | Bind admitted identities to a complete candidate; select retention, policy/event source and package contract; retain per-capability decisions. EA v0.4.1 is admitted; v0.5.0's matrix is still pending human acceptance. | tl-syntax consumer maintainer; human selections by TL release owner |
| TL-23 | Reviewed LR08 path dispositions and exhaustive local inventory are absent. Retention/package selections do not themselves block this step. | quire-research#64 owns shared policy/catalog; tl-syntax owns local enumeration and Rust replacement parity |
| TL-22 | Completed TL-23 census/parity and exact shared-executor consumer binding are absent. EA v0.4.1 contains the Rust executor but no producer-execution CLI command. The intended integration uses a separate released CLI, preserving the published library's MIT/Apache boundary. No local runner or library-license change is substituted. | tl-syntax owns consumer binding; Engineering Assurance owns the missing released CLI seam and executor semantics |
| TL-21 FR-016 | No immutable selection of durable backend, operator, retrieval policy and retention lifecycle is recorded. | Quoin contract maintainers and TL release owner/operator |
| TL-21 FR-018 | No immutable reviewer/decision policy, actor set or authoritative event-query source is selected. ix-flow's compatible release alone supplies none of those selections. | TL release owner; authoritative workflow/event-source owner |
| TL-20 | Candidate, lifecycle and decision adapters above are not implemented/admitted. | tl-syntax consumer maintainer, with Quire/Engineering Assurance/Quoin owners for shared seams |
| TL-19 | No admitted released integrator-package writer/reader contract was identified; preceding adapters are absent. | Engineering Assurance contract maintainers; tl-syntax consumer maintainer |

No TL-18 human release decision, TL-258 duplicate, verification campaign,
hardening or qualification gate was advanced.
