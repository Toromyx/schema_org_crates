/// <https://schema.org/UserReview>
pub const USER_REVIEW_IRI_HTTP: &str = "http://schema.org/UserReview";
/// <https://schema.org/UserReview>
pub const USER_REVIEW_IRI_HTTPS: &str = "https://schema.org/UserReview";
/// <https://schema.org/UserReview>
pub const USER_REVIEW_LABEL: &str = "UserReview";
pub struct UserReviewIri;
impl PartialEq<&str> for UserReviewIri {
	fn eq(&self, other: &&str) -> bool {
		*other == USER_REVIEW_IRI_HTTP || *other == USER_REVIEW_IRI_HTTPS
	}
}
impl PartialEq<UserReviewIri> for &str {
	fn eq(&self, other: &UserReviewIri) -> bool {
		*self == USER_REVIEW_IRI_HTTP || *self == USER_REVIEW_IRI_HTTPS
	}
}
pub struct UserReviewIriOrLabel;
impl PartialEq<&str> for UserReviewIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == UserReviewIri || *other == USER_REVIEW_LABEL
	}
}
impl PartialEq<UserReviewIriOrLabel> for &str {
	fn eq(&self, other: &UserReviewIriOrLabel) -> bool {
		*self == UserReviewIri || *self == USER_REVIEW_LABEL
	}
}
