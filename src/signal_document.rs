use alloc::{string::String, vec, vec::Vec};
#[cfg(feature = "serde")]
use core::{fmt, marker::PhantomData};

use crate::{
    PropositionBinding, SignalCatalog, SignalCatalogError, SignalDeclaration, SignalDomain,
    SignalId,
};
#[cfg(feature = "serde")]
use crate::{MAX_SIGNAL_CATALOG_BINDINGS, MAX_SIGNAL_CATALOG_SIGNALS};

/// Version of the serialized signal-catalog document.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum SignalCatalogSchemaVersion {
    /// Initial tl-syntax signal-catalog schema.
    #[cfg_attr(feature = "serde", serde(rename = "tl-syntax.signal-catalog/v1"))]
    V1,
}

impl SignalCatalogSchemaVersion {
    /// Returns the stable wire identifier.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::V1 => "tl-syntax.signal-catalog/v1",
        }
    }
}

/// One owned named signal declaration.
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(deny_unknown_fields))]
pub struct OwnedSignalDeclaration {
    id: SignalId,
    name: String,
    domain: SignalDomain,
}

impl OwnedSignalDeclaration {
    /// Constructs an unvalidated owned declaration.
    pub fn new(id: SignalId, name: String, domain: SignalDomain) -> Self {
        Self { id, name, domain }
    }

    /// Borrows this declaration without allocation.
    pub fn as_borrowed(&self) -> SignalDeclaration<'_> {
        SignalDeclaration::new(self.id, &self.name, self.domain)
    }
}

/// Owned, versioned signal-catalog exchange document.
#[derive(Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(try_from = "SignalCatalogDocumentWire"))]
pub struct SignalCatalogDocument {
    schema_version: SignalCatalogSchemaVersion,
    signals: Vec<OwnedSignalDeclaration>,
    bindings: Vec<PropositionBinding>,
}

#[cfg(feature = "serde")]
#[derive(serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct SignalCatalogDocumentWire {
    schema_version: SignalCatalogSchemaVersion,
    #[serde(deserialize_with = "deserialize_signals")]
    signals: Vec<OwnedSignalDeclaration>,
    #[serde(deserialize_with = "deserialize_bindings")]
    bindings: Vec<PropositionBinding>,
}

#[cfg(feature = "serde")]
impl TryFrom<SignalCatalogDocumentWire> for SignalCatalogDocument {
    type Error = SignalCatalogError;

    fn try_from(wire: SignalCatalogDocumentWire) -> Result<Self, Self::Error> {
        Self::from_parts(wire.schema_version, wire.signals, wire.bindings)
    }
}

impl SignalCatalogDocument {
    /// Constructs and validates a v1 signal-catalog document.
    pub fn new(
        signals: Vec<OwnedSignalDeclaration>,
        bindings: Vec<PropositionBinding>,
    ) -> Result<Self, SignalCatalogError> {
        Self::from_parts(SignalCatalogSchemaVersion::V1, signals, bindings)
    }

    fn from_parts(
        schema_version: SignalCatalogSchemaVersion,
        signals: Vec<OwnedSignalDeclaration>,
        bindings: Vec<PropositionBinding>,
    ) -> Result<Self, SignalCatalogError> {
        let document = Self {
            schema_version,
            signals,
            bindings,
        };
        document.validate()?;
        Ok(document)
    }

    /// Copies a validated borrowed catalog into a v1 owned document.
    pub fn from_catalog(catalog: SignalCatalog<'_>) -> Result<Self, SignalCatalogError> {
        Self::new(
            catalog
                .signals()
                .map(|signal| {
                    OwnedSignalDeclaration::new(signal.id(), signal.name().into(), signal.domain())
                })
                .collect(),
            catalog.bindings().to_vec(),
        )
    }

    /// Validates and returns an allocation-free view over this document.
    pub fn validate(&self) -> Result<SignalCatalog<'_>, SignalCatalogError> {
        let mut name_order_scratch = vec![0; self.signals.len()];
        SignalCatalog::from_owned(&self.signals, &self.bindings, &mut name_order_scratch)
    }

    /// Returns the wire schema version.
    pub const fn schema_version(&self) -> SignalCatalogSchemaVersion {
        self.schema_version
    }

    /// Returns owned signal declarations in strictly increasing identity order.
    pub fn signals(&self) -> &[OwnedSignalDeclaration] {
        &self.signals
    }

    /// Returns direct proposition bindings in strictly increasing identity order.
    pub fn bindings(&self) -> &[PropositionBinding] {
        &self.bindings
    }
}

#[cfg(feature = "serde")]
fn deserialize_signals<'de, D>(deserializer: D) -> Result<Vec<OwnedSignalDeclaration>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    deserialize_bounded_vec(
        deserializer,
        MAX_SIGNAL_CATALOG_SIGNALS,
        "signal declarations",
    )
}

#[cfg(feature = "serde")]
fn deserialize_bindings<'de, D>(deserializer: D) -> Result<Vec<PropositionBinding>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    deserialize_bounded_vec(
        deserializer,
        MAX_SIGNAL_CATALOG_BINDINGS,
        "proposition bindings",
    )
}

#[cfg(feature = "serde")]
fn deserialize_bounded_vec<'de, D, T>(
    deserializer: D,
    limit: usize,
    description: &'static str,
) -> Result<Vec<T>, D::Error>
where
    D: serde::Deserializer<'de>,
    T: serde::Deserialize<'de>,
{
    struct BoundedVisitor<T> {
        limit: usize,
        description: &'static str,
        marker: PhantomData<T>,
    }

    impl<'de, T> serde::de::Visitor<'de> for BoundedVisitor<T>
    where
        T: serde::Deserialize<'de>,
    {
        type Value = Vec<T>;

        fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(formatter, "at most {} {}", self.limit, self.description)
        }

        fn visit_seq<A>(self, mut sequence: A) -> Result<Self::Value, A::Error>
        where
            A: serde::de::SeqAccess<'de>,
        {
            if sequence.size_hint().is_some_and(|size| size > self.limit) {
                return Err(serde::de::Error::custom(format_args!(
                    "{} exceed the {}-item wire limit",
                    self.description, self.limit
                )));
            }
            let mut items = Vec::with_capacity(sequence.size_hint().unwrap_or(0).min(self.limit));
            while let Some(item) = sequence.next_element()? {
                if items.len() == self.limit {
                    return Err(serde::de::Error::custom(format_args!(
                        "{} exceed the {}-item wire limit",
                        self.description, self.limit
                    )));
                }
                items.push(item);
            }
            Ok(items)
        }
    }

    deserializer.deserialize_seq(BoundedVisitor {
        limit,
        description,
        marker: PhantomData,
    })
}
