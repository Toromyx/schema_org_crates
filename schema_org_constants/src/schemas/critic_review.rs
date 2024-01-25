/// <https://schema.org/CriticReview>
pub const CRITIC_REVIEW_IRI_HTTP: &str = "http://schema.org/CriticReview";
/// <https://schema.org/CriticReview>
pub const CRITIC_REVIEW_IRI_HTTPS: &str = "https://schema.org/CriticReview";
/// <https://schema.org/CriticReview>
pub const CRITIC_REVIEW_LABEL: &str = "CriticReview";
pub struct CriticReviewIri;
impl PartialEq<&str> for CriticReviewIri {
	fn eq(&self, other: &&str) -> bool {
		*other == CRITIC_REVIEW_IRI_HTTP || *other == CRITIC_REVIEW_IRI_HTTPS
	}
}
impl PartialEq<CriticReviewIri> for &str {
	fn eq(&self, other: &CriticReviewIri) -> bool {
		*self == CRITIC_REVIEW_IRI_HTTP || *self == CRITIC_REVIEW_IRI_HTTPS
	}
}
pub struct CriticReviewIriOrLabel;
impl PartialEq<&str> for CriticReviewIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == CriticReviewIri || *other == CRITIC_REVIEW_LABEL
	}
}
impl PartialEq<CriticReviewIriOrLabel> for &str {
	fn eq(&self, other: &CriticReviewIriOrLabel) -> bool {
		*self == CriticReviewIri || *self == CRITIC_REVIEW_LABEL
	}
}
