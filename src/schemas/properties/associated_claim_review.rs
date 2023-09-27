use super::*;
/// <https://schema.org/associatedClaimReview>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum AssociatedClaimReviewProperty {
    #[cfg(any(
        any(feature = "review-schema", feature = "general-schema-section"),
        doc
    ))]
    Review(Review),
}
