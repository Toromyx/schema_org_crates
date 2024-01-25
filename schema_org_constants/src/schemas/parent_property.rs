/// <https://schema.org/parent>
pub const PARENT_PROPERTY_IRI_HTTP: &str = "http://schema.org/parent";
/// <https://schema.org/parent>
pub const PARENT_PROPERTY_IRI_HTTPS: &str = "https://schema.org/parent";
/// <https://schema.org/parent>
pub const PARENT_PROPERTY_LABEL: &str = "parent";
pub struct ParentPropertyIri;
impl PartialEq<&str> for ParentPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == PARENT_PROPERTY_IRI_HTTP || *other == PARENT_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<ParentPropertyIri> for &str {
	fn eq(&self, other: &ParentPropertyIri) -> bool {
		*self == PARENT_PROPERTY_IRI_HTTP || *self == PARENT_PROPERTY_IRI_HTTPS
	}
}
pub struct ParentPropertyIriOrLabel;
impl PartialEq<&str> for ParentPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ParentPropertyIri || *other == PARENT_PROPERTY_LABEL
	}
}
impl PartialEq<ParentPropertyIriOrLabel> for &str {
	fn eq(&self, other: &ParentPropertyIriOrLabel) -> bool {
		*self == ParentPropertyIri || *self == PARENT_PROPERTY_LABEL
	}
}
