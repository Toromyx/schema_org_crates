/// <https://schema.org/sibling>
pub const SIBLING_PROPERTY_IRI_HTTP: &str = "http://schema.org/sibling";
/// <https://schema.org/sibling>
pub const SIBLING_PROPERTY_IRI_HTTPS: &str = "https://schema.org/sibling";
/// <https://schema.org/sibling>
pub const SIBLING_PROPERTY_LABEL: &str = "sibling";
pub struct SiblingPropertyIri;
impl PartialEq<&str> for SiblingPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == SIBLING_PROPERTY_IRI_HTTP || *other == SIBLING_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<SiblingPropertyIri> for &str {
	fn eq(&self, other: &SiblingPropertyIri) -> bool {
		*self == SIBLING_PROPERTY_IRI_HTTP || *self == SIBLING_PROPERTY_IRI_HTTPS
	}
}
pub struct SiblingPropertyIriOrLabel;
impl PartialEq<&str> for SiblingPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == SiblingPropertyIri || *other == SIBLING_PROPERTY_LABEL
	}
}
impl PartialEq<SiblingPropertyIriOrLabel> for &str {
	fn eq(&self, other: &SiblingPropertyIriOrLabel) -> bool {
		*self == SiblingPropertyIri || *self == SIBLING_PROPERTY_LABEL
	}
}
