/// <https://schema.org/numChildren>
pub const NUM_CHILDREN_PROPERTY_IRI_HTTP: &str = "http://schema.org/numChildren";
/// <https://schema.org/numChildren>
pub const NUM_CHILDREN_PROPERTY_IRI_HTTPS: &str = "https://schema.org/numChildren";
/// <https://schema.org/numChildren>
pub const NUM_CHILDREN_PROPERTY_LABEL: &str = "numChildren";
pub struct NumChildrenPropertyIri;
impl PartialEq<&str> for NumChildrenPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == NUM_CHILDREN_PROPERTY_IRI_HTTP || *other == NUM_CHILDREN_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<NumChildrenPropertyIri> for &str {
	fn eq(&self, other: &NumChildrenPropertyIri) -> bool {
		*self == NUM_CHILDREN_PROPERTY_IRI_HTTP || *self == NUM_CHILDREN_PROPERTY_IRI_HTTPS
	}
}
pub struct NumChildrenPropertyIriOrLabel;
impl PartialEq<&str> for NumChildrenPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == NumChildrenPropertyIri || *other == NUM_CHILDREN_PROPERTY_LABEL
	}
}
impl PartialEq<NumChildrenPropertyIriOrLabel> for &str {
	fn eq(&self, other: &NumChildrenPropertyIriOrLabel) -> bool {
		*self == NumChildrenPropertyIri || *self == NUM_CHILDREN_PROPERTY_LABEL
	}
}
