/// <https://schema.org/Rating>
pub const RATING_IRI_HTTP: &str = "http://schema.org/Rating";
/// <https://schema.org/Rating>
pub const RATING_IRI_HTTPS: &str = "https://schema.org/Rating";
/// <https://schema.org/Rating>
pub const RATING_LABEL: &str = "Rating";
pub struct RatingIri;
impl PartialEq<&str> for RatingIri {
	fn eq(&self, other: &&str) -> bool {
		*other == RATING_IRI_HTTP || *other == RATING_IRI_HTTPS
	}
}
impl PartialEq<RatingIri> for &str {
	fn eq(&self, other: &RatingIri) -> bool {
		*self == RATING_IRI_HTTP || *self == RATING_IRI_HTTPS
	}
}
pub struct RatingIriOrLabel;
impl PartialEq<&str> for RatingIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == RatingIri || *other == RATING_LABEL
	}
}
impl PartialEq<RatingIriOrLabel> for &str {
	fn eq(&self, other: &RatingIriOrLabel) -> bool {
		*self == RatingIri || *self == RATING_LABEL
	}
}
