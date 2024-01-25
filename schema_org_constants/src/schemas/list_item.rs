/// <https://schema.org/ListItem>
pub const LIST_ITEM_IRI_HTTP: &str = "http://schema.org/ListItem";
/// <https://schema.org/ListItem>
pub const LIST_ITEM_IRI_HTTPS: &str = "https://schema.org/ListItem";
/// <https://schema.org/ListItem>
pub const LIST_ITEM_LABEL: &str = "ListItem";
pub struct ListItemIri;
impl PartialEq<&str> for ListItemIri {
	fn eq(&self, other: &&str) -> bool {
		*other == LIST_ITEM_IRI_HTTP || *other == LIST_ITEM_IRI_HTTPS
	}
}
impl PartialEq<ListItemIri> for &str {
	fn eq(&self, other: &ListItemIri) -> bool {
		*self == LIST_ITEM_IRI_HTTP || *self == LIST_ITEM_IRI_HTTPS
	}
}
pub struct ListItemIriOrLabel;
impl PartialEq<&str> for ListItemIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ListItemIri || *other == LIST_ITEM_LABEL
	}
}
impl PartialEq<ListItemIriOrLabel> for &str {
	fn eq(&self, other: &ListItemIriOrLabel) -> bool {
		*self == ListItemIri || *self == LIST_ITEM_LABEL
	}
}
