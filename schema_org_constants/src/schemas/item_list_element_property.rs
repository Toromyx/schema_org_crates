/// <https://schema.org/itemListElement>
pub const ITEM_LIST_ELEMENT_PROPERTY_IRI_HTTP: &str = "http://schema.org/itemListElement";
/// <https://schema.org/itemListElement>
pub const ITEM_LIST_ELEMENT_PROPERTY_IRI_HTTPS: &str = "https://schema.org/itemListElement";
/// <https://schema.org/itemListElement>
pub const ITEM_LIST_ELEMENT_PROPERTY_LABEL: &str = "itemListElement";
pub struct ItemListElementPropertyIri;
impl PartialEq<&str> for ItemListElementPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == ITEM_LIST_ELEMENT_PROPERTY_IRI_HTTP
			|| *other == ITEM_LIST_ELEMENT_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<ItemListElementPropertyIri> for &str {
	fn eq(&self, other: &ItemListElementPropertyIri) -> bool {
		*self == ITEM_LIST_ELEMENT_PROPERTY_IRI_HTTP
			|| *self == ITEM_LIST_ELEMENT_PROPERTY_IRI_HTTPS
	}
}
pub struct ItemListElementPropertyIriOrLabel;
impl PartialEq<&str> for ItemListElementPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ItemListElementPropertyIri || *other == ITEM_LIST_ELEMENT_PROPERTY_LABEL
	}
}
impl PartialEq<ItemListElementPropertyIriOrLabel> for &str {
	fn eq(&self, other: &ItemListElementPropertyIriOrLabel) -> bool {
		*self == ItemListElementPropertyIri || *self == ITEM_LIST_ELEMENT_PROPERTY_LABEL
	}
}
