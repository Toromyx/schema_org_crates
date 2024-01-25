/// <https://schema.org/ItemListOrderDescending>
pub const ITEM_LIST_ORDER_DESCENDING_IRI_HTTP: &str = "http://schema.org/ItemListOrderDescending";
/// <https://schema.org/ItemListOrderDescending>
pub const ITEM_LIST_ORDER_DESCENDING_IRI_HTTPS: &str = "https://schema.org/ItemListOrderDescending";
/// <https://schema.org/ItemListOrderDescending>
pub const ITEM_LIST_ORDER_DESCENDING_LABEL: &str = "ItemListOrderDescending";
pub struct ItemListOrderDescendingIri;
impl PartialEq<&str> for ItemListOrderDescendingIri {
	fn eq(&self, other: &&str) -> bool {
		*other == ITEM_LIST_ORDER_DESCENDING_IRI_HTTP
			|| *other == ITEM_LIST_ORDER_DESCENDING_IRI_HTTPS
	}
}
impl PartialEq<ItemListOrderDescendingIri> for &str {
	fn eq(&self, other: &ItemListOrderDescendingIri) -> bool {
		*self == ITEM_LIST_ORDER_DESCENDING_IRI_HTTP
			|| *self == ITEM_LIST_ORDER_DESCENDING_IRI_HTTPS
	}
}
pub struct ItemListOrderDescendingIriOrLabel;
impl PartialEq<&str> for ItemListOrderDescendingIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ItemListOrderDescendingIri || *other == ITEM_LIST_ORDER_DESCENDING_LABEL
	}
}
impl PartialEq<ItemListOrderDescendingIriOrLabel> for &str {
	fn eq(&self, other: &ItemListOrderDescendingIriOrLabel) -> bool {
		*self == ItemListOrderDescendingIri || *self == ITEM_LIST_ORDER_DESCENDING_LABEL
	}
}
