/// <https://schema.org/item>
pub const ITEM_PROPERTY_IRI_HTTP: &str = "http://schema.org/item";
/// <https://schema.org/item>
pub const ITEM_PROPERTY_IRI_HTTPS: &str = "https://schema.org/item";
/// <https://schema.org/item>
pub const ITEM_PROPERTY_LABEL: &str = "item";
pub struct ItemPropertyIri;
impl PartialEq<&str> for ItemPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == ITEM_PROPERTY_IRI_HTTP || *other == ITEM_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<ItemPropertyIri> for &str {
	fn eq(&self, other: &ItemPropertyIri) -> bool {
		*self == ITEM_PROPERTY_IRI_HTTP || *self == ITEM_PROPERTY_IRI_HTTPS
	}
}
pub struct ItemPropertyIriOrLabel;
impl PartialEq<&str> for ItemPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ItemPropertyIri || *other == ITEM_PROPERTY_LABEL
	}
}
impl PartialEq<ItemPropertyIriOrLabel> for &str {
	fn eq(&self, other: &ItemPropertyIriOrLabel) -> bool {
		*self == ItemPropertyIri || *self == ITEM_PROPERTY_LABEL
	}
}
