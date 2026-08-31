# tl-syntax v0.1 remediation inspection record

Source subject: `a857efcc28a13e5b1159ea343b6292a0c52c14d8`

Provenance: performed by Codex on 2026-08-31 in response to the pull-request
review of the preceding candidate. This is agent-produced evidence and does not
replace the independent human review or source-release decision required by
AP-001.

| Review obligation | Verdict | Evidence and limitation |
|---|---|---|
| Mandatory-command failure propagation | Pass | The checked Make recipes name their tools directly; the collector removes ambient tool, Make, and Python-optimization variables. Static and expanded-recipe inspection rejects ignore controls, shell false-success operators, ignored Rust tests including `cfg_attr(ignore)`, and every synthetic command-position failure. |
| Policy executable exit contracts | Pass | The Rust evidence-contract test executes a failing discovered policy test; Python behavior tests execute the finalizer and evidence-tree verifier against negative fixtures. |
| Closed wire documents | Pass | TC-011 rejects unknown fields at the formula-document, proposition-map-document, node, and proposition-entry boundaries. |
| Traceability method census | Pass | Verification cells are limited to `Inspection` or catalogued `Test (TC-...)` forms; empty and uncatalogued methods have negative fixtures. The exact source reports 52/52 traceability bindings. |
| Evidence result and parameter derivation | Pass | Finalization rederives the retained result from status/output artifacts and rederives the parameter digest from the named source revision. The mutable anchor file is excluded from the parameter identity. |
| Formula construction bound | Pass | FR-004 and TC-020 bind both JSON decoding and owned programmatic construction to the 100,000-node limit with the typed document error. |
| Public error evolution | Pass | The four public validation error types are marked `non_exhaustive`. |

The retained collection
`evidence/tl-syntax-v01-a857efcc28a1-20260831T193914Z` contains twelve passing
local outcomes. Its PGM-01 result remains `inconclusive` by design because an
exact finalized envelope cannot self-attest and independent review is pending.
