use super::*;
/// <https://schema.org/originAddress>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum OriginAddressProperty {
    #[cfg(any(
        any(feature = "postal-address-schema", feature = "general-schema-section"),
        doc
    ))]
    PostalAddress(PostalAddress),
}
