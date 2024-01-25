/// <https://schema.org/documentation>
pub const DOCUMENTATION_PROPERTY_IRI_HTTP: &str = "http://schema.org/documentation";
/// <https://schema.org/documentation>
pub const DOCUMENTATION_PROPERTY_IRI_HTTPS: &str = "https://schema.org/documentation";
/// <https://schema.org/documentation>
pub const DOCUMENTATION_PROPERTY_LABEL: &str = "documentation";
pub struct DocumentationPropertyIri;
impl PartialEq<&str> for DocumentationPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == DOCUMENTATION_PROPERTY_IRI_HTTP || *other == DOCUMENTATION_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<DocumentationPropertyIri> for &str {
	fn eq(&self, other: &DocumentationPropertyIri) -> bool {
		*self == DOCUMENTATION_PROPERTY_IRI_HTTP || *self == DOCUMENTATION_PROPERTY_IRI_HTTPS
	}
}
pub struct DocumentationPropertyIriOrLabel;
impl PartialEq<&str> for DocumentationPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == DocumentationPropertyIri || *other == DOCUMENTATION_PROPERTY_LABEL
	}
}
impl PartialEq<DocumentationPropertyIriOrLabel> for &str {
	fn eq(&self, other: &DocumentationPropertyIriOrLabel) -> bool {
		*self == DocumentationPropertyIri || *self == DOCUMENTATION_PROPERTY_LABEL
	}
}
