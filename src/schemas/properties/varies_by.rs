use super::*;
/// <https://schema.org/variesBy>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum VariesByProperty {
    #[cfg(any(
        any(feature = "defined-term-schema", feature = "pending-schema-section"),
        doc
    ))]
    DefinedTerm(DefinedTerm),
    #[cfg(any(any(feature = "text-schema", feature = "general-schema-section"), doc))]
    Text(Text),
}
