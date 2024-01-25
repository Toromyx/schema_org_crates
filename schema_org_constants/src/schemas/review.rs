/// <https://schema.org/Review>
pub const REVIEW_IRI_HTTP: &str = "http://schema.org/Review";
/// <https://schema.org/Review>
pub const REVIEW_IRI_HTTPS: &str = "https://schema.org/Review";
/// <https://schema.org/Review>
pub const REVIEW_LABEL: &str = "Review";
pub struct ReviewIri;
impl PartialEq<&str> for ReviewIri {
	fn eq(&self, other: &&str) -> bool {
		*other == REVIEW_IRI_HTTP || *other == REVIEW_IRI_HTTPS
	}
}
impl PartialEq<ReviewIri> for &str {
	fn eq(&self, other: &ReviewIri) -> bool {
		*self == REVIEW_IRI_HTTP || *self == REVIEW_IRI_HTTPS
	}
}
pub struct ReviewIriOrLabel;
impl PartialEq<&str> for ReviewIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ReviewIri || *other == REVIEW_LABEL
	}
}
impl PartialEq<ReviewIriOrLabel> for &str {
	fn eq(&self, other: &ReviewIriOrLabel) -> bool {
		*self == ReviewIri || *self == REVIEW_LABEL
	}
}
