/// <https://schema.org/ratingExplanation>
pub const RATING_EXPLANATION_PROPERTY_IRI_HTTP: &str = "http://schema.org/ratingExplanation";
/// <https://schema.org/ratingExplanation>
pub const RATING_EXPLANATION_PROPERTY_IRI_HTTPS: &str = "https://schema.org/ratingExplanation";
/// <https://schema.org/ratingExplanation>
pub const RATING_EXPLANATION_PROPERTY_LABEL: &str = "ratingExplanation";
pub struct RatingExplanationPropertyIri;
impl PartialEq<&str> for RatingExplanationPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == RATING_EXPLANATION_PROPERTY_IRI_HTTP
			|| *other == RATING_EXPLANATION_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<RatingExplanationPropertyIri> for &str {
	fn eq(&self, other: &RatingExplanationPropertyIri) -> bool {
		*self == RATING_EXPLANATION_PROPERTY_IRI_HTTP
			|| *self == RATING_EXPLANATION_PROPERTY_IRI_HTTPS
	}
}
pub struct RatingExplanationPropertyIriOrLabel;
impl PartialEq<&str> for RatingExplanationPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == RatingExplanationPropertyIri || *other == RATING_EXPLANATION_PROPERTY_LABEL
	}
}
impl PartialEq<RatingExplanationPropertyIriOrLabel> for &str {
	fn eq(&self, other: &RatingExplanationPropertyIriOrLabel) -> bool {
		*self == RatingExplanationPropertyIri || *self == RATING_EXPLANATION_PROPERTY_LABEL
	}
}
