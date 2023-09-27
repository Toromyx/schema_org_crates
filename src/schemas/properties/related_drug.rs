use super::*;
/// <https://schema.org/relatedDrug>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum RelatedDrugProperty {
    #[cfg(any(
        any(feature = "drug-schema", feature = "health-lifesci-schema-section"),
        doc
    ))]
    Drug(Drug),
}
