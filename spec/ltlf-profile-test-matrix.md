---
id: TM-048
title: "LTLf finite-trace profile test matrix"
type: TestMatrix
relationships:
  - target: ix://agent-ix/tl-syntax/MRS-001
    type: covers
---

# TM-048: LTLf finite-trace profile test matrix

Specification stage (TL-427). No test below exists yet; every row is `🚧` until
the owning build ticket (TL-428) lands it. Rows trace to criteria only; read
coverage from `quire matrix`.

Ticket acceptance mapping (TL-428, syntax half): "new profile identifier and
admission rules; existing profiles unchanged" is FR-332 and FR-334; "weak and
strong next both representable" is FR-333. The evaluation and reference-agreement
criterion is tl-mltl FR-084.

## Test Case Summary

| Test ID | Title | Type | Priority | Traces To | Status |
|---------|-------|------|----------|-----------|--------|
| TC-220 | Admit every admitted LTLf node under the new profile and edition identity | Unit | P0 | FR-332-AC-1 | 🚧 not implemented (TL-428) |
| TC-221 | Refuse each of the nine refused classes with its own typed cause and no partial formula | Unit | P0 | FR-332-AC-2, StR-045-VC-1 | 🚧 not implemented (TL-428) |
| TC-222 | Refuse missing, unknown and mismatched profile or edition identity naming the axis | Unit | P0 | FR-332-AC-3 | 🚧 not implemented (TL-428) |
| TC-223 | Keep Xs and Xw as distinct graph identities that both round-trip | Unit | P0 | FR-333-AC-1, StR-045-VC-2 | 🚧 not implemented (TL-428) |
| TC-224 | Refuse a wire next tag with no strength and offer no defaulted constructor | Unit | P0 | FR-333-AC-2, StR-045-VC-2 | 🚧 not implemented (TL-428) |
| TC-225 | Refuse StrongNext and WeakNext under every existing edition and profile | Unit | P0 | FR-333-AC-3 | 🚧 not implemented (TL-428) |
| TC-226 | Hash every pre-existing corpus file against its pre-change digest and compare existing wire bytes | Integration | P0 | FR-334-AC-1, StR-045-VC-1 | 🚧 not implemented (TL-428) |
| TC-227 | Refuse the LTLf schema string in each existing edition decoder | Unit | P0 | FR-334-AC-2 | 🚧 not implemented (TL-428) |
| TC-228 | Assert five distinct profile identifiers and four verbatim old strings | Unit | P1 | FR-334-AC-3 | 🚧 not implemented (TL-428) |
| TC-229 | Census the ltlf corpus at 18 rule cases, 4 counterexample entries and 120 enumerated formulas | Integration | P0 | FR-335-AC-1 | 🚧 not implemented (TL-428) |
| TC-230 | Detect one-field mutations of the ltlf corpus manifest | Integration | P1 | FR-335-AC-2 | 🚧 not implemented (TL-428) |
| TC-231 | Build and validate LTLf documents in the borrowed no_std build and at limit and limit plus one | Compile | P1 | NFR-048-AC-1 | 🚧 not implemented (TL-428) |
