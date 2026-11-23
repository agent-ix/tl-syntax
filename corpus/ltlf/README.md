# tl-syntax.ltlf-corpus/v1

LTLf finite-trace evidence (FR-335), read by tl-mltl, tl-parse and tl-solve
through `tl_syntax::CORPUS_DIR`.

- 18 rule cases: each of `Xs`, `Xw`, `F`, `G`, `U`, `R` on a length-1 trace and
  on a satisfying and a violating length-2 trace, with the verdict at position
  0 written by hand from ADR-044 decision 3. Traces list the true propositions
  at each position (`p` is 0, `q` is 1).
- 4 counterexample entries: CE-1 `G Xs true` (false under LTLf on every
  nonempty trace; `G[0,) F[1,1] true` is true on every lasso) and CE-2
  `F Xw false` (true under LTLf; `F[0,) G[1,1] false` is false on every lasso).
  Tying them to the literature's named formula is an open citation task.
- The depth-one population (120 formulas) is not stored; the manifest records
  its enumerator and count.

tl-syntax evaluates nothing; its integrity check (`tests/ltlf_profile.rs`)
replays the rule cases through a test-only ADR-044 reference so a mutated
field is detected.
