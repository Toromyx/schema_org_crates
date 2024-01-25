/// <https://schema.org/worstRating>
pub const WORST_RATING_PROPERTY_IRI_HTTP: &str = "http://schema.org/worstRating";
/// <https://schema.org/worstRating>
pub const WORST_RATING_PROPERTY_IRI_HTTPS: &str = "https://schema.org/worstRating";
/// <https://schema.org/worstRating>
pub const WORST_RATING_PROPERTY_LABEL: &str = "worstRating";
pub struct WorstRatingPropertyIri;
impl PartialEq<&str> for WorstRatingPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == WORST_RATING_PROPERTY_IRI_HTTP || *other == WORST_RATING_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<WorstRatingPropertyIri> for &str {
	fn eq(&self, other: &WorstRatingPropertyIri) -> bool {
		*self == WORST_RATING_PROPERTY_IRI_HTTP || *self == WORST_RATING_PROPERTY_IRI_HTTPS
	}
}
pub struct WorstRatingPropertyIriOrLabel;
impl PartialEq<&str> for WorstRatingPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == WorstRatingPropertyIri || *other == WORST_RATING_PROPERTY_LABEL
	}
}
impl PartialEq<WorstRatingPropertyIriOrLabel> for &str {
	fn eq(&self, other: &WorstRatingPropertyIriOrLabel) -> bool {
		*self == WorstRatingPropertyIri || *self == WORST_RATING_PROPERTY_LABEL
	}
}
