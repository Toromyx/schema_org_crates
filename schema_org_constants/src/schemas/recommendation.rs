/// <https://schema.org/Recommendation>
pub const RECOMMENDATION_IRI_HTTP: &str = "http://schema.org/Recommendation";
/// <https://schema.org/Recommendation>
pub const RECOMMENDATION_IRI_HTTPS: &str = "https://schema.org/Recommendation";
/// <https://schema.org/Recommendation>
pub const RECOMMENDATION_LABEL: &str = "Recommendation";
pub struct RecommendationIri;
impl PartialEq<&str> for RecommendationIri {
	fn eq(&self, other: &&str) -> bool {
		*other == RECOMMENDATION_IRI_HTTP || *other == RECOMMENDATION_IRI_HTTPS
	}
}
impl PartialEq<RecommendationIri> for &str {
	fn eq(&self, other: &RecommendationIri) -> bool {
		*self == RECOMMENDATION_IRI_HTTP || *self == RECOMMENDATION_IRI_HTTPS
	}
}
pub struct RecommendationIriOrLabel;
impl PartialEq<&str> for RecommendationIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == RecommendationIri || *other == RECOMMENDATION_LABEL
	}
}
impl PartialEq<RecommendationIriOrLabel> for &str {
	fn eq(&self, other: &RecommendationIriOrLabel) -> bool {
		*self == RecommendationIri || *self == RECOMMENDATION_LABEL
	}
}
