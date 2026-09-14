---
id: ENUM-001
title: "Temporal ecosystem decision vocabulary"
type: enumeration
---
# [ENUM-001] Temporal ecosystem decision vocabulary

## Values

| Value | Description |
| --- | --- |
| admitted | Structural, contract and semantic admission completed; an operation-specific usable artifact is present. |
| valued | One checked predicate has one exact final Boolean value. |
| agreement | Independently validated native and TL result views agree under equal complete premises; only `equal-final` carries a Boolean. |
| incomplete | Required work or observations are not complete; retry may succeed after additional admissible input. |
| unavailable | A selected accepted contract, producer, result or authority artifact cannot currently be reached. |
| unsupported | A well-formed contract/profile/operator/domain is outside the selected implementation contract. |
| failed | Admitted execution attempted and failed without producing a semantic value. |
| refused | Input violates a selected contract, identity, invariant or state transition. |
| conflict | Equal claimed identity has unequal content, an authority reports contradiction, or independently validated results disagree. |

Each public operation selects only the subset explicitly declared by its own
contract. No value is an alias for another. Precedence is declared per
operation, causes are retained in closed dimension order, and no non-value or
non-final disposition carries a Boolean.
