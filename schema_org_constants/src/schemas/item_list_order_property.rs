/// <https://schema.org/itemListOrder>
pub const ITEM_LIST_ORDER_PROPERTY_IRI_HTTP: &str = "http://schema.org/itemListOrder";
/// <https://schema.org/itemListOrder>
pub const ITEM_LIST_ORDER_PROPERTY_IRI_HTTPS: &str = "https://schema.org/itemListOrder";
/// <https://schema.org/itemListOrder>
pub const ITEM_LIST_ORDER_PROPERTY_LABEL: &str = "itemListOrder";
pub struct ItemListOrderPropertyIri;
impl PartialEq<&str> for ItemListOrderPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == ITEM_LIST_ORDER_PROPERTY_IRI_HTTP || *other == ITEM_LIST_ORDER_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<ItemListOrderPropertyIri> for &str {
	fn eq(&self, other: &ItemListOrderPropertyIri) -> bool {
		*self == ITEM_LIST_ORDER_PROPERTY_IRI_HTTP || *self == ITEM_LIST_ORDER_PROPERTY_IRI_HTTPS
	}
}
pub struct ItemListOrderPropertyIriOrLabel;
impl PartialEq<&str> for ItemListOrderPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ItemListOrderPropertyIri || *other == ITEM_LIST_ORDER_PROPERTY_LABEL
	}
}
impl PartialEq<ItemListOrderPropertyIriOrLabel> for &str {
	fn eq(&self, other: &ItemListOrderPropertyIriOrLabel) -> bool {
		*self == ItemListOrderPropertyIri || *self == ITEM_LIST_ORDER_PROPERTY_LABEL
	}
}
