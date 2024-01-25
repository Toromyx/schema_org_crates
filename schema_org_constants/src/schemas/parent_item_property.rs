/// <https://schema.org/parentItem>
pub const PARENT_ITEM_PROPERTY_IRI_HTTP: &str = "http://schema.org/parentItem";
/// <https://schema.org/parentItem>
pub const PARENT_ITEM_PROPERTY_IRI_HTTPS: &str = "https://schema.org/parentItem";
/// <https://schema.org/parentItem>
pub const PARENT_ITEM_PROPERTY_LABEL: &str = "parentItem";
pub struct ParentItemPropertyIri;
impl PartialEq<&str> for ParentItemPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == PARENT_ITEM_PROPERTY_IRI_HTTP || *other == PARENT_ITEM_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<ParentItemPropertyIri> for &str {
	fn eq(&self, other: &ParentItemPropertyIri) -> bool {
		*self == PARENT_ITEM_PROPERTY_IRI_HTTP || *self == PARENT_ITEM_PROPERTY_IRI_HTTPS
	}
}
pub struct ParentItemPropertyIriOrLabel;
impl PartialEq<&str> for ParentItemPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ParentItemPropertyIri || *other == PARENT_ITEM_PROPERTY_LABEL
	}
}
impl PartialEq<ParentItemPropertyIriOrLabel> for &str {
	fn eq(&self, other: &ParentItemPropertyIriOrLabel) -> bool {
		*self == ParentItemPropertyIri || *self == PARENT_ITEM_PROPERTY_LABEL
	}
}
