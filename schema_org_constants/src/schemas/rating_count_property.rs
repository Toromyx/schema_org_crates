/// <https://schema.org/ratingCount>
pub const RATING_COUNT_PROPERTY_IRI_HTTP: &str = "http://schema.org/ratingCount";
/// <https://schema.org/ratingCount>
pub const RATING_COUNT_PROPERTY_IRI_HTTPS: &str = "https://schema.org/ratingCount";
/// <https://schema.org/ratingCount>
pub const RATING_COUNT_PROPERTY_LABEL: &str = "ratingCount";
pub struct RatingCountPropertyIri;
impl PartialEq<&str> for RatingCountPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == RATING_COUNT_PROPERTY_IRI_HTTP || *other == RATING_COUNT_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<RatingCountPropertyIri> for &str {
	fn eq(&self, other: &RatingCountPropertyIri) -> bool {
		*self == RATING_COUNT_PROPERTY_IRI_HTTP || *self == RATING_COUNT_PROPERTY_IRI_HTTPS
	}
}
pub struct RatingCountPropertyIriOrLabel;
impl PartialEq<&str> for RatingCountPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == RatingCountPropertyIri || *other == RATING_COUNT_PROPERTY_LABEL
	}
}
impl PartialEq<RatingCountPropertyIriOrLabel> for &str {
	fn eq(&self, other: &RatingCountPropertyIriOrLabel) -> bool {
		*self == RatingCountPropertyIri || *self == RATING_COUNT_PROPERTY_LABEL
	}
}
