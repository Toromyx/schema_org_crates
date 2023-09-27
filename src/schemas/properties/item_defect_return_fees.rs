use super::*;
/// <https://schema.org/itemDefectReturnFees>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum ItemDefectReturnFeesProperty {
    #[cfg(any(
        any(
            feature = "return-fees-enumeration-schema",
            feature = "pending-schema-section"
        ),
        doc
    ))]
    ReturnFeesEnumeration(ReturnFeesEnumeration),
}
