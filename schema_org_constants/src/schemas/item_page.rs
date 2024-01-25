/// <https://schema.org/ItemPage>
pub const ITEM_PAGE_IRI_HTTP: &str = "http://schema.org/ItemPage";
/// <https://schema.org/ItemPage>
pub const ITEM_PAGE_IRI_HTTPS: &str = "https://schema.org/ItemPage";
/// <https://schema.org/ItemPage>
pub const ITEM_PAGE_LABEL: &str = "ItemPage";
pub struct ItemPageIri;
impl PartialEq<&str> for ItemPageIri {
	fn eq(&self, other: &&str) -> bool {
		*other == ITEM_PAGE_IRI_HTTP || *other == ITEM_PAGE_IRI_HTTPS
	}
}
impl PartialEq<ItemPageIri> for &str {
	fn eq(&self, other: &ItemPageIri) -> bool {
		*self == ITEM_PAGE_IRI_HTTP || *self == ITEM_PAGE_IRI_HTTPS
	}
}
pub struct ItemPageIriOrLabel;
impl PartialEq<&str> for ItemPageIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ItemPageIri || *other == ITEM_PAGE_LABEL
	}
}
impl PartialEq<ItemPageIriOrLabel> for &str {
	fn eq(&self, other: &ItemPageIriOrLabel) -> bool {
		*self == ItemPageIri || *self == ITEM_PAGE_LABEL
	}
}
