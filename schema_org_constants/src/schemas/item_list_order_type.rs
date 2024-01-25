/// <https://schema.org/ItemListOrderType>
pub const ITEM_LIST_ORDER_TYPE_IRI_HTTP: &str = "http://schema.org/ItemListOrderType";
/// <https://schema.org/ItemListOrderType>
pub const ITEM_LIST_ORDER_TYPE_IRI_HTTPS: &str = "https://schema.org/ItemListOrderType";
/// <https://schema.org/ItemListOrderType>
pub const ITEM_LIST_ORDER_TYPE_LABEL: &str = "ItemListOrderType";
pub struct ItemListOrderTypeIri;
impl PartialEq<&str> for ItemListOrderTypeIri {
	fn eq(&self, other: &&str) -> bool {
		*other == ITEM_LIST_ORDER_TYPE_IRI_HTTP || *other == ITEM_LIST_ORDER_TYPE_IRI_HTTPS
	}
}
impl PartialEq<ItemListOrderTypeIri> for &str {
	fn eq(&self, other: &ItemListOrderTypeIri) -> bool {
		*self == ITEM_LIST_ORDER_TYPE_IRI_HTTP || *self == ITEM_LIST_ORDER_TYPE_IRI_HTTPS
	}
}
pub struct ItemListOrderTypeIriOrLabel;
impl PartialEq<&str> for ItemListOrderTypeIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ItemListOrderTypeIri || *other == ITEM_LIST_ORDER_TYPE_LABEL
	}
}
impl PartialEq<ItemListOrderTypeIriOrLabel> for &str {
	fn eq(&self, other: &ItemListOrderTypeIriOrLabel) -> bool {
		*self == ItemListOrderTypeIri || *self == ITEM_LIST_ORDER_TYPE_LABEL
	}
}
