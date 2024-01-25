/// <https://schema.org/EmployerReview>
pub const EMPLOYER_REVIEW_IRI_HTTP: &str = "http://schema.org/EmployerReview";
/// <https://schema.org/EmployerReview>
pub const EMPLOYER_REVIEW_IRI_HTTPS: &str = "https://schema.org/EmployerReview";
/// <https://schema.org/EmployerReview>
pub const EMPLOYER_REVIEW_LABEL: &str = "EmployerReview";
pub struct EmployerReviewIri;
impl PartialEq<&str> for EmployerReviewIri {
	fn eq(&self, other: &&str) -> bool {
		*other == EMPLOYER_REVIEW_IRI_HTTP || *other == EMPLOYER_REVIEW_IRI_HTTPS
	}
}
impl PartialEq<EmployerReviewIri> for &str {
	fn eq(&self, other: &EmployerReviewIri) -> bool {
		*self == EMPLOYER_REVIEW_IRI_HTTP || *self == EMPLOYER_REVIEW_IRI_HTTPS
	}
}
pub struct EmployerReviewIriOrLabel;
impl PartialEq<&str> for EmployerReviewIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == EmployerReviewIri || *other == EMPLOYER_REVIEW_LABEL
	}
}
impl PartialEq<EmployerReviewIriOrLabel> for &str {
	fn eq(&self, other: &EmployerReviewIriOrLabel) -> bool {
		*self == EmployerReviewIri || *self == EMPLOYER_REVIEW_LABEL
	}
}
