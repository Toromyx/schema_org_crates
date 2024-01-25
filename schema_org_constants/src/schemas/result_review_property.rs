/// <https://schema.org/resultReview>
pub const RESULT_REVIEW_PROPERTY_IRI_HTTP: &str = "http://schema.org/resultReview";
/// <https://schema.org/resultReview>
pub const RESULT_REVIEW_PROPERTY_IRI_HTTPS: &str = "https://schema.org/resultReview";
/// <https://schema.org/resultReview>
pub const RESULT_REVIEW_PROPERTY_LABEL: &str = "resultReview";
pub struct ResultReviewPropertyIri;
impl PartialEq<&str> for ResultReviewPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == RESULT_REVIEW_PROPERTY_IRI_HTTP || *other == RESULT_REVIEW_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<ResultReviewPropertyIri> for &str {
	fn eq(&self, other: &ResultReviewPropertyIri) -> bool {
		*self == RESULT_REVIEW_PROPERTY_IRI_HTTP || *self == RESULT_REVIEW_PROPERTY_IRI_HTTPS
	}
}
pub struct ResultReviewPropertyIriOrLabel;
impl PartialEq<&str> for ResultReviewPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ResultReviewPropertyIri || *other == RESULT_REVIEW_PROPERTY_LABEL
	}
}
impl PartialEq<ResultReviewPropertyIriOrLabel> for &str {
	fn eq(&self, other: &ResultReviewPropertyIriOrLabel) -> bool {
		*self == ResultReviewPropertyIri || *self == RESULT_REVIEW_PROPERTY_LABEL
	}
}
