# Examples

Every example here compiles and passes against the crate. The first group uses the default features. Examples that read JSON need `--features serde`.

## A formula with an interval

```rust
use tl_syntax::{Formula, Interval, Node, NodeId, NodeKind, PropositionId, SemanticProfile};

let nodes = [
    Node::new(NodeKind::Proposition { proposition: PropositionId(0) }),
    Node::new(NodeKind::Future {
        interval: Interval::new(1, 3).unwrap(),
        operand: NodeId(0),
    }),
];
let formula = Formula::new(SemanticProfile::ClosedTraceV1, NodeId(1), &nodes).unwrap();
assert_eq!(formula.root(), NodeId(1));
```

This is `F[1,3] p0` under the closed-trace profile. The nodes are listed operands first, and the root is the last node.

## Refusals

Construction returns a typed error that names the first problem.

```rust
use tl_syntax::{
    Formula, FormulaError, Interval, Node, NodeId, NodeKind, PropositionId, SemanticProfile,
    TemporalFamily,
};

// An inverted interval cannot be built.
assert!(Interval::new(3, 1).is_err());

// A past operator is not admitted by a future profile.
let nodes = [
    Node::new(NodeKind::Proposition { proposition: PropositionId(0) }),
    Node::new(NodeKind::Once { interval: Interval::new(0, 2).unwrap(), operand: NodeId(0) }),
];
let error = Formula::new(SemanticProfile::ClosedTraceV1, NodeId(1), &nodes).unwrap_err();
assert_eq!(
    error,
    FormulaError::ProfileIncompatibleNode {
        profile: SemanticProfile::ClosedTraceV1,
        node: NodeId(1),
        family: TemporalFamily::Past,
    }
);

// The same table is valid under the history profile.
assert!(Formula::new(SemanticProfile::OriginCompleteHistoryV1, NodeId(1), &nodes).is_ok());
```

## Bind a formula to a signal catalog

The README example binds one proposition to a Boolean signal. A catalog can also declare numeric signals. They validate, but a proposition cannot bind to one. `tests/typed_signal_context.rs` covers each refusal, including `FormulaBindingError::MissingPropositionBinding` and `SignalCatalogError::NonBooleanPropositionBinding`.

## Read a JSON document (feature `serde`)

```rust
use tl_syntax::{FormulaDocument, SyntaxArtifactLimits};

let bytes = br#"{"schema_version":"tl-syntax.formula/v1","semantic_profile":"mltl.closed-trace/v1","root":1,"nodes":[{"kind":"proposition","proposition":1},{"kind":"future","interval":{"start":1,"end":2},"operand":0}]}"#;
let document = FormulaDocument::from_json_bytes(bytes, SyntaxArtifactLimits::default()).unwrap();

// The canonical form is exactly the input bytes.
assert_eq!(document.canonical_json_bytes().unwrap(), bytes);
// The content identity is a 64-character lowercase SHA-256.
assert_eq!(document.content_identity().unwrap().len(), 64);

// A v1 document upgrades to v2 and converts back.
assert_eq!(document.to_v2().try_to_v1().unwrap(), document);

// The strict reader refuses bytes that are valid JSON but not the canonical encoding.
let spaced = String::from_utf8(bytes.to_vec()).unwrap().replace(',', ", ");
assert!(FormulaDocument::from_json_bytes(spaced.as_bytes(), SyntaxArtifactLimits::default()).is_err());
```

The formula is the corpus fixture `corpus/formulas/short-trace-future.json`, minified. Lower the limits by setting fields of `SyntaxArtifactLimits`; a reader never accepts more than the owner maxima.

## Lower a derived operator

```rust
use tl_syntax::{
    Formula, FutureLoweringRequest, Node, NodeId, NodeKind, PropositionId, RawBounds,
    SemanticProfile, FUTURE_LOWERING_REQUEST_V1, FUTURE_OPERATORS_V1,
};

let nodes = [
    Node::new(NodeKind::Proposition { proposition: PropositionId(0) }),
    Node::new(NodeKind::Proposition { proposition: PropositionId(1) }),
];
let formula = Formula::new(SemanticProfile::ClosedTraceV1, NodeId(1), &nodes).unwrap();
let lowering = FutureLoweringRequest {
    request_identity: FUTURE_LOWERING_REQUEST_V1.as_bytes(),
    operator_profile: FUTURE_OPERATORS_V1.as_bytes(),
    kind: b"W",
    semantic_profile: SemanticProfile::ClosedTraceV1.as_str().as_bytes(),
    formula,
    left: 0,
    right: 1,
    interval: Some(RawBounds::new(0, 4)),
    operator_span: None,
    expression_span: None,
}
.lower()
.unwrap();

// p W[0,4] q appends three nodes after the two that exist: (p U q) | G p.
assert_eq!(lowering.node_ids(), [NodeId(2), NodeId(3), NodeId(4)]);
assert_eq!(lowering.root(), NodeId(4));
```

The caller appends `lowering.nodes()` to its table. `lowering.report()` records the operator, the operand and generated identities and the spans. A refused request returns a `FutureLoweringRefusal` and `code()` gives its stable name, for example `inverted_interval`.

## Replay the corpus

```sh
cargo run --quiet --example corpus_conformance --features serde -- --manifest corpus/manifest.json
cargo run --quiet --example corpus_gate --features serde
```

See [corpus.md](corpus.md).
