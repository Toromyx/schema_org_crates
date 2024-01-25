/// <https://schema.org/itemShipped>
pub const ITEM_SHIPPED_PROPERTY_IRI_HTTP: &str = "http://schema.org/itemShipped";
/// <https://schema.org/itemShipped>
pub const ITEM_SHIPPED_PROPERTY_IRI_HTTPS: &str = "https://schema.org/itemShipped";
/// <https://schema.org/itemShipped>
pub const ITEM_SHIPPED_PROPERTY_LABEL: &str = "itemShipped";
pub struct ItemShippedPropertyIri;
impl PartialEq<&str> for ItemShippedPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == ITEM_SHIPPED_PROPERTY_IRI_HTTP || *other == ITEM_SHIPPED_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<ItemShippedPropertyIri> for &str {
	fn eq(&self, other: &ItemShippedPropertyIri) -> bool {
		*self == ITEM_SHIPPED_PROPERTY_IRI_HTTP || *self == ITEM_SHIPPED_PROPERTY_IRI_HTTPS
	}
}
pub struct ItemShippedPropertyIriOrLabel;
impl PartialEq<&str> for ItemShippedPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ItemShippedPropertyIri || *other == ITEM_SHIPPED_PROPERTY_LABEL
	}
}
impl PartialEq<ItemShippedPropertyIriOrLabel> for &str {
	fn eq(&self, other: &ItemShippedPropertyIriOrLabel) -> bool {
		*self == ItemShippedPropertyIri || *self == ITEM_SHIPPED_PROPERTY_LABEL
	}
}
