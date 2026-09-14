---
id: SR-072
title: "Scope and boundary review of the complete temporal ecosystem"
type: SpecReview
analysis: scope-boundary
scope: "DOM-001, ADR-003, all owner interfaces/requirements, bridge FR-025..FR-028, PLAN-010"
review_set: all
---
## Summary

The review allocated every shared meaning, executable wire, evaluation,
correspondence and descriptive-model responsibility to one subsystem. The
final boundary excludes qualification, transport, business action, alternate
source languages and self-certification.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | Resolved — membership/population and closure authority were incorrectly delegated from Observation to QSL; Observation now derives its own canonical authority identities while retaining producer-definition raw digests in their original domain. | QObs spec.md; QObs FR-001/FR-004; qspec FR-263/FR-264 |
| FND-002 | high | Resolved — activation state appeared in ObservationContext despite being a Protocol result object; Observation now owns only trigger/capture facts and Protocol owns the closed activation state. | VO-004; IF-002; qspec FR-231/FR-244; QProtocol FR-019 |
| FND-003 | high | Resolved — ecosystem model export was unallocated; Contract IR FR-027 owns the executable document while quire-specification remains non-runtime shared semantic authority. | IF-006/IF-007; QCI FR-027; VO-008 |
| FND-004 | medium | Resolved — the shared semantic repository and owner schemas risked becoming duplicate wire authorities; VO-008 and IF-007 now state that shared objects fix meaning while one executable owner keeps each canonical type/reader. | VO-008; IF-007; ADR-003 |
| FND-005 | medium | Resolved — Contract IR’s foundation and bridge responsibilities were inseparable at the package boundary; IF-008/FR-028 allocate a dependency-free model package and a consumer bridge package with compatibility re-exports. | IF-008; QCI FR-028 |
