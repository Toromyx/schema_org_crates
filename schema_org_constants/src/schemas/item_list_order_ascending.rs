/// <https://schema.org/ItemListOrderAscending>
pub const ITEM_LIST_ORDER_ASCENDING_IRI_HTTP: &str = "http://schema.org/ItemListOrderAscending";
/// <https://schema.org/ItemListOrderAscending>
pub const ITEM_LIST_ORDER_ASCENDING_IRI_HTTPS: &str = "https://schema.org/ItemListOrderAscending";
/// <https://schema.org/ItemListOrderAscending>
pub const ITEM_LIST_ORDER_ASCENDING_LABEL: &str = "ItemListOrderAscending";
pub struct ItemListOrderAscendingIri;
impl PartialEq<&str> for ItemListOrderAscendingIri {
	fn eq(&self, other: &&str) -> bool {
		*other == ITEM_LIST_ORDER_ASCENDING_IRI_HTTP
			|| *other == ITEM_LIST_ORDER_ASCENDING_IRI_HTTPS
	}
}
impl PartialEq<ItemListOrderAscendingIri> for &str {
	fn eq(&self, other: &ItemListOrderAscendingIri) -> bool {
		*self == ITEM_LIST_ORDER_ASCENDING_IRI_HTTP || *self == ITEM_LIST_ORDER_ASCENDING_IRI_HTTPS
	}
}
pub struct ItemListOrderAscendingIriOrLabel;
impl PartialEq<&str> for ItemListOrderAscendingIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ItemListOrderAscendingIri || *other == ITEM_LIST_ORDER_ASCENDING_LABEL
	}
}
impl PartialEq<ItemListOrderAscendingIriOrLabel> for &str {
	fn eq(&self, other: &ItemListOrderAscendingIriOrLabel) -> bool {
		*self == ItemListOrderAscendingIri || *self == ITEM_LIST_ORDER_ASCENDING_LABEL
	}
}
