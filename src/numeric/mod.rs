//! Numeric predicate layer (P-TL-150): exact expressions over catalog
//! signals, predicate tables that define propositions, and their binding to
//! formulas. The temporal core (node kinds, profiles, formula editions) is
//! unchanged; the borrowed model stays `no_std` without allocation (NFR-018).

pub mod expr;
pub mod predicate;

#[cfg(feature = "alloc")]
pub mod document;

#[cfg(test)]
mod tests {
    use super::{
        expr::{ExprId, ExprNode, ExprSlot},
        predicate::{BoundPredicateFormula, ComparisonOperator, Predicate, PredicateTable},
    };
    use crate::{
        Formula, IntegerSignalDomain, Node, NodeId, NodeKind, PropositionId, SemanticProfile,
        SignalCatalog, SignalDeclaration, SignalDomain, SignalId,
    };

    /// Every borrowed numeric API runs on fixed-size stack arrays only.
    // Trace: TC-1530, NFR-018-AC-1
    #[test]
    fn borrowed_numeric_model_needs_no_allocation() {
        let signals = [SignalDeclaration::new(
            SignalId(1),
            "count",
            SignalDomain::Integer(IntegerSignalDomain::new(0, 9).unwrap()),
        )];
        let mut order = [0_u32; 1];
        let catalog = SignalCatalog::new(&signals, &[], &mut order).unwrap();
        let expressions = [
            ExprNode::Signal {
                signal: SignalId(1),
            },
            ExprNode::Integer { value: 4 },
        ];
        let predicates = [Predicate {
            proposition: PropositionId(3),
            operator: ComparisonOperator::Gt,
            left: ExprId(0),
            right: ExprId(1),
        }];
        let mut slots = [ExprSlot::EMPTY; 2];
        let table = PredicateTable::new(catalog, &predicates, &expressions, &mut slots).unwrap();
        assert_eq!(slots[0].ty().maximum, 9);
        let nodes = [Node::new(NodeKind::Proposition {
            proposition: PropositionId(3),
        })];
        let formula = Formula::new(SemanticProfile::ClosedTraceV1, NodeId(0), &nodes).unwrap();
        let mut binding_slots = [ExprSlot::EMPTY; 2];
        let bound =
            BoundPredicateFormula::new(formula, catalog, table, &mut binding_slots).unwrap();
        assert!(bound.source(PropositionId(3)).is_some());
    }
}
