//! Non-wire liveness capability routing and subject-bound settlements.

use crate::{InfiniteFormulaDocument, LIVENESS_CAPABILITY_V1};

/// Exact subject scope selected by a liveness request.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum LivenessSubjectKind {
    /// One complete ultimately periodic trace.
    LassoTrace,
    /// A finite prefix with no claimed infinite closure.
    FinitePrefix,
    /// A transition system or model with multiple admitted traces.
    Model,
}

/// A subject selection whose identity is supplied by its owner.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct LivenessSubject<'a> {
    /// Exact kind of subject.
    pub kind: LivenessSubjectKind,
    /// Identity of that one subject.
    pub identity: &'a str,
}

/// One liveness settlement disposition, with FR-341 resource failure kept distinct.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LivenessDisposition {
    /// The provider proved the formula for the selected subject.
    Proved,
    /// The provider found a counterexample for the selected subject.
    Refuted,
    /// Completed run with unsettled truth.
    Inconclusive,
    /// Requested capability or subject procedure is unavailable.
    Unsupported,
    /// A resource ceiling prevented completion.
    ResourceIncomplete,
    /// Internal execution failure.
    Failed,
}

/// Settlement returned by the owner capability router.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct LivenessSettlement<'formula, 'subject> {
    /// Exact canonical formula document supplied to the capability router.
    pub formula: &'formula InfiniteFormulaDocument,
    /// The selected subject, never widened by routing.
    pub subject: LivenessSubject<'subject>,
    /// The provider disposition or absent-backend `Unsupported`.
    pub disposition: LivenessDisposition,
    /// Capability named by the absence warning, if no backend was registered.
    pub warning: Option<&'static str>,
}

/// Registered infinite-trace provider for `tl-syntax.liveness/v1`.
pub trait LivenessBackend {
    /// Settles the canonical formula for exactly the selected subject.
    fn settle(
        &self,
        formula: &InfiniteFormulaDocument,
        subject: LivenessSubject<'_>,
    ) -> LivenessDisposition;
}

/// Routes a canonical infinite formula through the optional registered provider.
///
/// A missing provider settles `Unsupported` immediately. A finite prefix
/// cannot yield `Proved` for an unbounded liveness claim.
pub fn settle_liveness<'formula, 'subject>(
    formula: &'formula InfiniteFormulaDocument,
    subject: LivenessSubject<'subject>,
    backend: Option<&dyn LivenessBackend>,
) -> LivenessSettlement<'formula, 'subject> {
    let Some(backend) = backend else {
        return LivenessSettlement {
            formula,
            subject,
            disposition: LivenessDisposition::Unsupported,
            warning: Some(LIVENESS_CAPABILITY_V1),
        };
    };
    let disposition = backend.settle(formula, subject);
    let disposition = if subject.kind == LivenessSubjectKind::FinitePrefix
        && disposition == LivenessDisposition::Proved
    {
        LivenessDisposition::Failed
    } else {
        disposition
    };
    LivenessSettlement {
        formula,
        subject,
        disposition,
        warning: None,
    }
}
