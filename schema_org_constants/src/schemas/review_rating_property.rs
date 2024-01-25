/// <https://schema.org/reviewRating>
pub const REVIEW_RATING_PROPERTY_IRI_HTTP: &str = "http://schema.org/reviewRating";
/// <https://schema.org/reviewRating>
pub const REVIEW_RATING_PROPERTY_IRI_HTTPS: &str = "https://schema.org/reviewRating";
/// <https://schema.org/reviewRating>
pub const REVIEW_RATING_PROPERTY_LABEL: &str = "reviewRating";
pub struct ReviewRatingPropertyIri;
impl PartialEq<&str> for ReviewRatingPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == REVIEW_RATING_PROPERTY_IRI_HTTP || *other == REVIEW_RATING_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<ReviewRatingPropertyIri> for &str {
	fn eq(&self, other: &ReviewRatingPropertyIri) -> bool {
		*self == REVIEW_RATING_PROPERTY_IRI_HTTP || *self == REVIEW_RATING_PROPERTY_IRI_HTTPS
	}
}
pub struct ReviewRatingPropertyIriOrLabel;
impl PartialEq<&str> for ReviewRatingPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ReviewRatingPropertyIri || *other == REVIEW_RATING_PROPERTY_LABEL
	}
}
impl PartialEq<ReviewRatingPropertyIriOrLabel> for &str {
	fn eq(&self, other: &ReviewRatingPropertyIriOrLabel) -> bool {
		*self == ReviewRatingPropertyIri || *self == REVIEW_RATING_PROPERTY_LABEL
	}
}
