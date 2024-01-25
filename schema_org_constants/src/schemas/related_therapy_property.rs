/// <https://schema.org/relatedTherapy>
pub const RELATED_THERAPY_PROPERTY_IRI_HTTP: &str = "http://schema.org/relatedTherapy";
/// <https://schema.org/relatedTherapy>
pub const RELATED_THERAPY_PROPERTY_IRI_HTTPS: &str = "https://schema.org/relatedTherapy";
/// <https://schema.org/relatedTherapy>
pub const RELATED_THERAPY_PROPERTY_LABEL: &str = "relatedTherapy";
pub struct RelatedTherapyPropertyIri;
impl PartialEq<&str> for RelatedTherapyPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == RELATED_THERAPY_PROPERTY_IRI_HTTP || *other == RELATED_THERAPY_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<RelatedTherapyPropertyIri> for &str {
	fn eq(&self, other: &RelatedTherapyPropertyIri) -> bool {
		*self == RELATED_THERAPY_PROPERTY_IRI_HTTP || *self == RELATED_THERAPY_PROPERTY_IRI_HTTPS
	}
}
pub struct RelatedTherapyPropertyIriOrLabel;
impl PartialEq<&str> for RelatedTherapyPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == RelatedTherapyPropertyIri || *other == RELATED_THERAPY_PROPERTY_LABEL
	}
}
impl PartialEq<RelatedTherapyPropertyIriOrLabel> for &str {
	fn eq(&self, other: &RelatedTherapyPropertyIriOrLabel) -> bool {
		*self == RelatedTherapyPropertyIri || *self == RELATED_THERAPY_PROPERTY_LABEL
	}
}
