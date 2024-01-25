/// <https://schema.org/itemLocation>
pub const ITEM_LOCATION_PROPERTY_IRI_HTTP: &str = "http://schema.org/itemLocation";
/// <https://schema.org/itemLocation>
pub const ITEM_LOCATION_PROPERTY_IRI_HTTPS: &str = "https://schema.org/itemLocation";
/// <https://schema.org/itemLocation>
pub const ITEM_LOCATION_PROPERTY_LABEL: &str = "itemLocation";
pub struct ItemLocationPropertyIri;
impl PartialEq<&str> for ItemLocationPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == ITEM_LOCATION_PROPERTY_IRI_HTTP || *other == ITEM_LOCATION_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<ItemLocationPropertyIri> for &str {
	fn eq(&self, other: &ItemLocationPropertyIri) -> bool {
		*self == ITEM_LOCATION_PROPERTY_IRI_HTTP || *self == ITEM_LOCATION_PROPERTY_IRI_HTTPS
	}
}
pub struct ItemLocationPropertyIriOrLabel;
impl PartialEq<&str> for ItemLocationPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ItemLocationPropertyIri || *other == ITEM_LOCATION_PROPERTY_LABEL
	}
}
impl PartialEq<ItemLocationPropertyIriOrLabel> for &str {
	fn eq(&self, other: &ItemLocationPropertyIriOrLabel) -> bool {
		*self == ItemLocationPropertyIri || *self == ITEM_LOCATION_PROPERTY_LABEL
	}
}
