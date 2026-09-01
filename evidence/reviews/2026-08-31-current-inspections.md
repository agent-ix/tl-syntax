# tl-syntax v0.1 current inspection record

Source subject: `731dcf2881ab4992e79fa4ee03dc15f6f07a5d36`

Provenance: performed by Codex on 2026-08-31 while remediating the exact-head
pull-request review. This agent-produced inspection does not replace the
independent human review or source-release decision required by AP-001.

| Obligation | Verdict | Evidence and limitation |
|---|---|---|
| FR-005-AC-3 | Pass | The current corpus manifest, schemas, fixtures, and checksum census were inspected together and are exercised by the independently derived corpus validator. |
| NFR-002-AC-2 | Pass | Every current corpus document declares the supported v1 schema/profile and the complete manifest is covered by TC-014. |
| NFR-001-M-1 | Pass | The current default dependency check requires the graph to contain only tl-syntax. |
| NFR-002-M-2 | Pass | Current formula and proposition-map kinds remain closed/versioned and retain unknown-field/version regressions. |
| StR-001-VC-2 | Pass | Borrowed validated formulas expose slices and require no owned collection; the no-alloc feature boundary is compiled directly. |
| StR-002-VC-2 | Pass | The current corpus manifest names its stable revision and every fixture identity. |

The 2026-08-30 inspection and the a857efc remediation note are historical and
superseded for current-source discharge by this record.
