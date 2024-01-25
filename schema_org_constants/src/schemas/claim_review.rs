/// <https://schema.org/ClaimReview>
pub const CLAIM_REVIEW_IRI_HTTP: &str = "http://schema.org/ClaimReview";
/// <https://schema.org/ClaimReview>
pub const CLAIM_REVIEW_IRI_HTTPS: &str = "https://schema.org/ClaimReview";
/// <https://schema.org/ClaimReview>
pub const CLAIM_REVIEW_LABEL: &str = "ClaimReview";
pub struct ClaimReviewIri;
impl PartialEq<&str> for ClaimReviewIri {
	fn eq(&self, other: &&str) -> bool {
		*other == CLAIM_REVIEW_IRI_HTTP || *other == CLAIM_REVIEW_IRI_HTTPS
	}
}
impl PartialEq<ClaimReviewIri> for &str {
	fn eq(&self, other: &ClaimReviewIri) -> bool {
		*self == CLAIM_REVIEW_IRI_HTTP || *self == CLAIM_REVIEW_IRI_HTTPS
	}
}
pub struct ClaimReviewIriOrLabel;
impl PartialEq<&str> for ClaimReviewIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ClaimReviewIri || *other == CLAIM_REVIEW_LABEL
	}
}
impl PartialEq<ClaimReviewIriOrLabel> for &str {
	fn eq(&self, other: &ClaimReviewIriOrLabel) -> bool {
		*self == ClaimReviewIri || *self == CLAIM_REVIEW_LABEL
	}
}
