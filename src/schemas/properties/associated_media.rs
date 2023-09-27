use super::*;
/// <https://schema.org/associatedMedia>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum AssociatedMediaProperty {
    #[cfg(any(
        any(feature = "media-object-schema", feature = "general-schema-section"),
        doc
    ))]
    MediaObject(MediaObject),
}
