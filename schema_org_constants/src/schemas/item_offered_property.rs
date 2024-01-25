/// <https://schema.org/itemOffered>
pub const ITEM_OFFERED_PROPERTY_IRI_HTTP: &str = "http://schema.org/itemOffered";
/// <https://schema.org/itemOffered>
pub const ITEM_OFFERED_PROPERTY_IRI_HTTPS: &str = "https://schema.org/itemOffered";
/// <https://schema.org/itemOffered>
pub const ITEM_OFFERED_PROPERTY_LABEL: &str = "itemOffered";
pub struct ItemOfferedPropertyIri;
impl PartialEq<&str> for ItemOfferedPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == ITEM_OFFERED_PROPERTY_IRI_HTTP || *other == ITEM_OFFERED_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<ItemOfferedPropertyIri> for &str {
	fn eq(&self, other: &ItemOfferedPropertyIri) -> bool {
		*self == ITEM_OFFERED_PROPERTY_IRI_HTTP || *self == ITEM_OFFERED_PROPERTY_IRI_HTTPS
	}
}
pub struct ItemOfferedPropertyIriOrLabel;
impl PartialEq<&str> for ItemOfferedPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ItemOfferedPropertyIri || *other == ITEM_OFFERED_PROPERTY_LABEL
	}
}
impl PartialEq<ItemOfferedPropertyIriOrLabel> for &str {
	fn eq(&self, other: &ItemOfferedPropertyIriOrLabel) -> bool {
		*self == ItemOfferedPropertyIri || *self == ITEM_OFFERED_PROPERTY_LABEL
	}
}
