/// <https://schema.org/reviewBody>
pub const REVIEW_BODY_PROPERTY_IRI_HTTP: &str = "http://schema.org/reviewBody";
/// <https://schema.org/reviewBody>
pub const REVIEW_BODY_PROPERTY_IRI_HTTPS: &str = "https://schema.org/reviewBody";
/// <https://schema.org/reviewBody>
pub const REVIEW_BODY_PROPERTY_LABEL: &str = "reviewBody";
pub struct ReviewBodyPropertyIri;
impl PartialEq<&str> for ReviewBodyPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == REVIEW_BODY_PROPERTY_IRI_HTTP || *other == REVIEW_BODY_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<ReviewBodyPropertyIri> for &str {
	fn eq(&self, other: &ReviewBodyPropertyIri) -> bool {
		*self == REVIEW_BODY_PROPERTY_IRI_HTTP || *self == REVIEW_BODY_PROPERTY_IRI_HTTPS
	}
}
pub struct ReviewBodyPropertyIriOrLabel;
impl PartialEq<&str> for ReviewBodyPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ReviewBodyPropertyIri || *other == REVIEW_BODY_PROPERTY_LABEL
	}
}
impl PartialEq<ReviewBodyPropertyIriOrLabel> for &str {
	fn eq(&self, other: &ReviewBodyPropertyIriOrLabel) -> bool {
		*self == ReviewBodyPropertyIri || *self == REVIEW_BODY_PROPERTY_LABEL
	}
}
