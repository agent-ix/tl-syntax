---
id: VO-003
title: "Checked native temporal definition"
type: value_object
relationships:
  - { target: ix://agent-ix/quire-spec-language/FR-051, type: implemented_by }
---
# [VO-003] Checked native temporal definition

## Properties

- **source** — exact native source document identity, positive revision, digest and complete source span domain.
- **subject_ref** — owner identity of one admitted whole-clause or temporal subject.
- **subject_kind** — `whole-clause-boolean` or `native-temporal`.
- **requirement_and_clause** — authored package/requirement/revision plus clause identity and clause span.
- **compiled_package** — exact admitted `quire.compiled-protocol/2` artifact identity and digest.
- **definition_and_profile** — accepted native definition identity/revision/digest, evaluation profile, definedness profile and clock profile.
- **model_and_bindings** — model closure, declaration closure, canonical declaration identity, complete binding/capture requirements and activation identity.
- **typed_tree** — finite source-bound Boolean leaf or temporal node tree with stable owner node identities, source spans, operators, inclusive intervals and operand edges.
- **checked_predicates** — every `holds(expr)` or whole-clause Boolean leaf, each binding its Boolean expected type, canonical typed-expression identity and parent subject.

The value exists only after the native owner revalidates the complete package,
source, graph, type, binding, profile and span correspondence. Equal text in a
different source, clause, model, declaration, parent subject or profile is a
different definition. No text fragment, AST supplied outside the owner,
self-asserted total flag, or downstream `BoundClause` can construct this value.
