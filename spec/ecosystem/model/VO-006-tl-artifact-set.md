---
id: VO-006
title: "Canonical TL artifact set"
type: value_object
relationships:
  - { target: ix://agent-ix/tl-syntax/FR-014, type: implemented_by }
  - { target: ix://agent-ix/tl-parse/FR-009, type: implemented_by }
  - { target: ix://agent-ix/tl-mltl/FR-018, type: implemented_by }
  - { target: ix://agent-ix/tl-rewrite/FR-010, type: implemented_by }
---
# [VO-006] Canonical TL artifact set

## Properties

- **predicate artifacts** — one Boolean signal catalog and bijective proposition map derived from a sorted distinct checked-predicate population.
- **formula** — canonical typed TL graph under an exact formula and semantic-profile selection.
- **history_or_trace** — complete origin-bound past history or future trace under an exact clock and observation population.
- **valuation_set** — rectangular position-by-proposition population retaining every explicit true and false owner-backed cell.
- **request** — exact evaluator command binding the formula and history/trace artifacts.
- **report** — typed evaluator output retaining truth, closure/progress, completeness, support, limits and correction relation under the selected evaluator contract.
- **identity_set** — a separate content identity for every artifact, computed over its exact canonical bytes and explicit domain.

Each member is independently versioned and strict-read by its owning TL crate.
No identity is inferred from another. A consumer cannot substitute display
text, a parser-accepted string, an omitted false proposition, or a copied wire
struct for an admitted artifact. Future, past and mixed semantic domains remain
closed by their selected profiles.
