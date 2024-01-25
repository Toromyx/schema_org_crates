/// <https://schema.org/relatedAnatomy>
pub const RELATED_ANATOMY_PROPERTY_IRI_HTTP: &str = "http://schema.org/relatedAnatomy";
/// <https://schema.org/relatedAnatomy>
pub const RELATED_ANATOMY_PROPERTY_IRI_HTTPS: &str = "https://schema.org/relatedAnatomy";
/// <https://schema.org/relatedAnatomy>
pub const RELATED_ANATOMY_PROPERTY_LABEL: &str = "relatedAnatomy";
pub struct RelatedAnatomyPropertyIri;
impl PartialEq<&str> for RelatedAnatomyPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == RELATED_ANATOMY_PROPERTY_IRI_HTTP || *other == RELATED_ANATOMY_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<RelatedAnatomyPropertyIri> for &str {
	fn eq(&self, other: &RelatedAnatomyPropertyIri) -> bool {
		*self == RELATED_ANATOMY_PROPERTY_IRI_HTTP || *self == RELATED_ANATOMY_PROPERTY_IRI_HTTPS
	}
}
pub struct RelatedAnatomyPropertyIriOrLabel;
impl PartialEq<&str> for RelatedAnatomyPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == RelatedAnatomyPropertyIri || *other == RELATED_ANATOMY_PROPERTY_LABEL
	}
}
impl PartialEq<RelatedAnatomyPropertyIriOrLabel> for &str {
	fn eq(&self, other: &RelatedAnatomyPropertyIriOrLabel) -> bool {
		*self == RelatedAnatomyPropertyIri || *self == RELATED_ANATOMY_PROPERTY_LABEL
	}
}
