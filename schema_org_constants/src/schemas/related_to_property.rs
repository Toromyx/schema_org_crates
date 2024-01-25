/// <https://schema.org/relatedTo>
pub const RELATED_TO_PROPERTY_IRI_HTTP: &str = "http://schema.org/relatedTo";
/// <https://schema.org/relatedTo>
pub const RELATED_TO_PROPERTY_IRI_HTTPS: &str = "https://schema.org/relatedTo";
/// <https://schema.org/relatedTo>
pub const RELATED_TO_PROPERTY_LABEL: &str = "relatedTo";
pub struct RelatedToPropertyIri;
impl PartialEq<&str> for RelatedToPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == RELATED_TO_PROPERTY_IRI_HTTP || *other == RELATED_TO_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<RelatedToPropertyIri> for &str {
	fn eq(&self, other: &RelatedToPropertyIri) -> bool {
		*self == RELATED_TO_PROPERTY_IRI_HTTP || *self == RELATED_TO_PROPERTY_IRI_HTTPS
	}
}
pub struct RelatedToPropertyIriOrLabel;
impl PartialEq<&str> for RelatedToPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == RelatedToPropertyIri || *other == RELATED_TO_PROPERTY_LABEL
	}
}
impl PartialEq<RelatedToPropertyIriOrLabel> for &str {
	fn eq(&self, other: &RelatedToPropertyIriOrLabel) -> bool {
		*self == RelatedToPropertyIri || *self == RELATED_TO_PROPERTY_LABEL
	}
}
