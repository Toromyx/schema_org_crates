/// <https://schema.org/ratingValue>
pub const RATING_VALUE_PROPERTY_IRI_HTTP: &str = "http://schema.org/ratingValue";
/// <https://schema.org/ratingValue>
pub const RATING_VALUE_PROPERTY_IRI_HTTPS: &str = "https://schema.org/ratingValue";
/// <https://schema.org/ratingValue>
pub const RATING_VALUE_PROPERTY_LABEL: &str = "ratingValue";
pub struct RatingValuePropertyIri;
impl PartialEq<&str> for RatingValuePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == RATING_VALUE_PROPERTY_IRI_HTTP || *other == RATING_VALUE_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<RatingValuePropertyIri> for &str {
	fn eq(&self, other: &RatingValuePropertyIri) -> bool {
		*self == RATING_VALUE_PROPERTY_IRI_HTTP || *self == RATING_VALUE_PROPERTY_IRI_HTTPS
	}
}
pub struct RatingValuePropertyIriOrLabel;
impl PartialEq<&str> for RatingValuePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == RatingValuePropertyIri || *other == RATING_VALUE_PROPERTY_LABEL
	}
}
impl PartialEq<RatingValuePropertyIriOrLabel> for &str {
	fn eq(&self, other: &RatingValuePropertyIriOrLabel) -> bool {
		*self == RatingValuePropertyIri || *self == RATING_VALUE_PROPERTY_LABEL
	}
}
