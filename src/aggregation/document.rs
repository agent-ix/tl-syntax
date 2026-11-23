//! Owned `tl-syntax.set-aggregation/v1` document, validation and the
//! deterministic bounded expansion (FR-306, FR-307).
//!
//! Dependency rule: a template expression node is member-dependent when it
//! reaches a `slot_signal`; a predicate when either root is dependent; a
//! template formula node when it is a slot or inner leaf, a proposition
//! defined by a dependent predicate, or reaches one. Member-independent
//! nodes, predicates and expressions of every template are written once, in
//! a shared prefix; dependent ones once per instance, in (member, template)
//! order. Fresh proposition identities go only to dependent predicates.

use alloc::{boxed::Box, string::String, vec, vec::Vec};
use core::fmt;

use super::{
    expand_quantifier_into, quantifier_node_count, AggregationLimit, AggregationRefusal,
    ExpansionCounts, Quantifier, SetAggregationError, TemplateNode, MAX_AGGREGATION_DEPTH,
    MAX_AGGREGATION_EXPRESSIONS, MAX_AGGREGATION_NODES, MAX_AGGREGATION_PREDICATES,
    MAX_SET_MEMBERS,
};
use crate::{
    numeric::expr::{catalog_signal_type, type_node, ExprId, ExprNode, ExprType},
    ExpressionError, Node, NodeId, NodeKind, Predicate, PredicateTableError, PropositionId,
    SignalCatalog, SignalDomain, SignalId, MAX_EXPRESSION_DEPTH, MAX_SIGNAL_NAME_BYTES,
};
#[cfg(feature = "serde")]
use crate::{
    contracts::{
        identity::canonical_json,
        reader::{read_strict_document, StrictDocument, StrictDocumentReadError},
    },
    SyntaxArtifactLimits,
};

/// Stable identity of the set-aggregation edition.
pub const SET_AGGREGATION_V1: &str = "tl-syntax.set-aggregation/v1";

/// Exact checked-in Draft 7 schema text for `tl-syntax.set-aggregation/v1`.
/// No digest constant exists (ADR-019 register item 1).
#[cfg(feature = "serde")]
pub const SET_AGGREGATION_V1_SCHEMA: &str =
    include_str!("../../spec/set-aggregation-v1.schema.json");

/// Exact checked-in Draft 7 schema bytes for `tl-syntax.set-aggregation/v1`.
#[cfg(feature = "serde")]
pub const SET_AGGREGATION_V1_SCHEMA_BYTES: &[u8] =
    include_bytes!("../../spec/set-aggregation-v1.schema.json");

/// Version of the serialized aggregation document.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum SetAggregationSchemaVersion {
    /// First set-aggregation edition.
    #[cfg_attr(feature = "serde", serde(rename = "tl-syntax.set-aggregation/v1"))]
    V1,
}

/// One named, ordered member set of catalog signals.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(deny_unknown_fields))]
pub struct MemberSet {
    /// Unique set name, 1..=255 UTF-8 bytes.
    pub name: String,
    /// Members in document order; may be explicitly empty.
    pub members: Vec<SignalId>,
}

/// A quantified template: a set, a quantifier and the template body.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(deny_unknown_fields))]
pub struct Aggregation {
    /// Name of the member set.
    pub set: String,
    /// The quantifier.
    pub quantifier: Quantifier,
    /// The template instantiated once per member.
    pub template: AggregationTemplate,
}

/// A template body: formula nodes, its own expression arena and predicate
/// definitions, and at most one nested quantified template.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(deny_unknown_fields))]
pub struct AggregationTemplate {
    /// Template formula nodes; operands precede their user.
    pub nodes: Vec<TemplateNode>,
    /// Template root.
    pub root: NodeId,
    /// Template expression arena (may contain `slot_signal` leaves).
    pub expressions: Vec<ExprNode>,
    /// Template predicates, strictly increasing by proposition. A dependent
    /// predicate's proposition is a template-local placeholder.
    pub predicates: Vec<Predicate>,
    /// The nested quantified template referenced by the `inner` leaf.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub inner: Option<Box<Aggregation>>,
}

// SPEC-GAP: FR-307 names one `root`; a document holds several quantified
// templates (FR-307 "Identity precedence"), so the expansion returns one root
// per top-level aggregation in document order. FR-307 "Outputs" to update.
// SPEC-GAP: member-independent template items are written once in a shared
// prefix and counted once; FR-307's counts assume this reading.
/// The result of an expansion: ordinary nodes, predicates and expressions.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ExpandedAggregation {
    /// Formula nodes in topological order.
    pub nodes: Vec<Node>,
    /// Predicates, strictly increasing by proposition.
    pub predicates: Vec<Predicate>,
    /// Expression arena referenced by the predicates.
    pub expressions: Vec<ExprNode>,
    /// One root per top-level aggregation, in source order.
    pub roots: Vec<NodeId>,
    /// Exact populations.
    pub report: ExpansionCounts,
}

/// Per-template dependency flags.
#[derive(Clone, Debug, Default)]
struct Dependency {
    expressions: Vec<bool>,
    predicates: Vec<bool>,
    nodes: Vec<bool>,
}

impl Dependency {
    fn of(template: &AggregationTemplate) -> Self {
        let mut expressions = Vec::with_capacity(template.expressions.len());
        for node in &template.expressions {
            let dependent = matches!(node, ExprNode::SlotSignal { .. })
                || node.operands().into_iter().flatten().any(|operand| {
                    operand
                        .index()
                        .and_then(|index| expressions.get(index))
                        .copied()
                        .unwrap_or(false)
                });
            expressions.push(dependent);
        }
        let flag = |root: ExprId| {
            root.index()
                .and_then(|index| expressions.get(index))
                .copied()
                .unwrap_or(false)
        };
        let predicates: Vec<bool> = template
            .predicates
            .iter()
            .map(|predicate| flag(predicate.left) || flag(predicate.right))
            .collect();
        let dependent_proposition = |proposition: PropositionId| {
            template
                .predicates
                .iter()
                .zip(&predicates)
                .any(|(predicate, dependent)| *dependent && predicate.proposition == proposition)
        };
        let mut nodes = Vec::with_capacity(template.nodes.len());
        for node in &template.nodes {
            let dependent = match node {
                TemplateNode::SlotProposition { .. } | TemplateNode::Inner => true,
                TemplateNode::Node { node } => {
                    let own = matches!(
                        node.kind,
                        NodeKind::Proposition { proposition } if dependent_proposition(proposition)
                    );
                    own || node.kind.operands().into_iter().flatten().any(|operand| {
                        usize::try_from(operand.0)
                            .ok()
                            .and_then(|index| nodes.get(index))
                            .copied()
                            .unwrap_or(false)
                    })
                }
            };
            nodes.push(dependent);
        }
        Self {
            expressions,
            predicates,
            nodes,
        }
    }

    fn count(flags: &[bool], dependent: bool) -> usize {
        flags.iter().filter(|flag| **flag == dependent).count()
    }
}

/// Per-template counts used by the bound.
struct TemplateCounts {
    shared: ExpansionCounts,
    /// Dependent nodes written per instance (the inner leaf writes none).
    nodes: usize,
    predicates: usize,
    expressions: usize,
}

fn template_counts(template: &AggregationTemplate, dependency: &Dependency) -> TemplateCounts {
    let inner_leaves = template
        .nodes
        .iter()
        .filter(|node| matches!(node, TemplateNode::Inner))
        .count();
    TemplateCounts {
        shared: ExpansionCounts {
            nodes: Dependency::count(&dependency.nodes, false),
            predicates: Dependency::count(&dependency.predicates, false),
            expressions: Dependency::count(&dependency.expressions, false),
            members: 0,
        },
        nodes: Dependency::count(&dependency.nodes, true) - inner_leaves,
        predicates: Dependency::count(&dependency.predicates, true),
        expressions: Dependency::count(&dependency.expressions, true),
    }
}

/// The expansion refusal for a document error: bounds pass through as
/// `AggregationTooLarge`, anything else is `Invalid`.
fn refusal(error: SetAggregationError) -> AggregationRefusal {
    match error {
        SetAggregationError::AggregationTooLarge {
            resource,
            actual,
            limit,
        } => AggregationRefusal::AggregationTooLarge {
            resource,
            actual,
            limit,
        },
        other => AggregationRefusal::Invalid(other),
    }
}

fn too_large(resource: AggregationLimit, actual: usize, limit: usize) -> SetAggregationError {
    SetAggregationError::AggregationTooLarge {
        resource,
        actual,
        limit,
    }
}

fn arithmetic() -> SetAggregationError {
    too_large(AggregationLimit::Arithmetic, usize::MAX, MAX_AGGREGATION_NODES)
}

fn add(left: usize, right: usize) -> Result<usize, SetAggregationError> {
    left.checked_add(right).ok_or_else(arithmetic)
}

fn mul(left: usize, right: usize) -> Result<usize, SetAggregationError> {
    left.checked_mul(right).ok_or_else(arithmetic)
}

/// Owned, validated set-aggregation document.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct SetAggregationDocument {
    schema_version: SetAggregationSchemaVersion,
    sets: Vec<MemberSet>,
    first_proposition: PropositionId,
    aggregations: Vec<Aggregation>,
}

impl SetAggregationDocument {
    /// Validates every set, template, member typing, fresh range and the
    /// expansion bound against `catalog` (FR-306, FR-307).
    pub fn new(
        catalog: SignalCatalog<'_>,
        sets: Vec<MemberSet>,
        first_proposition: PropositionId,
        aggregations: Vec<Aggregation>,
    ) -> Result<Self, SetAggregationError> {
        let document = Self {
            schema_version: SetAggregationSchemaVersion::V1,
            sets,
            first_proposition,
            aggregations,
        };
        document.validate(catalog)?;
        Ok(document)
    }

    /// Returns the schema version.
    pub const fn schema_version(&self) -> SetAggregationSchemaVersion {
        self.schema_version
    }

    /// Returns the member sets.
    pub fn sets(&self) -> &[MemberSet] {
        &self.sets
    }

    /// Returns the base of the fresh identity range.
    pub const fn first_proposition(&self) -> PropositionId {
        self.first_proposition
    }

    /// Returns the quantified templates in source order.
    pub fn aggregations(&self) -> &[Aggregation] {
        &self.aggregations
    }

    fn members(&self, aggregation: &Aggregation, index: u32) -> Result<&[SignalId], SetAggregationError> {
        self.sets
            .iter()
            .find(|set| set.name == aggregation.set)
            .map(|set| set.members.as_slice())
            .ok_or(SetAggregationError::UnknownSet { aggregation: index })
    }

    /// Exact output populations; refuses above any bound before building.
    pub fn counts(&self) -> Result<ExpansionCounts, SetAggregationError> {
        let mut total = ExpansionCounts::default();
        for (index, aggregation) in self.aggregations.iter().enumerate() {
            let index = u32::try_from(index).map_err(|_| arithmetic())?;
            let (shared, fresh) = self.aggregation_counts(aggregation, index, 0)?;
            total.nodes = add(add(total.nodes, shared.nodes)?, fresh.nodes)?;
            total.predicates = add(add(total.predicates, shared.predicates)?, fresh.predicates)?;
            total.expressions = add(add(total.expressions, shared.expressions)?, fresh.expressions)?;
            total.members = add(total.members, self.members(aggregation, index)?.len())?;
        }
        for (resource, actual, limit) in [
            (AggregationLimit::Nodes, total.nodes, MAX_AGGREGATION_NODES),
            (AggregationLimit::Predicates, total.predicates, MAX_AGGREGATION_PREDICATES),
            (AggregationLimit::Expressions, total.expressions, MAX_AGGREGATION_EXPRESSIONS),
        ] {
            if actual > limit {
                return Err(too_large(resource, actual, limit));
            }
        }
        Ok(total)
    }

    /// Returns (shared prefix counts, per-expansion counts) for one level.
    /// The per-expansion predicates are exactly the fresh identities needed.
    fn aggregation_counts(
        &self,
        aggregation: &Aggregation,
        index: u32,
        depth: usize,
    ) -> Result<(ExpansionCounts, ExpansionCounts), SetAggregationError> {
        if depth >= MAX_AGGREGATION_DEPTH {
            return Err(SetAggregationError::AggregationTooDeep { aggregation: index });
        }
        let members = self.members(aggregation, index)?.len();
        if members > MAX_SET_MEMBERS {
            return Err(too_large(AggregationLimit::Members, members, MAX_SET_MEMBERS));
        }
        let combinator = quantifier_node_count(aggregation.quantifier, members)?;
        let template = &aggregation.template;
        let counts = template_counts(template, &Dependency::of(template));
        let mut shared = counts.shared;
        let mut per_instance = ExpansionCounts {
            nodes: counts.nodes,
            predicates: counts.predicates,
            expressions: counts.expressions,
            members: 0,
        };
        if let Some(inner) = &template.inner {
            let (inner_shared, inner_expansion) = self.aggregation_counts(inner, index, depth + 1)?;
            shared.nodes = add(shared.nodes, inner_shared.nodes)?;
            shared.predicates = add(shared.predicates, inner_shared.predicates)?;
            shared.expressions = add(shared.expressions, inner_shared.expressions)?;
            per_instance.nodes = add(per_instance.nodes, inner_expansion.nodes)?;
            per_instance.predicates = add(per_instance.predicates, inner_expansion.predicates)?;
            per_instance.expressions = add(per_instance.expressions, inner_expansion.expressions)?;
        }
        let expansion = ExpansionCounts {
            nodes: add(mul(members, per_instance.nodes)?, combinator)?,
            predicates: mul(members, per_instance.predicates)?,
            expressions: mul(members, per_instance.expressions)?,
            members,
        };
        Ok((shared, expansion))
    }

    fn validate(&self, catalog: SignalCatalog<'_>) -> Result<(), SetAggregationError> {
        for (index, set) in self.sets.iter().enumerate() {
            let set_index = u32::try_from(index).map_err(|_| arithmetic())?;
            if set.name.is_empty() {
                return Err(SetAggregationError::EmptySetName { set: set_index });
            }
            if set.name.len() > MAX_SIGNAL_NAME_BYTES {
                return Err(SetAggregationError::SetNameTooLong { set: set_index });
            }
            if let Some(first) = self.sets[..index].iter().position(|other| other.name == set.name) {
                return Err(SetAggregationError::DuplicateSetName {
                    first: first as u32,
                    second: set_index,
                });
            }
            if set.members.len() > MAX_SET_MEMBERS {
                return Err(too_large(
                    AggregationLimit::Members,
                    set.members.len(),
                    MAX_SET_MEMBERS,
                ));
            }
            for (position, member) in set.members.iter().enumerate() {
                if catalog.signal(*member).is_none() {
                    return Err(SetAggregationError::UndeclaredMember {
                        set: set_index,
                        signal: *member,
                    });
                }
                if set.members[..position].contains(member) {
                    return Err(SetAggregationError::DuplicateMember {
                        set: set_index,
                        signal: *member,
                    });
                }
            }
        }
        for (index, aggregation) in self.aggregations.iter().enumerate() {
            let index = u32::try_from(index).map_err(|_| arithmetic())?;
            self.validate_structure(aggregation, index, 0)?;
        }
        let counts = self.counts()?;
        let bindings = BindingIndex::new(catalog);
        for (index, aggregation) in self.aggregations.iter().enumerate() {
            // The index fits `u32`: checked by `validate_structure` above.
            let index = index as u32;
            self.validate_typing(catalog, &bindings, aggregation, index)?;
        }
        self.validate_fresh_range(catalog, counts)
    }

    fn validate_structure(
        &self,
        aggregation: &Aggregation,
        index: u32,
        depth: usize,
    ) -> Result<(), SetAggregationError> {
        if depth >= MAX_AGGREGATION_DEPTH {
            return Err(SetAggregationError::AggregationTooDeep { aggregation: index });
        }
        // `depth` is below MAX_AGGREGATION_DEPTH.
        let level = depth as u8;
        self.members(aggregation, index)?;
        if let Quantifier::AtLeast { n } = aggregation.quantifier {
            if n as usize > MAX_SET_MEMBERS {
                return Err(SetAggregationError::QuantifierOutOfRange {
                    aggregation: index,
                    n,
                });
            }
        }
        let template = &aggregation.template;
        let structure = |node: usize| SetAggregationError::TemplateStructure {
            aggregation: index,
            depth: level,
            node: NodeId(u32::try_from(node).unwrap_or(u32::MAX)),
        };
        let mut inner_leaves = 0_usize;
        for (position, node) in template.nodes.iter().enumerate() {
            match node {
                TemplateNode::Node { node } => {
                    for operand in node.kind.operands().into_iter().flatten() {
                        if usize::try_from(operand.0).map_or(true, |item| item >= position) {
                            return Err(structure(position));
                        }
                    }
                }
                TemplateNode::SlotProposition { depth: slot } => {
                    if usize::from(*slot) > depth {
                        return Err(SetAggregationError::SlotOutsideTemplate {
                            aggregation: index,
                            depth: level,
                        });
                    }
                }
                TemplateNode::Inner => inner_leaves += 1,
            }
        }
        if usize::try_from(template.root.0).map_or(true, |root| root >= template.nodes.len()) {
            return Err(structure(template.nodes.len()));
        }
        for node in &template.expressions {
            if let ExprNode::SlotSignal { depth: slot } = node {
                if usize::from(*slot) > depth {
                    return Err(SetAggregationError::SlotOutsideTemplate {
                        aggregation: index,
                        depth: level,
                    });
                }
            }
        }
        match (&template.inner, inner_leaves) {
            (Some(inner), 1) => self.validate_structure(inner, index, depth + 1),
            (None, 0) => Ok(()),
            _ => Err(SetAggregationError::InnerLeafMismatch {
                aggregation: index,
                depth: level,
            }),
        }
    }

    /// Types each template level once: the member-independent part in one
    /// pass, then only the member-dependent expressions and predicates once
    /// per distinct member domain (pair, for a nested level). The work is
    /// bounded by the template sizes plus the dependent populations the
    /// expansion bound already limits (NFR-019).
    fn validate_typing(
        &self,
        catalog: SignalCatalog<'_>,
        bindings: &BindingIndex,
        aggregation: &Aggregation,
        index: u32,
    ) -> Result<(), SetAggregationError> {
        let outer_members = self.members(aggregation, index)?;
        let mut outer = TemplateTyping::new(catalog, &aggregation.template).map_err(|error| {
            SetAggregationError::TemplatePredicate {
                aggregation: index,
                depth: 0,
                error,
            }
        })?;
        if outer.slot_propositions[0] {
            bindings.require_all(outer_members, index)?;
        }
        let outer_domains = distinct_domains(catalog, outer_members);
        if outer.has_dependents() {
            for member in &outer_domains {
                outer
                    .member(catalog, &aggregation.template, &[*member])
                    .map_err(|error| SetAggregationError::TemplateIllTypedForMember {
                        aggregation: index,
                        member: *member,
                        error,
                    })?;
            }
        }
        let Some(inner) = aggregation.template.inner.as_deref() else {
            return Ok(());
        };
        let inner_members = self.members(inner, index)?;
        let mut typing = TemplateTyping::new(catalog, &inner.template).map_err(|error| {
            SetAggregationError::TemplatePredicate {
                aggregation: index,
                depth: 1,
                error,
            }
        })?;
        if typing.slot_propositions[0] {
            bindings.require_all(outer_members, index)?;
        }
        if typing.slot_propositions[1] {
            bindings.require_all(inner_members, index)?;
        }
        if !typing.has_dependents() {
            return Ok(());
        }
        // A depth whose slot_signal is unused needs one representative only.
        let representatives = |reads: bool, members: &[SignalId], domains: &[SignalId]| {
            if reads {
                domains.to_vec()
            } else {
                members.iter().take(1).copied().collect::<Vec<SignalId>>()
            }
        };
        let inner_domains = distinct_domains(catalog, inner_members);
        let outer_reps = representatives(typing.slot_signals[0], outer_members, &outer_domains);
        let inner_reps = representatives(typing.slot_signals[1], inner_members, &inner_domains);
        for outer_member in &outer_reps {
            for inner_member in &inner_reps {
                typing
                    .member(catalog, &inner.template, &[*outer_member, *inner_member])
                    .map_err(|error| SetAggregationError::TemplateIllTypedForMember {
                        aggregation: index,
                        member: *inner_member,
                        error,
                    })?;
            }
        }
        Ok(())
    }

    fn validate_fresh_range(
        &self,
        catalog: SignalCatalog<'_>,
        counts: ExpansionCounts,
    ) -> Result<(), SetAggregationError> {
        let fresh = self.fresh_count()?;
        let start = u64::from(self.first_proposition.0);
        let end = start + fresh as u64;
        if end > u64::from(u32::MAX) + 1 {
            return Err(SetAggregationError::FreshIdentityCollision {
                proposition: self.first_proposition,
            });
        }
        let inside = |proposition: PropositionId| {
            let value = u64::from(proposition.0);
            start <= value && value < end
        };
        for binding in catalog.bindings() {
            if inside(binding.proposition()) {
                return Err(SetAggregationError::FreshIdentityCollision {
                    proposition: binding.proposition(),
                });
            }
        }
        let mut stack: Vec<&Aggregation> = self.aggregations.iter().collect();
        while let Some(aggregation) = stack.pop() {
            let template = &aggregation.template;
            for node in &template.nodes {
                if let TemplateNode::Node { node } = node {
                    if let NodeKind::Proposition { proposition } = node.kind {
                        if inside(proposition) {
                            return Err(SetAggregationError::FreshIdentityCollision {
                                proposition,
                            });
                        }
                    }
                }
            }
            for predicate in &template.predicates {
                if inside(predicate.proposition) {
                    return Err(SetAggregationError::FreshIdentityCollision {
                        proposition: predicate.proposition,
                    });
                }
            }
            if let Some(inner) = &template.inner {
                stack.push(inner);
            }
        }
        let _ = counts;
        // Shared predicates keep their own identities and are written once,
        // so two templates may not share one.
        let mut shared: Vec<PropositionId> = Vec::new();
        for aggregation in &self.aggregations {
            let mut level = Some(aggregation);
            while let Some(current) = level {
                let template = &current.template;
                let dependency = Dependency::of(template);
                for (position, predicate) in template.predicates.iter().enumerate() {
                    if !dependency.predicates[position] {
                        shared.push(predicate.proposition);
                    }
                }
                level = template.inner.as_deref();
            }
        }
        shared.sort_unstable();
        if let Some(pair) = shared.windows(2).find(|pair| pair[0] == pair[1]) {
            return Err(SetAggregationError::FreshIdentityCollision {
                proposition: pair[0],
            });
        }
        // A shared predicate may not also be a catalog binding (review L-12).
        if let Some(proposition) = shared
            .iter()
            .find(|proposition| catalog.signal_for_proposition(**proposition).is_some())
        {
            return Err(SetAggregationError::PredicateDefinedTwice {
                proposition: *proposition,
            });
        }
        self.validate_placeholder_scope()
    }

    /// A member-dependent placeholder is renamed only inside its own template;
    /// a leaf of any other template naming it would dangle (review L-12).
    fn validate_placeholder_scope(&self) -> Result<(), SetAggregationError> {
        let mut levels: Vec<(&AggregationTemplate, Vec<PropositionId>)> = Vec::new();
        for aggregation in &self.aggregations {
            let mut level = Some(aggregation);
            while let Some(current) = level {
                let template = &current.template;
                let dependency = Dependency::of(template);
                let own: Vec<PropositionId> = template
                    .predicates
                    .iter()
                    .zip(&dependency.predicates)
                    .filter(|(_, dependent)| **dependent)
                    .map(|(predicate, _)| predicate.proposition)
                    .collect();
                levels.push((template, own));
                level = template.inner.as_deref();
            }
        }
        let mut all: Vec<PropositionId> =
            levels.iter().flat_map(|(_, own)| own.iter().copied()).collect();
        all.sort_unstable();
        for (template, own) in &levels {
            for node in &template.nodes {
                let TemplateNode::Node { node } = node else {
                    continue;
                };
                let NodeKind::Proposition { proposition } = node.kind else {
                    continue;
                };
                if all.binary_search(&proposition).is_ok() && !own.contains(&proposition) {
                    return Err(SetAggregationError::PlaceholderOutOfScope { proposition });
                }
            }
        }
        Ok(())
    }

    /// Number of fresh identities the expansion allocates.
    fn fresh_count(&self) -> Result<usize, SetAggregationError> {
        let mut fresh = 0_usize;
        for (index, aggregation) in self.aggregations.iter().enumerate() {
            let (_, expansion) = self.aggregation_counts(aggregation, index as u32, 0)?;
            fresh = add(fresh, expansion.predicates)?;
        }
        Ok(fresh)
    }

    /// Expands into caller-owned slices without allocating output. The
    /// document is first re-validated against `catalog` (review M-13), so a
    /// catalog other than the validating one cannot yield dangling or
    /// ill-typed output. Refuses with `ExpansionTooSmall` and the exact counts,
    /// writing nothing, when any slice is too small.
    pub fn expand_into(
        &self,
        catalog: SignalCatalog<'_>,
        nodes: &mut [Node],
        predicates: &mut [Predicate],
        expressions: &mut [ExprNode],
        roots: &mut [NodeId],
    ) -> Result<ExpansionCounts, AggregationRefusal> {
        self.validate(catalog).map_err(refusal)?;
        let required = self.counts().map_err(refusal)?;
        if nodes.len() < required.nodes
            || predicates.len() < required.predicates
            || expressions.len() < required.expressions
            || roots.len() < self.aggregations.len()
        {
            return Err(AggregationRefusal::ExpansionTooSmall { required });
        }
        let mut expander = Expander {
            document: self,
            nodes,
            predicates,
            expressions,
            node_count: 0,
            predicate_count: 0,
            expression_count: 0,
            next_fresh: self.first_proposition.0,
            bindings: BindingIndex::new(catalog),
            shared: Vec::new(),
        };
        expander.shared_prefix();
        for (index, aggregation) in self.aggregations.iter().enumerate() {
            let mut context = Vec::new();
            roots[index] = expander.aggregation(aggregation, index, 0, &mut context);
        }
        let written = ExpansionCounts {
            nodes: expander.node_count,
            predicates: expander.predicate_count,
            expressions: expander.expression_count,
            members: required.members,
        };
        expander.predicates[..written.predicates]
            .sort_unstable_by_key(|predicate| predicate.proposition);
        debug_assert_eq!(written, required);
        Ok(written)
    }

    /// Owned convenience: sizes the output from [`Self::counts`] and expands.
    /// Refusals are the same as [`Self::expand_into`]'s (review M-14).
    pub fn expand(
        &self,
        catalog: SignalCatalog<'_>,
    ) -> Result<ExpandedAggregation, AggregationRefusal> {
        let required = self.counts().map_err(refusal)?;
        let mut nodes = vec![Node::new(NodeKind::False); required.nodes];
        let mut predicates = vec![
            Predicate {
                proposition: PropositionId(0),
                operator: crate::ComparisonOperator::Eq,
                left: ExprId(0),
                right: ExprId(0),
            };
            required.predicates
        ];
        let mut expressions = vec![ExprNode::Integer { value: 0 }; required.expressions];
        let mut roots = vec![NodeId(0); self.aggregations.len()];
        let report = self.expand_into(
            catalog,
            &mut nodes,
            &mut predicates,
            &mut expressions,
            &mut roots,
        )?;
        Ok(ExpandedAggregation {
            nodes,
            predicates,
            expressions,
            roots,
            report,
        })
    }

    /// Serializes to the one canonical owner encoding.
    #[cfg(feature = "serde")]
    pub fn canonical_json_bytes(&self) -> Result<Vec<u8>, serde_json::Error> {
        canonical_json(self)
    }

    /// Reads exactly one bounded canonical document and validates it against
    /// `catalog` through [`Self::new`].
    #[cfg(feature = "serde")]
    pub fn from_json_bytes(
        bytes: &[u8],
        limits: SyntaxArtifactLimits,
        catalog: SignalCatalog<'_>,
    ) -> Result<Self, SetAggregationReadError> {
        let wire: SetAggregationWire =
            read_strict_document(bytes, limits).map_err(SetAggregationReadError::Read)?;
        let _ = wire.schema_version;
        Self::new(catalog, wire.sets, wire.first_proposition, wire.aggregations)
            .map_err(SetAggregationReadError::Document)
    }
}

/// Catalog bindings indexed by signal: the smallest proposition bound to each
/// Boolean signal. Built once per validation or expansion, O(B log B).
struct BindingIndex {
    by_signal: Vec<(SignalId, PropositionId)>,
}

impl BindingIndex {
    fn new(catalog: SignalCatalog<'_>) -> Self {
        let mut by_signal: Vec<(SignalId, PropositionId)> = catalog
            .bindings()
            .iter()
            .filter(|binding| {
                catalog
                    .signal(binding.signal())
                    .is_some_and(|signal| signal.domain() == SignalDomain::Boolean)
            })
            .map(|binding| (binding.signal(), binding.proposition()))
            .collect();
        by_signal.sort_unstable();
        by_signal.dedup_by_key(|entry| entry.0);
        Self { by_signal }
    }

    fn proposition(&self, member: SignalId) -> Option<PropositionId> {
        self.by_signal
            .binary_search_by_key(&member, |entry| entry.0)
            .ok()
            .map(|index| self.by_signal[index].1)
    }

    fn require_all(&self, members: &[SignalId], aggregation: u32) -> Result<(), SetAggregationError> {
        match members.iter().find(|member| self.proposition(**member).is_none()) {
            Some(member) => Err(SetAggregationError::MemberHasNoBooleanBinding {
                aggregation,
                member: *member,
            }),
            None => Ok(()),
        }
    }
}

/// The first member of each distinct declared domain, in document order.
fn distinct_domains(catalog: SignalCatalog<'_>, members: &[SignalId]) -> Vec<SignalId> {
    let mut seen: Vec<Option<SignalDomain>> = Vec::new();
    let mut representatives = Vec::new();
    for member in members {
        let domain = catalog.signal(*member).map(|signal| signal.domain());
        if !seen.contains(&domain) {
            seen.push(domain);
            representatives.push(*member);
        }
    }
    representatives
}

/// One template level's typing: member-independent types computed once, and
/// the dependent expression and predicate indices re-typed per member.
struct TemplateTyping {
    dependent_expressions: Vec<usize>,
    dependent_predicates: Vec<usize>,
    types: Vec<ExprType>,
    /// Whether a `slot_proposition` names each depth.
    slot_propositions: [bool; MAX_AGGREGATION_DEPTH],
    /// Whether a `slot_signal` names each depth.
    slot_signals: [bool; MAX_AGGREGATION_DEPTH],
}

impl TemplateTyping {
    /// Checks operand order, depth, predicate order and roots, and types the
    /// member-independent part. Dependent nodes get a placeholder type.
    fn new(
        catalog: SignalCatalog<'_>,
        template: &AggregationTemplate,
    ) -> Result<Self, PredicateTableError> {
        let dependency = Dependency::of(template);
        let expressions = &template.expressions;
        let mut types: Vec<ExprType> = Vec::with_capacity(expressions.len());
        let mut depths: Vec<usize> = Vec::with_capacity(expressions.len());
        let mut slot_signals = [false; MAX_AGGREGATION_DEPTH];
        for (index, node) in expressions.iter().copied().enumerate() {
            let node_id = ExprId(u32::try_from(index).map_err(|_| {
                PredicateTableError::Expression(ExpressionError::ExpressionLimitExceeded {
                    resource: crate::ExpressionLimit::Nodes,
                    actual: expressions.len(),
                    limit: crate::MAX_EXPRESSION_NODES,
                })
            })?);
            let mut depth = 1_usize;
            for operand in node.operands().into_iter().flatten() {
                match operand.index().filter(|position| *position < index) {
                    Some(position) => depth = depth.max(depths[position] + 1),
                    None => {
                        return Err(PredicateTableError::Expression(
                            ExpressionError::OperandNotPreceding {
                                node: node_id,
                                operand,
                            },
                        ))
                    }
                }
            }
            if depth > MAX_EXPRESSION_DEPTH {
                return Err(PredicateTableError::Expression(
                    ExpressionError::ExpressionLimitExceeded {
                        resource: crate::ExpressionLimit::Depth,
                        actual: depth,
                        limit: MAX_EXPRESSION_DEPTH,
                    },
                ));
            }
            depths.push(depth);
            if let ExprNode::SlotSignal { depth: slot } = node {
                if let Some(flag) = slot_signals.get_mut(usize::from(slot)) {
                    *flag = true;
                }
            }
            let ty = if dependency.expressions[index] {
                ExprType::default()
            } else {
                type_node(
                    node_id,
                    node,
                    |operand| {
                        operand
                            .index()
                            .and_then(|position| types.get(position))
                            .copied()
                            .unwrap_or_default()
                    },
                    |id, signal| catalog_signal_type(catalog, id, signal),
                    None,
                )?
            };
            types.push(ty);
        }
        for (position, predicate) in template.predicates.iter().enumerate() {
            if let Some(previous) = position.checked_sub(1).map(|item| &template.predicates[item])
            {
                if previous.proposition >= predicate.proposition {
                    return Err(PredicateTableError::PropositionNotIncreasing {
                        previous: previous.proposition,
                        current: predicate.proposition,
                    });
                }
            }
            for root in [predicate.left, predicate.right] {
                if root.index().filter(|index| *index < types.len()).is_none() {
                    return Err(PredicateTableError::RootOutOfRange {
                        proposition: predicate.proposition,
                        root,
                    });
                }
            }
            if !dependency.predicates[position] {
                check_scales(predicate, &types)?;
            }
        }
        let mut slot_propositions = [false; MAX_AGGREGATION_DEPTH];
        for node in &template.nodes {
            if let TemplateNode::SlotProposition { depth } = node {
                if let Some(flag) = slot_propositions.get_mut(usize::from(*depth)) {
                    *flag = true;
                }
            }
        }
        Ok(Self {
            dependent_expressions: (0..expressions.len())
                .filter(|index| dependency.expressions[*index])
                .collect(),
            dependent_predicates: (0..template.predicates.len())
                .filter(|index| dependency.predicates[*index])
                .collect(),
            types,
            slot_propositions,
            slot_signals,
        })
    }

    fn has_dependents(&self) -> bool {
        !self.dependent_expressions.is_empty() || !self.dependent_predicates.is_empty()
    }

    /// Re-types only the dependent expressions and predicates for `context`.
    fn member(
        &mut self,
        catalog: SignalCatalog<'_>,
        template: &AggregationTemplate,
        context: &[SignalId],
    ) -> Result<(), PredicateTableError> {
        let member = |node: ExprId, depth: u8| -> Result<ExprType, ExpressionError> {
            let signal = context
                .get(usize::from(depth))
                .copied()
                .ok_or(ExpressionError::SlotOutsideTemplate { node })?;
            catalog_signal_type(catalog, node, signal)
        };
        for &index in &self.dependent_expressions {
            // `index` is below the template length, checked to fit `u32`.
            let node_id = ExprId(index as u32);
            let types = &self.types;
            let ty = type_node(
                node_id,
                template.expressions[index],
                |operand| {
                    operand
                        .index()
                        .and_then(|position| types.get(position))
                        .copied()
                        .unwrap_or_default()
                },
                |id, signal| catalog_signal_type(catalog, id, signal),
                Some(&member as &dyn Fn(ExprId, u8) -> Result<ExprType, ExpressionError>),
            )?;
            self.types[index] = ty;
        }
        for &index in &self.dependent_predicates {
            check_scales(&template.predicates[index], &self.types)?;
        }
        Ok(())
    }
}

fn check_scales(predicate: &Predicate, types: &[ExprType]) -> Result<(), PredicateTableError> {
    let scale = |root: ExprId| {
        root.index()
            .and_then(|index| types.get(index))
            .map_or(0, |ty| ty.scale)
    };
    let (left, right) = (scale(predicate.left), scale(predicate.right));
    if left != right {
        return Err(PredicateTableError::ScaleMismatch {
            proposition: predicate.proposition,
            left,
            right,
        });
    }
    Ok(())
}

/// Writes one validated expansion into pre-sized slices.
struct Expander<'d, 'o> {
    document: &'d SetAggregationDocument,
    nodes: &'o mut [Node],
    predicates: &'o mut [Predicate],
    expressions: &'o mut [ExprNode],
    node_count: usize,
    predicate_count: usize,
    expression_count: usize,
    next_fresh: u32,
    bindings: BindingIndex,
    /// Shared-prefix identities per aggregation, per template depth.
    shared: Vec<Vec<SharedMap>>,
}

/// Output identities of one template's shared (member-independent) items,
/// with the template's dependency flags computed once (review M-12).
#[derive(Clone, Debug, Default)]
struct SharedMap {
    expressions: Vec<Option<ExprId>>,
    nodes: Vec<Option<NodeId>>,
    dependency: Dependency,
}

fn lookup<T: Copy>(local: &[Option<T>], shared: &[Option<T>], index: usize) -> Option<T> {
    local
        .get(index)
        .copied()
        .flatten()
        .or_else(|| shared.get(index).copied().flatten())
}

impl Expander<'_, '_> {
    // Counts were checked against MAX_AGGREGATION_* (all below u32::MAX)
    // before any write, so the identity casts below are lossless.
    fn push_node(&mut self, kind: NodeKind) -> NodeId {
        self.nodes[self.node_count] = Node::new(kind);
        let id = NodeId(self.node_count as u32);
        self.node_count += 1;
        id
    }

    fn push_expression(&mut self, node: ExprNode) -> ExprId {
        self.expressions[self.expression_count] = node;
        let id = ExprId(self.expression_count as u32);
        self.expression_count += 1;
        id
    }

    fn push_predicate(&mut self, predicate: Predicate) {
        self.predicates[self.predicate_count] = predicate;
        self.predicate_count += 1;
    }

    /// The shared prefix of every template, in aggregation then depth order.
    fn shared_prefix(&mut self) {
        let document = self.document;
        for aggregation in &document.aggregations {
            let mut maps = Vec::new();
            let mut level = Some(aggregation);
            while let Some(current) = level {
                maps.push(self.write_shared(&current.template));
                level = current.template.inner.as_deref();
            }
            self.shared.push(maps);
        }
    }

    fn write_shared(&mut self, template: &AggregationTemplate) -> SharedMap {
        let dependency = Dependency::of(template);
        let mut map = SharedMap {
            expressions: vec![None; template.expressions.len()],
            nodes: vec![None; template.nodes.len()],
            dependency: Dependency::default(),
        };
        for (index, node) in template.expressions.iter().copied().enumerate() {
            if !dependency.expressions[index] {
                let remapped = remap_expression(node, &map.expressions, &[], None);
                map.expressions[index] = Some(self.push_expression(remapped));
            }
        }
        for (index, predicate) in template.predicates.iter().enumerate() {
            if !dependency.predicates[index] {
                let left = lookup(&[], &map.expressions, predicate.left.0 as usize);
                let right = lookup(&[], &map.expressions, predicate.right.0 as usize);
                self.push_predicate(Predicate {
                    left: left.unwrap_or(predicate.left),
                    right: right.unwrap_or(predicate.right),
                    ..*predicate
                });
            }
        }
        for (index, node) in template.nodes.iter().enumerate() {
            if let (false, TemplateNode::Node { node }) = (dependency.nodes[index], node) {
                let kind = remap_kind(node.kind, |operand| {
                    lookup(&[], &map.nodes, operand.0 as usize).unwrap_or(operand)
                });
                map.nodes[index] = Some(self.push_node(kind));
            }
        }
        map.dependency = dependency;
        map
    }

    /// Expands one quantified template at `depth` under `context`.
    fn aggregation(
        &mut self,
        aggregation: &Aggregation,
        index: usize,
        depth: usize,
        context: &mut Vec<SignalId>,
    ) -> NodeId {
        let document = self.document;
        // Validated: the set exists.
        let members = document.members(aggregation, index as u32).unwrap_or(&[]);
        let mut roots = Vec::with_capacity(members.len());
        for member in members {
            context.push(*member);
            let root = self.instance(&aggregation.template, index, depth, context);
            roots.push(root);
            context.pop();
        }
        let first = NodeId(self.node_count as u32);
        let start = self.node_count;
        match expand_quantifier_into(aggregation.quantifier, &roots, first, &mut self.nodes[start..]) {
            Ok((root, written)) => {
                self.node_count += written;
                root
            }
            // Unreachable for a validated, pre-sized expansion.
            Err(_) => first,
        }
    }

    /// Writes one instance of `template` for the members in `context`.
    fn instance(
        &mut self,
        template: &AggregationTemplate,
        index: usize,
        depth: usize,
        context: &mut Vec<SignalId>,
    ) -> NodeId {
        let shared = core::mem::take(&mut self.shared[index][depth]);
        let dependency = &shared.dependency;
        let mut expressions = vec![None; template.expressions.len()];
        for (position, node) in template.expressions.iter().copied().enumerate() {
            if dependency.expressions[position] {
                let remapped =
                    remap_expression(node, &shared.expressions, &expressions, Some(context.as_slice()));
                expressions[position] = Some(self.push_expression(remapped));
            }
        }
        let mut renamed: Vec<(PropositionId, PropositionId)> = Vec::new();
        for (position, predicate) in template.predicates.iter().enumerate() {
            if dependency.predicates[position] {
                let fresh = PropositionId(self.next_fresh);
                // The fresh range was checked to fit `u32`.
                self.next_fresh = self.next_fresh.wrapping_add(1);
                let left = lookup(&expressions, &shared.expressions, predicate.left.0 as usize);
                let right = lookup(&expressions, &shared.expressions, predicate.right.0 as usize);
                self.push_predicate(Predicate {
                    proposition: fresh,
                    operator: predicate.operator,
                    left: left.unwrap_or(predicate.left),
                    right: right.unwrap_or(predicate.right),
                });
                renamed.push((predicate.proposition, fresh));
            }
        }
        let mut nodes: Vec<Option<NodeId>> = vec![None; template.nodes.len()];
        for (position, node) in template.nodes.iter().enumerate() {
            if !dependency.nodes[position] {
                continue;
            }
            let id = match node {
                TemplateNode::SlotProposition { depth: slot } => {
                    let member = context[usize::from(*slot)];
                    // Validated: every member of a slot_proposition set is bound.
                    let proposition =
                        self.bindings.proposition(member).unwrap_or(PropositionId(0));
                    self.push_node(NodeKind::Proposition { proposition })
                }
                TemplateNode::Inner => match template.inner.as_deref() {
                    Some(inner) => self.aggregation(inner, index, depth + 1, context),
                    None => self.push_node(NodeKind::False),
                },
                TemplateNode::Node { node } => {
                    let kind = match node.kind {
                        NodeKind::Proposition { proposition } => NodeKind::Proposition {
                            proposition: renamed
                                .iter()
                                .find(|(placeholder, _)| *placeholder == proposition)
                                .map_or(proposition, |(_, fresh)| *fresh),
                        },
                        other => remap_kind(other, |operand| {
                            lookup(&nodes, &shared.nodes, operand.0 as usize).unwrap_or(operand)
                        }),
                    };
                    self.push_node(kind)
                }
            };
            nodes[position] = Some(id);
        }
        let root = lookup(&nodes, &shared.nodes, template.root.0 as usize).unwrap_or(template.root);
        self.shared[index][depth] = shared;
        root
    }
}

/// Remaps expression operands; dependent ones from `local`, shared from
/// `shared`; resolves `slot_signal` from `context`.
fn remap_expression(
    node: ExprNode,
    shared: &[Option<ExprId>],
    local: &[Option<ExprId>],
    context: Option<&[SignalId]>,
) -> ExprNode {
    let map = |operand: ExprId| {
        let index = operand.0 as usize;
        local
            .get(index)
            .copied()
            .flatten()
            .or_else(|| shared.get(index).copied().flatten())
            .unwrap_or(operand)
    };
    match node {
        ExprNode::SlotSignal { depth } => match context.and_then(|c| c.get(usize::from(depth))) {
            Some(signal) => ExprNode::Signal { signal: *signal },
            None => node,
        },
        ExprNode::Neg { operand } => ExprNode::Neg {
            operand: map(operand),
        },
        ExprNode::Abs { operand } => ExprNode::Abs {
            operand: map(operand),
        },
        ExprNode::Rescale { operand, scale } => ExprNode::Rescale {
            operand: map(operand),
            scale,
        },
        ExprNode::Add { left, right } => ExprNode::Add {
            left: map(left),
            right: map(right),
        },
        ExprNode::Sub { left, right } => ExprNode::Sub {
            left: map(left),
            right: map(right),
        },
        ExprNode::Mul { left, right } => ExprNode::Mul {
            left: map(left),
            right: map(right),
        },
        other => other,
    }
}

/// Rewrites every operand of a formula node kind.
fn remap_kind(kind: NodeKind, map: impl Fn(NodeId) -> NodeId) -> NodeKind {
    match kind {
        NodeKind::Not { operand } => NodeKind::Not {
            operand: map(operand),
        },
        NodeKind::And { left, right } => NodeKind::And {
            left: map(left),
            right: map(right),
        },
        NodeKind::Or { left, right } => NodeKind::Or {
            left: map(left),
            right: map(right),
        },
        NodeKind::Implies { left, right } => NodeKind::Implies {
            left: map(left),
            right: map(right),
        },
        NodeKind::Equivalent { left, right } => NodeKind::Equivalent {
            left: map(left),
            right: map(right),
        },
        NodeKind::Future { interval, operand } => NodeKind::Future {
            interval,
            operand: map(operand),
        },
        NodeKind::Globally { interval, operand } => NodeKind::Globally {
            interval,
            operand: map(operand),
        },
        NodeKind::Until {
            interval,
            left,
            right,
        } => NodeKind::Until {
            interval,
            left: map(left),
            right: map(right),
        },
        NodeKind::Release {
            interval,
            left,
            right,
        } => NodeKind::Release {
            interval,
            left: map(left),
            right: map(right),
        },
        NodeKind::Once { interval, operand } => NodeKind::Once {
            interval,
            operand: map(operand),
        },
        NodeKind::Historically { interval, operand } => NodeKind::Historically {
            interval,
            operand: map(operand),
        },
        NodeKind::StrongPrevious { operand } => NodeKind::StrongPrevious {
            operand: map(operand),
        },
        NodeKind::Since {
            interval,
            left,
            right,
        } => NodeKind::Since {
            interval,
            left: map(left),
            right: map(right),
        },
        NodeKind::Triggered {
            interval,
            left,
            right,
        } => NodeKind::Triggered {
            interval,
            left: map(left),
            right: map(right),
        },
        NodeKind::LtlfFuture { operand } => NodeKind::LtlfFuture {
            operand: map(operand),
        },
        NodeKind::LtlfGlobally { operand } => NodeKind::LtlfGlobally {
            operand: map(operand),
        },
        NodeKind::LtlfUntil { left, right } => NodeKind::LtlfUntil {
            left: map(left),
            right: map(right),
        },
        NodeKind::LtlfRelease { left, right } => NodeKind::LtlfRelease {
            left: map(left),
            right: map(right),
        },
        NodeKind::StrongNext { operand } => NodeKind::StrongNext {
            operand: map(operand),
        },
        NodeKind::WeakNext { operand } => NodeKind::WeakNext {
            operand: map(operand),
        },
        leaf @ (NodeKind::False | NodeKind::True | NodeKind::Proposition { .. }) => leaf,
    }
}

/// Failure to read an aggregation document.
#[derive(Debug)]
#[non_exhaustive]
pub enum SetAggregationReadError {
    /// Byte, shape, canonical-form or resource violation.
    #[cfg(feature = "serde")]
    Read(StrictDocumentReadError),
    /// The decoded document fails exactly as direct construction would.
    Document(SetAggregationError),
}

impl fmt::Display for SetAggregationReadError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            #[cfg(feature = "serde")]
            Self::Read(error) => error.fmt(formatter),
            Self::Document(error) => error.fmt(formatter),
        }
    }
}

#[cfg(feature = "serde")]
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
struct SetAggregationWire {
    schema_version: SetAggregationSchemaVersion,
    sets: Vec<MemberSet>,
    first_proposition: PropositionId,
    aggregations: Vec<Aggregation>,
}

#[cfg(feature = "serde")]
impl StrictDocument for SetAggregationWire {
    fn preflight_resource_limits(
        bytes: &[u8],
        _limits: SyntaxArtifactLimits,
    ) -> Result<usize, StrictDocumentReadError> {
        // Population bounds are the document's own AggregationTooLarge refusals,
        // so the reader and direct construction fail alike (FR-303 pattern).
        Ok(crate::contracts::reader::array_field_population(bytes, b"members"))
    }

    fn validate_resource_limits(
        &self,
        _limits: SyntaxArtifactLimits,
    ) -> Result<(), StrictDocumentReadError> {
        Ok(())
    }
}
