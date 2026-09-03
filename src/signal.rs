use core::{fmt, iter::FusedIterator};

use crate::{Formula, NodeKind, PropositionId};

#[cfg(feature = "alloc")]
use crate::signal_document::OwnedSignalDeclaration;

/// Maximum number of signal declarations in one v1 catalog.
pub const MAX_SIGNAL_CATALOG_SIGNALS: usize = 100_000;
/// Maximum number of proposition bindings in one v1 catalog.
pub const MAX_SIGNAL_CATALOG_BINDINGS: usize = 100_000;
/// Maximum UTF-8 byte length of one signal name.
pub const MAX_SIGNAL_NAME_BYTES: usize = 255;

/// Stable numeric identity of a declared input signal.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
pub struct SignalId(pub u32);

/// Checked inclusive bounds for a signed integer signal.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(try_from = "IntegerSignalDomainWire"))]
pub struct IntegerSignalDomain {
    minimum: i64,
    maximum: i64,
}

#[cfg(feature = "serde")]
#[derive(serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct IntegerSignalDomainWire {
    minimum: i64,
    maximum: i64,
}

#[cfg(feature = "serde")]
impl TryFrom<IntegerSignalDomainWire> for IntegerSignalDomain {
    type Error = SignalDomainError;

    fn try_from(wire: IntegerSignalDomainWire) -> Result<Self, Self::Error> {
        Self::new(wire.minimum, wire.maximum)
    }
}

impl IntegerSignalDomain {
    /// Constructs inclusive signed integer bounds.
    pub const fn new(minimum: i64, maximum: i64) -> Result<Self, SignalDomainError> {
        if minimum <= maximum {
            Ok(Self { minimum, maximum })
        } else {
            Err(SignalDomainError::IntegerBoundsInverted { minimum, maximum })
        }
    }

    /// Returns the inclusive minimum.
    pub const fn minimum(self) -> i64 {
        self.minimum
    }

    /// Returns the inclusive maximum.
    pub const fn maximum(self) -> i64 {
        self.maximum
    }
}

/// Checked inclusive coefficient bounds and scale for a fixed-decimal signal.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(try_from = "FixedDecimalSignalDomainWire"))]
pub struct FixedDecimalSignalDomain {
    minimum_coefficient: i64,
    maximum_coefficient: i64,
    scale: u8,
}

#[cfg(feature = "serde")]
#[derive(serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct FixedDecimalSignalDomainWire {
    minimum_coefficient: i64,
    maximum_coefficient: i64,
    scale: u8,
}

#[cfg(feature = "serde")]
impl TryFrom<FixedDecimalSignalDomainWire> for FixedDecimalSignalDomain {
    type Error = SignalDomainError;

    fn try_from(wire: FixedDecimalSignalDomainWire) -> Result<Self, Self::Error> {
        Self::new(
            wire.minimum_coefficient,
            wire.maximum_coefficient,
            wire.scale,
        )
    }
}

impl FixedDecimalSignalDomain {
    /// Constructs inclusive coefficient bounds at a scale from 0 through 18.
    pub const fn new(
        minimum_coefficient: i64,
        maximum_coefficient: i64,
        scale: u8,
    ) -> Result<Self, SignalDomainError> {
        if minimum_coefficient > maximum_coefficient {
            Err(SignalDomainError::DecimalBoundsInverted {
                minimum_coefficient,
                maximum_coefficient,
            })
        } else if scale > 18 {
            Err(SignalDomainError::DecimalScaleOutOfRange { scale })
        } else {
            Ok(Self {
                minimum_coefficient,
                maximum_coefficient,
                scale,
            })
        }
    }

    /// Returns the inclusive minimum coefficient.
    pub const fn minimum_coefficient(self) -> i64 {
        self.minimum_coefficient
    }

    /// Returns the inclusive maximum coefficient.
    pub const fn maximum_coefficient(self) -> i64 {
        self.maximum_coefficient
    }

    /// Returns the number of base-ten fractional digits.
    pub const fn scale(self) -> u8 {
        self.scale
    }
}

/// Closed v1 value-domain vocabulary for temporal input signals.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum SignalDomain {
    /// A Boolean signal that may bind directly to an MLTL proposition.
    Boolean,
    /// A bounded signed integer signal.
    Integer(IntegerSignalDomain),
    /// A bounded fixed-decimal signal represented by a scaled coefficient.
    FixedDecimal(FixedDecimalSignalDomain),
}

#[cfg(feature = "serde")]
impl serde::Serialize for SignalDomain {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;

        match self {
            Self::Boolean => {
                let mut state = serializer.serialize_struct("SignalDomain", 1)?;
                state.serialize_field("kind", "boolean")?;
                state.end()
            }
            Self::Integer(value) => {
                let mut state = serializer.serialize_struct("SignalDomain", 3)?;
                state.serialize_field("kind", "integer")?;
                state.serialize_field("minimum", &value.minimum())?;
                state.serialize_field("maximum", &value.maximum())?;
                state.end()
            }
            Self::FixedDecimal(value) => {
                let mut state = serializer.serialize_struct("SignalDomain", 4)?;
                state.serialize_field("kind", "fixed_decimal")?;
                state.serialize_field("minimum_coefficient", &value.minimum_coefficient())?;
                state.serialize_field("maximum_coefficient", &value.maximum_coefficient())?;
                state.serialize_field("scale", &value.scale())?;
                state.end()
            }
        }
    }
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for SignalDomain {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde::de::Error as _;

        #[derive(serde::Deserialize)]
        #[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
        enum Wire {
            Boolean,
            Integer {
                minimum: i64,
                maximum: i64,
            },
            FixedDecimal {
                minimum_coefficient: i64,
                maximum_coefficient: i64,
                scale: u8,
            },
        }

        match Wire::deserialize(deserializer)? {
            Wire::Boolean => Ok(Self::Boolean),
            Wire::Integer { minimum, maximum } => IntegerSignalDomain::new(minimum, maximum)
                .map(Self::Integer)
                .map_err(D::Error::custom),
            Wire::FixedDecimal {
                minimum_coefficient,
                maximum_coefficient,
                scale,
            } => FixedDecimalSignalDomain::new(minimum_coefficient, maximum_coefficient, scale)
                .map(Self::FixedDecimal)
                .map_err(D::Error::custom),
        }
    }
}

/// One borrowed named signal declaration.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct SignalDeclaration<'a> {
    id: SignalId,
    name: &'a str,
    domain: SignalDomain,
}

impl<'a> SignalDeclaration<'a> {
    /// Constructs an unvalidated declaration for validation as part of a catalog.
    pub const fn new(id: SignalId, name: &'a str, domain: SignalDomain) -> Self {
        Self { id, name, domain }
    }

    /// Returns the stable signal identity.
    pub const fn id(self) -> SignalId {
        self.id
    }

    /// Returns the exact caller-supplied UTF-8 name.
    pub const fn name(self) -> &'a str {
        self.name
    }

    /// Returns the closed value domain.
    pub const fn domain(self) -> SignalDomain {
        self.domain
    }
}

/// One direct MLTL proposition-to-signal binding.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(deny_unknown_fields))]
pub struct PropositionBinding {
    proposition: PropositionId,
    signal: SignalId,
}

impl PropositionBinding {
    /// Constructs an unvalidated binding for validation as part of a catalog.
    pub const fn new(proposition: PropositionId, signal: SignalId) -> Self {
        Self {
            proposition,
            signal,
        }
    }

    /// Returns the proposition identity.
    pub const fn proposition(self) -> PropositionId {
        self.proposition
    }

    /// Returns the target signal identity.
    pub const fn signal(self) -> SignalId {
        self.signal
    }
}

#[derive(Clone, Copy, Debug)]
enum SignalStorage<'a> {
    Borrowed(&'a [SignalDeclaration<'a>]),
    #[cfg(feature = "alloc")]
    Owned(&'a [OwnedSignalDeclaration]),
}

impl<'a> SignalStorage<'a> {
    const fn len(self) -> usize {
        match self {
            Self::Borrowed(signals) => signals.len(),
            #[cfg(feature = "alloc")]
            Self::Owned(signals) => signals.len(),
        }
    }

    fn get(self, index: usize) -> Option<SignalDeclaration<'a>> {
        match self {
            Self::Borrowed(signals) => signals.get(index).copied(),
            #[cfg(feature = "alloc")]
            Self::Owned(signals) => signals.get(index).map(OwnedSignalDeclaration::as_borrowed),
        }
    }
}

/// Allocation-free validated view of a signal catalog.
#[derive(Clone, Copy, Debug)]
pub struct SignalCatalog<'a> {
    signals: SignalStorage<'a>,
    bindings: &'a [PropositionBinding],
}

impl<'a> SignalCatalog<'a> {
    /// Validates borrowed declarations and direct proposition bindings.
    pub fn new(
        signals: &'a [SignalDeclaration<'a>],
        bindings: &'a [PropositionBinding],
        name_order_scratch: &mut [u32],
    ) -> Result<Self, SignalCatalogError> {
        Self::from_storage(
            SignalStorage::Borrowed(signals),
            bindings,
            name_order_scratch,
        )
    }

    #[cfg(feature = "alloc")]
    pub(crate) fn from_owned(
        signals: &'a [OwnedSignalDeclaration],
        bindings: &'a [PropositionBinding],
        name_order_scratch: &mut [u32],
    ) -> Result<Self, SignalCatalogError> {
        Self::from_storage(SignalStorage::Owned(signals), bindings, name_order_scratch)
    }

    fn from_storage(
        signals: SignalStorage<'a>,
        bindings: &'a [PropositionBinding],
        name_order_scratch: &mut [u32],
    ) -> Result<Self, SignalCatalogError> {
        let catalog = Self { signals, bindings };
        catalog.validate(name_order_scratch)?;
        Ok(catalog)
    }

    fn validate(self, name_order_scratch: &mut [u32]) -> Result<(), SignalCatalogError> {
        if self.signals.len() > MAX_SIGNAL_CATALOG_SIGNALS {
            return Err(SignalCatalogError::SignalLimitExceeded {
                count: self.signals.len(),
                limit: MAX_SIGNAL_CATALOG_SIGNALS,
            });
        }
        if self.bindings.len() > MAX_SIGNAL_CATALOG_BINDINGS {
            return Err(SignalCatalogError::BindingLimitExceeded {
                count: self.bindings.len(),
                limit: MAX_SIGNAL_CATALOG_BINDINGS,
            });
        }
        if name_order_scratch.len() < self.signals.len() {
            return Err(SignalCatalogError::NameOrderScratchTooSmall {
                provided: name_order_scratch.len(),
                required: self.signals.len(),
            });
        }

        for index in 0..self.signals.len() {
            let signal = self
                .signals
                .get(index)
                .expect("index is bounded by signal storage length");
            if let Some(previous) = index.checked_sub(1).and_then(|item| self.signals.get(item)) {
                if previous.id >= signal.id {
                    return Err(SignalCatalogError::SignalIdentityNotIncreasing {
                        previous: previous.id,
                        current: signal.id,
                    });
                }
            }
            if signal.name.is_empty() {
                return Err(SignalCatalogError::EmptySignalName { signal: signal.id });
            }
            if signal.name.len() > MAX_SIGNAL_NAME_BYTES {
                return Err(SignalCatalogError::SignalNameTooLong {
                    signal: signal.id,
                    length: signal.name.len(),
                    limit: MAX_SIGNAL_NAME_BYTES,
                });
            }
        }

        let name_order = &mut name_order_scratch[..self.signals.len()];
        for (index, slot) in name_order.iter_mut().enumerate() {
            *slot = u32::try_from(index)
                .expect("the catalog limit is smaller than the u32 identity space");
        }
        name_order.sort_unstable_by(|left, right| {
            let left = self
                .signals
                .get(*left as usize)
                .expect("scratch indices are initialized from signal storage");
            let right = self
                .signals
                .get(*right as usize)
                .expect("scratch indices are initialized from signal storage");
            left.name
                .as_bytes()
                .cmp(right.name.as_bytes())
                .then_with(|| left.id.cmp(&right.id))
        });
        for pair in name_order.windows(2) {
            let first = self
                .signals
                .get(pair[0] as usize)
                .expect("sorted scratch index came from signal storage");
            let second = self
                .signals
                .get(pair[1] as usize)
                .expect("sorted scratch index came from signal storage");
            if first.name.as_bytes() == second.name.as_bytes() {
                return Err(SignalCatalogError::DuplicateSignalName {
                    first: first.id,
                    second: second.id,
                });
            }
        }

        for (index, binding) in self.bindings.iter().copied().enumerate() {
            if let Some(previous) = index
                .checked_sub(1)
                .and_then(|item| self.bindings.get(item))
                .copied()
            {
                if previous.proposition >= binding.proposition {
                    return Err(SignalCatalogError::BindingIdentityNotIncreasing {
                        previous: previous.proposition,
                        current: binding.proposition,
                    });
                }
            }
            let signal =
                self.signal(binding.signal)
                    .ok_or(SignalCatalogError::BindingTargetMissing {
                        proposition: binding.proposition,
                        signal: binding.signal,
                    })?;
            if signal.domain != SignalDomain::Boolean {
                return Err(SignalCatalogError::NonBooleanPropositionBinding {
                    proposition: binding.proposition,
                    signal: binding.signal,
                });
            }
        }
        Ok(())
    }

    /// Returns the number of declared signals.
    pub const fn signal_count(self) -> usize {
        self.signals.len()
    }

    /// Iterates declarations in strictly increasing identity order.
    pub const fn signals(self) -> SignalIter<'a> {
        SignalIter {
            catalog: self,
            index: 0,
        }
    }

    /// Returns direct bindings in strictly increasing proposition order.
    pub const fn bindings(self) -> &'a [PropositionBinding] {
        self.bindings
    }

    /// Looks up a declaration by signal identity.
    pub fn signal(self, id: SignalId) -> Option<SignalDeclaration<'a>> {
        let mut low = 0;
        let mut high = self.signals.len();
        while low < high {
            let middle = low + (high - low) / 2;
            let candidate = self
                .signals
                .get(middle)
                .expect("binary-search index is bounded by signal storage length");
            match candidate.id.cmp(&id) {
                core::cmp::Ordering::Less => low = middle + 1,
                core::cmp::Ordering::Greater => high = middle,
                core::cmp::Ordering::Equal => return Some(candidate),
            }
        }
        None
    }

    /// Resolves a direct proposition binding to its Boolean signal.
    pub fn signal_for_proposition(
        self,
        proposition: PropositionId,
    ) -> Option<SignalDeclaration<'a>> {
        self.bindings
            .binary_search_by_key(&proposition, |binding| binding.proposition)
            .ok()
            .and_then(|index| self.bindings.get(index))
            .and_then(|binding| self.signal(binding.signal))
    }

    /// Validates every proposition occurrence and returns a borrowed bound view.
    pub fn bind_formula<'formula>(
        self,
        formula: Formula<'formula>,
    ) -> Result<BoundFormula<'formula, 'a>, FormulaBindingError> {
        for node in formula.nodes() {
            if let NodeKind::Proposition { proposition } = node.kind {
                if self.signal_for_proposition(proposition).is_none() {
                    return Err(FormulaBindingError::MissingPropositionBinding { proposition });
                }
            }
        }
        Ok(BoundFormula {
            formula,
            catalog: self,
        })
    }
}

/// Iterator over borrowed signal declarations.
#[derive(Clone, Copy, Debug)]
pub struct SignalIter<'a> {
    catalog: SignalCatalog<'a>,
    index: usize,
}

impl<'a> Iterator for SignalIter<'a> {
    type Item = SignalDeclaration<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let item = self.catalog.signals.get(self.index)?;
        self.index += 1;
        Some(item)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = self.catalog.signal_count().saturating_sub(self.index);
        (remaining, Some(remaining))
    }
}

impl ExactSizeIterator for SignalIter<'_> {}
impl FusedIterator for SignalIter<'_> {}

/// A formula whose proposition occurrences all resolve through one catalog.
#[derive(Clone, Copy, Debug)]
pub struct BoundFormula<'formula, 'catalog> {
    formula: Formula<'formula>,
    catalog: SignalCatalog<'catalog>,
}

impl<'formula, 'catalog> BoundFormula<'formula, 'catalog> {
    /// Returns the validated formula.
    pub const fn formula(self) -> Formula<'formula> {
        self.formula
    }

    /// Returns the validated signal catalog.
    pub const fn catalog(self) -> SignalCatalog<'catalog> {
        self.catalog
    }

    /// Resolves a proposition to its declared Boolean signal.
    pub fn signal_for_proposition(
        self,
        proposition: PropositionId,
    ) -> Option<SignalDeclaration<'catalog>> {
        self.catalog.signal_for_proposition(proposition)
    }
}

/// Validation failure for a borrowed or owned signal catalog.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[non_exhaustive]
pub enum SignalCatalogError {
    /// The declaration population exceeds the v1 limit.
    SignalLimitExceeded { count: usize, limit: usize },
    /// The binding population exceeds the v1 limit.
    BindingLimitExceeded { count: usize, limit: usize },
    /// Caller-owned name-order scratch has fewer slots than declarations.
    NameOrderScratchTooSmall { provided: usize, required: usize },
    /// Signal identities are duplicated or not strictly increasing.
    SignalIdentityNotIncreasing {
        previous: SignalId,
        current: SignalId,
    },
    /// A signal name is empty.
    EmptySignalName { signal: SignalId },
    /// A signal name exceeds the v1 UTF-8 byte limit.
    SignalNameTooLong {
        signal: SignalId,
        length: usize,
        limit: usize,
    },
    /// Two declarations contain the same exact UTF-8 name bytes.
    DuplicateSignalName { first: SignalId, second: SignalId },
    /// Proposition binding identities are duplicated or not strictly increasing.
    BindingIdentityNotIncreasing {
        previous: PropositionId,
        current: PropositionId,
    },
    /// A proposition binding names no declared signal.
    BindingTargetMissing {
        proposition: PropositionId,
        signal: SignalId,
    },
    /// A direct MLTL proposition binding targets a non-Boolean signal.
    NonBooleanPropositionBinding {
        proposition: PropositionId,
        signal: SignalId,
    },
}

impl fmt::Display for SignalCatalogError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SignalLimitExceeded { count, limit } => {
                write!(
                    formatter,
                    "signal count {count} exceeds the {limit}-signal limit"
                )
            }
            Self::BindingLimitExceeded { count, limit } => write!(
                formatter,
                "proposition binding count {count} exceeds the {limit}-binding limit"
            ),
            Self::NameOrderScratchTooSmall { provided, required } => write!(
                formatter,
                "signal name-order scratch has {provided} slots but {required} are required"
            ),
            Self::SignalIdentityNotIncreasing { previous, current } => write!(
                formatter,
                "signal identity {} does not follow {}",
                current.0, previous.0
            ),
            Self::EmptySignalName { signal } => {
                write!(formatter, "signal {} has an empty name", signal.0)
            }
            Self::SignalNameTooLong {
                signal,
                length,
                limit,
            } => write!(
                formatter,
                "signal {} name has {length} UTF-8 bytes, exceeding the {limit}-byte limit",
                signal.0
            ),
            Self::DuplicateSignalName { first, second } => write!(
                formatter,
                "signals {} and {} have the same name",
                first.0, second.0
            ),
            Self::BindingIdentityNotIncreasing { previous, current } => write!(
                formatter,
                "proposition binding identity {} does not follow {}",
                current.0, previous.0
            ),
            Self::BindingTargetMissing {
                proposition,
                signal,
            } => write!(
                formatter,
                "proposition {} binding targets absent signal {}",
                proposition.0, signal.0
            ),
            Self::NonBooleanPropositionBinding {
                proposition,
                signal,
            } => write!(
                formatter,
                "proposition {} binding targets non-Boolean signal {}",
                proposition.0, signal.0
            ),
        }
    }
}

/// Construction failure for a bounded scalar signal domain.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[non_exhaustive]
pub enum SignalDomainError {
    /// Integer bounds are inverted.
    IntegerBoundsInverted { minimum: i64, maximum: i64 },
    /// Fixed-decimal coefficient bounds are inverted.
    DecimalBoundsInverted {
        minimum_coefficient: i64,
        maximum_coefficient: i64,
    },
    /// A fixed-decimal scale exceeds 18.
    DecimalScaleOutOfRange { scale: u8 },
}

impl fmt::Display for SignalDomainError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::IntegerBoundsInverted { minimum, maximum } => write!(
                formatter,
                "integer signal minimum {minimum} exceeds maximum {maximum}"
            ),
            Self::DecimalBoundsInverted {
                minimum_coefficient,
                maximum_coefficient,
            } => write!(
                formatter,
                "decimal signal coefficient minimum {minimum_coefficient} exceeds maximum {maximum_coefficient}"
            ),
            Self::DecimalScaleOutOfRange { scale } => {
                write!(formatter, "decimal signal scale {scale} exceeds 18")
            }
        }
    }
}

/// Formula-to-catalog binding failure.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[non_exhaustive]
pub enum FormulaBindingError {
    /// The first proposition occurrence without a direct catalog binding.
    MissingPropositionBinding { proposition: PropositionId },
}

impl fmt::Display for FormulaBindingError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingPropositionBinding { proposition } => write!(
                formatter,
                "formula proposition {} has no signal binding",
                proposition.0
            ),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Node, NodeId, SemanticProfile};

    // Trace: TC-027, TC-030, TC-033, FR-007-AC-1, FR-007-AC-3, NFR-001-AC-1
    #[test]
    fn borrowed_catalog_binding_neither_allocates_nor_retains_scratch() {
        let signals = [
            SignalDeclaration::new(SignalId(1), "ready", SignalDomain::Boolean),
            SignalDeclaration::new(
                SignalId(2),
                "count",
                SignalDomain::Integer(IntegerSignalDomain::new(0, 8).unwrap()),
            ),
        ];
        let bindings = [PropositionBinding::new(PropositionId(7), SignalId(1))];
        let mut scratch = [0; 2];
        let catalog = SignalCatalog::new(&signals, &bindings, &mut scratch).unwrap();

        // The validated view cannot borrow scratch: mutating it while retaining
        // the catalog is accepted by the borrow checker and changes no lookup.
        scratch.fill(u32::MAX);
        assert_eq!(catalog.signal(SignalId(2)).unwrap().name(), "count");

        let nodes = [Node::new(NodeKind::Proposition {
            proposition: PropositionId(7),
        })];
        let formula = Formula::new(SemanticProfile::ClosedTraceV1, NodeId(0), &nodes).unwrap();
        let bound = catalog.bind_formula(formula).unwrap();
        assert_eq!(
            bound
                .signal_for_proposition(PropositionId(7))
                .unwrap()
                .name(),
            "ready"
        );
    }
}
