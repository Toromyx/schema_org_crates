/// <https://schema.org/starRating>
pub const STAR_RATING_PROPERTY_IRI_HTTP: &str = "http://schema.org/starRating";
/// <https://schema.org/starRating>
pub const STAR_RATING_PROPERTY_IRI_HTTPS: &str = "https://schema.org/starRating";
/// <https://schema.org/starRating>
pub const STAR_RATING_PROPERTY_LABEL: &str = "starRating";
pub struct StarRatingPropertyIri;
impl PartialEq<&str> for StarRatingPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == STAR_RATING_PROPERTY_IRI_HTTP || *other == STAR_RATING_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<StarRatingPropertyIri> for &str {
	fn eq(&self, other: &StarRatingPropertyIri) -> bool {
		*self == STAR_RATING_PROPERTY_IRI_HTTP || *self == STAR_RATING_PROPERTY_IRI_HTTPS
	}
}
pub struct StarRatingPropertyIriOrLabel;
impl PartialEq<&str> for StarRatingPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == StarRatingPropertyIri || *other == STAR_RATING_PROPERTY_LABEL
	}
}
impl PartialEq<StarRatingPropertyIriOrLabel> for &str {
	fn eq(&self, other: &StarRatingPropertyIriOrLabel) -> bool {
		*self == StarRatingPropertyIri || *self == STAR_RATING_PROPERTY_LABEL
	}
}
