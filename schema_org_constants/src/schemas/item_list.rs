/// <https://schema.org/ItemList>
pub const ITEM_LIST_IRI_HTTP: &str = "http://schema.org/ItemList";
/// <https://schema.org/ItemList>
pub const ITEM_LIST_IRI_HTTPS: &str = "https://schema.org/ItemList";
/// <https://schema.org/ItemList>
pub const ITEM_LIST_LABEL: &str = "ItemList";
pub struct ItemListIri;
impl PartialEq<&str> for ItemListIri {
	fn eq(&self, other: &&str) -> bool {
		*other == ITEM_LIST_IRI_HTTP || *other == ITEM_LIST_IRI_HTTPS
	}
}
impl PartialEq<ItemListIri> for &str {
	fn eq(&self, other: &ItemListIri) -> bool {
		*self == ITEM_LIST_IRI_HTTP || *self == ITEM_LIST_IRI_HTTPS
	}
}
pub struct ItemListIriOrLabel;
impl PartialEq<&str> for ItemListIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ItemListIri || *other == ITEM_LIST_LABEL
	}
}
impl PartialEq<ItemListIriOrLabel> for &str {
	fn eq(&self, other: &ItemListIriOrLabel) -> bool {
		*self == ItemListIri || *self == ITEM_LIST_LABEL
	}
}
