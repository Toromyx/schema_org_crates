/// <https://schema.org/orderedItem>
pub const ORDERED_ITEM_PROPERTY_IRI_HTTP: &str = "http://schema.org/orderedItem";
/// <https://schema.org/orderedItem>
pub const ORDERED_ITEM_PROPERTY_IRI_HTTPS: &str = "https://schema.org/orderedItem";
/// <https://schema.org/orderedItem>
pub const ORDERED_ITEM_PROPERTY_LABEL: &str = "orderedItem";
pub struct OrderedItemPropertyIri;
impl PartialEq<&str> for OrderedItemPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == ORDERED_ITEM_PROPERTY_IRI_HTTP || *other == ORDERED_ITEM_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<OrderedItemPropertyIri> for &str {
	fn eq(&self, other: &OrderedItemPropertyIri) -> bool {
		*self == ORDERED_ITEM_PROPERTY_IRI_HTTP || *self == ORDERED_ITEM_PROPERTY_IRI_HTTPS
	}
}
pub struct OrderedItemPropertyIriOrLabel;
impl PartialEq<&str> for OrderedItemPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == OrderedItemPropertyIri || *other == ORDERED_ITEM_PROPERTY_LABEL
	}
}
impl PartialEq<OrderedItemPropertyIriOrLabel> for &str {
	fn eq(&self, other: &OrderedItemPropertyIriOrLabel) -> bool {
		*self == OrderedItemPropertyIri || *self == ORDERED_ITEM_PROPERTY_LABEL
	}
}
