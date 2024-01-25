/// <https://schema.org/review>
pub const REVIEW_PROPERTY_IRI_HTTP: &str = "http://schema.org/review";
/// <https://schema.org/review>
pub const REVIEW_PROPERTY_IRI_HTTPS: &str = "https://schema.org/review";
/// <https://schema.org/review>
pub const REVIEW_PROPERTY_LABEL: &str = "review";
pub struct ReviewPropertyIri;
impl PartialEq<&str> for ReviewPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == REVIEW_PROPERTY_IRI_HTTP || *other == REVIEW_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<ReviewPropertyIri> for &str {
	fn eq(&self, other: &ReviewPropertyIri) -> bool {
		*self == REVIEW_PROPERTY_IRI_HTTP || *self == REVIEW_PROPERTY_IRI_HTTPS
	}
}
pub struct ReviewPropertyIriOrLabel;
impl PartialEq<&str> for ReviewPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ReviewPropertyIri || *other == REVIEW_PROPERTY_LABEL
	}
}
impl PartialEq<ReviewPropertyIriOrLabel> for &str {
	fn eq(&self, other: &ReviewPropertyIriOrLabel) -> bool {
		*self == ReviewPropertyIri || *self == REVIEW_PROPERTY_LABEL
	}
}
