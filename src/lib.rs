#![no_std]
#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "alloc")]
mod bounded_string;
pub mod aggregation;
pub mod assume_guarantee;
mod context;
pub mod contracts;
#[cfg(all(test, feature = "serde"))]
mod error_variants;
pub mod formula;
mod future;
pub mod numeric;
pub mod signal;

#[cfg(feature = "alloc")]
mod context_document;

pub use context::{
    RequirementContext, RequirementContextError, RequirementContextField,
    MAX_REQUIREMENT_CONTEXT_FIELD_BYTES,
};
#[cfg(feature = "alloc")]
pub use context_document::{RequirementContextDocument, RequirementContextSchemaVersion};
#[cfg(feature = "serde")]
pub use contracts::identity::CONTENT_IDENTITY_DOMAIN_V1;
pub use contracts::limits::SyntaxArtifactLimits;
#[cfg(feature = "serde")]
pub use contracts::manifest::{
    validate_past_profile_implementation, PastProfileManifestError, PAST_PROFILE_IMPLEMENTATION_V1,
    PAST_PROFILE_MANIFEST_MAX_BYTES,
};
#[cfg(feature = "alloc")]
pub use formula::document::{
    FormulaConversionError, FormulaDocument, FormulaSchemaVersion, SemanticFormulaDocument,
};
#[cfg(feature = "serde")]
pub use formula::document::{
    StrictDocumentReadError, FORMULA_V1_SCHEMA, FORMULA_V1_SCHEMA_BYTES, FORMULA_V1_SCHEMA_SHA256,
    FORMULA_V2_SCHEMA, FORMULA_V2_SCHEMA_BYTES, FORMULA_V2_SCHEMA_SHA256, MAX_TL_DOCUMENT_BYTES,
    MAX_TL_DOCUMENT_DEPTH,
};
pub use formula::graph::{
    Formula, FormulaError, Interval, IntervalError, LtlfRefusal, Node, NodeId, NodeKind,
    OperatorArity, PastOperatorKind, PropositionId, SemanticProfile, SourceSpan, SourceSpanError,
    TemporalFamily, LTLF_OPERATORS_V1, MAX_FORMULA_DOCUMENT_DEPTH, MAX_FORMULA_DOCUMENT_NODES,
    PAST_OPERATORS_V1,
};
pub use formula::ltlf::{
    select_ltlf_identity, validate_ltlf, LtlfFormulaError, LtlfIdentityError, FORMULA_LTLF_V1,
    LTLF_FINITE_TRACE_PROFILE,
};
#[cfg(feature = "alloc")]
pub use formula::ltlf::LtlfFormulaDocument;
#[cfg(feature = "serde")]
pub use formula::ltlf::{LtlfReadError, FORMULA_LTLF_V1_SCHEMA, FORMULA_LTLF_V1_SCHEMA_BYTES};
#[cfg(feature = "alloc")]
pub use formula::infinite::InfiniteFormulaDocument;
pub use formula::infinite::{
    lower_infinite_future, select_infinite_profile, InfiniteClock, InfiniteClockError,
    InfiniteFormula, InfiniteFormulaError, InfiniteFormulaSchemaVersion, InfiniteNode,
    InfiniteNodeKind, InfiniteProfileError, TemporalInterval, TemporalIntervalParseError,
    UnboundedInterval, EVENT_POSITION_CLOCK, FORMULA_UNBOUNDED_V1, INFINITE_TRACE_PROFILE,
    LIVENESS_CAPABILITY_V1,
};
#[cfg(feature = "serde")]
pub use formula::infinite::{
    FORMULA_UNBOUNDED_V1_SCHEMA, FORMULA_UNBOUNDED_V1_SCHEMA_BYTES,
    FORMULA_UNBOUNDED_V1_SCHEMA_SHA256,
};
#[cfg(feature = "alloc")]
pub use formula::liveness::{
    settle_liveness, LivenessBackend, LivenessDisposition, LivenessSettlement, LivenessSubject,
    LivenessSubjectKind,
};
#[cfg(feature = "serde")]
pub use formula::trace::FairnessReadError;
#[cfg(feature = "alloc")]
pub use formula::trace::{
    FairnessPremisesDocument, FairnessPremisesError, LassoTraceDocument, LassoTraceError,
    PartialValuation, PartialValuationError, PartialValue, TraceObservation, ValuationEntry,
    FAIRNESS_PREMISES_V1, LASSO_TRACE_V1, PARTIAL_VALUATION_V1,
};
pub use future::{
    FutureKind, FutureLowering, FutureLoweringAxis, FutureLoweringOperand, FutureLoweringRefusal,
    FutureLoweringReport, FutureLoweringRequest, FutureLoweringSpanRole, RawBounds,
    UnsupportedFutureKind, FUTURE_LOWERING_NODE_CHARGE, FUTURE_LOWERING_REFUSAL_V1,
    FUTURE_LOWERING_REPORT_V1, FUTURE_LOWERING_REQUEST_V1, FUTURE_OPERATORS_V1,
    MAX_FUTURE_LOWERING_IDENTITY_BYTES, MAX_FUTURE_LOWERING_KIND_BYTES,
};
pub use signal::catalog::{
    BoundFormula, FixedDecimalSignalDomain, FormulaBindingError, IntegerSignalDomain,
    PropositionBinding, SignalCatalog, SignalCatalogError, SignalDeclaration, SignalDirection,
    SignalDomain, SignalDomainError, SignalId, SignalIter, SignalPartitionError,
    MAX_SIGNAL_CATALOG_BINDINGS, MAX_SIGNAL_CATALOG_SIGNALS, MAX_SIGNAL_NAME_BYTES,
};
#[cfg(feature = "alloc")]
pub use signal::catalog::SignalPartition;
#[cfg(feature = "alloc")]
pub use signal::document::{
    OwnedSignalDeclaration, PropositionEntry, PropositionMapDocument, PropositionMapError,
    PropositionMapSchemaVersion, SignalCatalogDocument, SignalCatalogSchemaVersion,
};
#[cfg(feature = "serde")]
pub use signal::document::{
    MAX_PROPOSITION_MAP_ENTRIES, PROPOSITION_MAP_V1_SCHEMA, PROPOSITION_MAP_V1_SCHEMA_BYTES,
    PROPOSITION_MAP_V1_SCHEMA_SHA256, SIGNAL_CATALOG_V1_SCHEMA, SIGNAL_CATALOG_V1_SCHEMA_BYTES,
    SIGNAL_CATALOG_V1_SCHEMA_SHA256, SIGNAL_CATALOG_V2_SCHEMA, SIGNAL_CATALOG_V2_SCHEMA_BYTES,
};

pub use numeric::expr::{
    ExprId, ExprNode, ExprSlot, ExprType, ExpressionArena, ExpressionError, ExpressionLimit,
    MAX_DECIMAL_SCALE, MAX_EXPRESSION_DEPTH, MAX_EXPRESSION_NODES,
};
pub use numeric::predicate::{
    BoundPredicateFormula, ComparisonOperator, Predicate, PredicateBindingError, PredicateTable,
    PredicateTableError, PropositionSource, MAX_PREDICATES,
};
#[cfg(feature = "alloc")]
pub use numeric::document::{
    PredicateTableDocument, PredicateTableReadError, PredicateTableSchemaVersion,
    PREDICATE_TABLE_V1,
};
#[cfg(feature = "serde")]
pub use numeric::document::{PREDICATE_TABLE_V1_SCHEMA, PREDICATE_TABLE_V1_SCHEMA_BYTES};

pub use aggregation::{
    expand_quantifier_into, quantifier_node_count, AggregationLimit, AggregationRefusal,
    ExpansionCounts, Quantifier, SetAggregationError, TemplateNode, MAX_AGGREGATION_DEPTH,
    MAX_AGGREGATION_EXPRESSIONS, MAX_AGGREGATION_NODES, MAX_AGGREGATION_PREDICATES,
    MAX_SET_MEMBERS,
};
#[cfg(feature = "alloc")]
pub use aggregation::document::{
    Aggregation, AggregationTemplate, ExpandedAggregation, MemberSet, SetAggregationDocument,
    SetAggregationReadError, SetAggregationSchemaVersion, SET_AGGREGATION_V1,
};
#[cfg(feature = "serde")]
pub use aggregation::document::{SET_AGGREGATION_V1_SCHEMA, SET_AGGREGATION_V1_SCHEMA_BYTES};

pub use assume_guarantee::{
    assumption_prefix_verdict, contract_obligation_node, contract_outcome, settled_positions,
    AssumptionPrefixVerdict, ContractAssumptionError, ContractObligationError, ContractOutcome,
    ContractStatus, ContractTruth, MAX_CONTRACTS,
};
#[cfg(feature = "alloc")]
pub use assume_guarantee::{check_assumption_inputs, lower_contract_obligation};
#[cfg(feature = "serde")]
pub use assume_guarantee::document::{
    Contract, ContractGraph, ContractSetDocument, ContractSetError, ContractSetReadError,
    ContractView, CONTRACT_SET_V1, CONTRACT_SET_V1_SCHEMA, CONTRACT_SET_V1_SCHEMA_BYTES,
};

/// Stable revision identifier for the checked-in shared temporal corpus.
pub const CORPUS_REVISION: &str = "tl-syntax-corpus/v1";

/// Stable identity of the immutable paired past/history corpus.
pub const PAST_HISTORY_CORPUS_V1: &str = "tl-syntax.past-history-corpus/v1";

/// Filesystem path to this crate's own `corpus/` directory, resolved at this
/// crate's compile time via its own `CARGO_MANIFEST_DIR`. A git or path
/// dependency's checkout carries the full repository tree, so a dependent
/// crate can join a corpus-relative path onto this constant and read the
/// shared corpus directly, instead of vendoring a copy of it.
pub const CORPUS_DIR: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/corpus");
