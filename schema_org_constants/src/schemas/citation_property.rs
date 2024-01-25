/// <https://schema.org/citation>
pub const CITATION_PROPERTY_IRI_HTTP: &str = "http://schema.org/citation";
/// <https://schema.org/citation>
pub const CITATION_PROPERTY_IRI_HTTPS: &str = "https://schema.org/citation";
/// <https://schema.org/citation>
pub const CITATION_PROPERTY_LABEL: &str = "citation";
pub struct CitationPropertyIri;
impl PartialEq<&str> for CitationPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == CITATION_PROPERTY_IRI_HTTP || *other == CITATION_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<CitationPropertyIri> for &str {
	fn eq(&self, other: &CitationPropertyIri) -> bool {
		*self == CITATION_PROPERTY_IRI_HTTP || *self == CITATION_PROPERTY_IRI_HTTPS
	}
}
pub struct CitationPropertyIriOrLabel;
impl PartialEq<&str> for CitationPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == CitationPropertyIri || *other == CITATION_PROPERTY_LABEL
	}
}
impl PartialEq<CitationPropertyIriOrLabel> for &str {
	fn eq(&self, other: &CitationPropertyIriOrLabel) -> bool {
		*self == CitationPropertyIri || *self == CITATION_PROPERTY_LABEL
	}
}
