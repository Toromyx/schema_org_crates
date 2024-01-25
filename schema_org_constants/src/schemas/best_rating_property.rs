/// <https://schema.org/bestRating>
pub const BEST_RATING_PROPERTY_IRI_HTTP: &str = "http://schema.org/bestRating";
/// <https://schema.org/bestRating>
pub const BEST_RATING_PROPERTY_IRI_HTTPS: &str = "https://schema.org/bestRating";
/// <https://schema.org/bestRating>
pub const BEST_RATING_PROPERTY_LABEL: &str = "bestRating";
pub struct BestRatingPropertyIri;
impl PartialEq<&str> for BestRatingPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == BEST_RATING_PROPERTY_IRI_HTTP || *other == BEST_RATING_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<BestRatingPropertyIri> for &str {
	fn eq(&self, other: &BestRatingPropertyIri) -> bool {
		*self == BEST_RATING_PROPERTY_IRI_HTTP || *self == BEST_RATING_PROPERTY_IRI_HTTPS
	}
}
pub struct BestRatingPropertyIriOrLabel;
impl PartialEq<&str> for BestRatingPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == BestRatingPropertyIri || *other == BEST_RATING_PROPERTY_LABEL
	}
}
impl PartialEq<BestRatingPropertyIriOrLabel> for &str {
	fn eq(&self, other: &BestRatingPropertyIriOrLabel) -> bool {
		*self == BestRatingPropertyIri || *self == BEST_RATING_PROPERTY_LABEL
	}
}
