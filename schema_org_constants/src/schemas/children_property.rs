/// <https://schema.org/children>
pub const CHILDREN_PROPERTY_IRI_HTTP: &str = "http://schema.org/children";
/// <https://schema.org/children>
pub const CHILDREN_PROPERTY_IRI_HTTPS: &str = "https://schema.org/children";
/// <https://schema.org/children>
pub const CHILDREN_PROPERTY_LABEL: &str = "children";
pub struct ChildrenPropertyIri;
impl PartialEq<&str> for ChildrenPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == CHILDREN_PROPERTY_IRI_HTTP || *other == CHILDREN_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<ChildrenPropertyIri> for &str {
	fn eq(&self, other: &ChildrenPropertyIri) -> bool {
		*self == CHILDREN_PROPERTY_IRI_HTTP || *self == CHILDREN_PROPERTY_IRI_HTTPS
	}
}
pub struct ChildrenPropertyIriOrLabel;
impl PartialEq<&str> for ChildrenPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ChildrenPropertyIri || *other == CHILDREN_PROPERTY_LABEL
	}
}
impl PartialEq<ChildrenPropertyIriOrLabel> for &str {
	fn eq(&self, other: &ChildrenPropertyIriOrLabel) -> bool {
		*self == ChildrenPropertyIri || *self == CHILDREN_PROPERTY_LABEL
	}
}
