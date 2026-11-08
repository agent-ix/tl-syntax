# Boundary corpus (`boundary-v1`)

`cases.json` is a hand-authored corpus of positive, negative and boundary cases
for every temporal operator x interval-shape cell (point `[0,0]`, `[0,b]`,
`[a,a]`, `[a,b]` and the `u32::MAX` extremes), the Boolean connectives and
`StrongPrevious`, plus named edge conditions: empty trace, horizon equal to the
prefix length, nested windows that only just fit, and windows reaching before
the origin.

Each case records `profile`, `operator`, `shape`, `kind`, `edges`, a formula
tree, a `trace` (instants listing the numeric ids of true propositions), the
evaluation position `at`, the `expected` result (`true`, `false`, `"pending"` or
`"refused"`), a `rationale` and, for the online-prefix profile, `closed`.
`checkedBy`, when present, names the only evaluator that may replay the case
(for example `["production"]` where an independent oracle abstains by design).

Expected results are derived by hand from the profile definitions. No evaluator
produces them. tl-syntax checks that every formula is a valid graph under the
case's profile and that every cell keeps its three cases; evaluators replay the
verdicts (tl-mltl against `tl-oracle` and its own evaluators).

Formula tree shape: `{"op":"until","interval":[a,b],"left":..,"right":..}`,
`{"op":"future","interval":[a,b],"arg":..}`, `{"op":"not","arg":..}`,
`{"op":"prop","id":0}`, `{"op":"true"}`, `{"op":"false"}`.
