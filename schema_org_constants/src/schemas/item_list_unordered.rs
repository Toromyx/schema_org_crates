/// <https://schema.org/ItemListUnordered>
pub const ITEM_LIST_UNORDERED_IRI_HTTP: &str = "http://schema.org/ItemListUnordered";
/// <https://schema.org/ItemListUnordered>
pub const ITEM_LIST_UNORDERED_IRI_HTTPS: &str = "https://schema.org/ItemListUnordered";
/// <https://schema.org/ItemListUnordered>
pub const ITEM_LIST_UNORDERED_LABEL: &str = "ItemListUnordered";
pub struct ItemListUnorderedIri;
impl PartialEq<&str> for ItemListUnorderedIri {
	fn eq(&self, other: &&str) -> bool {
		*other == ITEM_LIST_UNORDERED_IRI_HTTP || *other == ITEM_LIST_UNORDERED_IRI_HTTPS
	}
}
impl PartialEq<ItemListUnorderedIri> for &str {
	fn eq(&self, other: &ItemListUnorderedIri) -> bool {
		*self == ITEM_LIST_UNORDERED_IRI_HTTP || *self == ITEM_LIST_UNORDERED_IRI_HTTPS
	}
}
pub struct ItemListUnorderedIriOrLabel;
impl PartialEq<&str> for ItemListUnorderedIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ItemListUnorderedIri || *other == ITEM_LIST_UNORDERED_LABEL
	}
}
impl PartialEq<ItemListUnorderedIriOrLabel> for &str {
	fn eq(&self, other: &ItemListUnorderedIriOrLabel) -> bool {
		*self == ItemListUnorderedIri || *self == ITEM_LIST_UNORDERED_LABEL
	}
}
