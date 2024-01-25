/// <https://schema.org/reviewAspect>
pub const REVIEW_ASPECT_PROPERTY_IRI_HTTP: &str = "http://schema.org/reviewAspect";
/// <https://schema.org/reviewAspect>
pub const REVIEW_ASPECT_PROPERTY_IRI_HTTPS: &str = "https://schema.org/reviewAspect";
/// <https://schema.org/reviewAspect>
pub const REVIEW_ASPECT_PROPERTY_LABEL: &str = "reviewAspect";
pub struct ReviewAspectPropertyIri;
impl PartialEq<&str> for ReviewAspectPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == REVIEW_ASPECT_PROPERTY_IRI_HTTP || *other == REVIEW_ASPECT_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<ReviewAspectPropertyIri> for &str {
	fn eq(&self, other: &ReviewAspectPropertyIri) -> bool {
		*self == REVIEW_ASPECT_PROPERTY_IRI_HTTP || *self == REVIEW_ASPECT_PROPERTY_IRI_HTTPS
	}
}
pub struct ReviewAspectPropertyIriOrLabel;
impl PartialEq<&str> for ReviewAspectPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ReviewAspectPropertyIri || *other == REVIEW_ASPECT_PROPERTY_LABEL
	}
}
impl PartialEq<ReviewAspectPropertyIriOrLabel> for &str {
	fn eq(&self, other: &ReviewAspectPropertyIriOrLabel) -> bool {
		*self == ReviewAspectPropertyIri || *self == REVIEW_ASPECT_PROPERTY_LABEL
	}
}
