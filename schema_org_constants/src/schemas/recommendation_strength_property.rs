/// <https://schema.org/recommendationStrength>
pub const RECOMMENDATION_STRENGTH_PROPERTY_IRI_HTTP: &str =
	"http://schema.org/recommendationStrength";
/// <https://schema.org/recommendationStrength>
pub const RECOMMENDATION_STRENGTH_PROPERTY_IRI_HTTPS: &str =
	"https://schema.org/recommendationStrength";
/// <https://schema.org/recommendationStrength>
pub const RECOMMENDATION_STRENGTH_PROPERTY_LABEL: &str = "recommendationStrength";
pub struct RecommendationStrengthPropertyIri;
impl PartialEq<&str> for RecommendationStrengthPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == RECOMMENDATION_STRENGTH_PROPERTY_IRI_HTTP
			|| *other == RECOMMENDATION_STRENGTH_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<RecommendationStrengthPropertyIri> for &str {
	fn eq(&self, other: &RecommendationStrengthPropertyIri) -> bool {
		*self == RECOMMENDATION_STRENGTH_PROPERTY_IRI_HTTP
			|| *self == RECOMMENDATION_STRENGTH_PROPERTY_IRI_HTTPS
	}
}
pub struct RecommendationStrengthPropertyIriOrLabel;
impl PartialEq<&str> for RecommendationStrengthPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == RecommendationStrengthPropertyIri
			|| *other == RECOMMENDATION_STRENGTH_PROPERTY_LABEL
	}
}
impl PartialEq<RecommendationStrengthPropertyIriOrLabel> for &str {
	fn eq(&self, other: &RecommendationStrengthPropertyIriOrLabel) -> bool {
		*self == RecommendationStrengthPropertyIri
			|| *self == RECOMMENDATION_STRENGTH_PROPERTY_LABEL
	}
}
