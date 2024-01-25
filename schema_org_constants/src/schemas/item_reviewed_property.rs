/// <https://schema.org/itemReviewed>
pub const ITEM_REVIEWED_PROPERTY_IRI_HTTP: &str = "http://schema.org/itemReviewed";
/// <https://schema.org/itemReviewed>
pub const ITEM_REVIEWED_PROPERTY_IRI_HTTPS: &str = "https://schema.org/itemReviewed";
/// <https://schema.org/itemReviewed>
pub const ITEM_REVIEWED_PROPERTY_LABEL: &str = "itemReviewed";
pub struct ItemReviewedPropertyIri;
impl PartialEq<&str> for ItemReviewedPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == ITEM_REVIEWED_PROPERTY_IRI_HTTP || *other == ITEM_REVIEWED_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<ItemReviewedPropertyIri> for &str {
	fn eq(&self, other: &ItemReviewedPropertyIri) -> bool {
		*self == ITEM_REVIEWED_PROPERTY_IRI_HTTP || *self == ITEM_REVIEWED_PROPERTY_IRI_HTTPS
	}
}
pub struct ItemReviewedPropertyIriOrLabel;
impl PartialEq<&str> for ItemReviewedPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ItemReviewedPropertyIri || *other == ITEM_REVIEWED_PROPERTY_LABEL
	}
}
impl PartialEq<ItemReviewedPropertyIriOrLabel> for &str {
	fn eq(&self, other: &ItemReviewedPropertyIriOrLabel) -> bool {
		*self == ItemReviewedPropertyIri || *self == ITEM_REVIEWED_PROPERTY_LABEL
	}
}
