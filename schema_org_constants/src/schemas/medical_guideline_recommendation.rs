/// <https://schema.org/MedicalGuidelineRecommendation>
pub const MEDICAL_GUIDELINE_RECOMMENDATION_IRI_HTTP: &str =
	"http://schema.org/MedicalGuidelineRecommendation";
/// <https://schema.org/MedicalGuidelineRecommendation>
pub const MEDICAL_GUIDELINE_RECOMMENDATION_IRI_HTTPS: &str =
	"https://schema.org/MedicalGuidelineRecommendation";
/// <https://schema.org/MedicalGuidelineRecommendation>
pub const MEDICAL_GUIDELINE_RECOMMENDATION_LABEL: &str = "MedicalGuidelineRecommendation";
pub struct MedicalGuidelineRecommendationIri;
impl PartialEq<&str> for MedicalGuidelineRecommendationIri {
	fn eq(&self, other: &&str) -> bool {
		*other == MEDICAL_GUIDELINE_RECOMMENDATION_IRI_HTTP
			|| *other == MEDICAL_GUIDELINE_RECOMMENDATION_IRI_HTTPS
	}
}
impl PartialEq<MedicalGuidelineRecommendationIri> for &str {
	fn eq(&self, other: &MedicalGuidelineRecommendationIri) -> bool {
		*self == MEDICAL_GUIDELINE_RECOMMENDATION_IRI_HTTP
			|| *self == MEDICAL_GUIDELINE_RECOMMENDATION_IRI_HTTPS
	}
}
pub struct MedicalGuidelineRecommendationIriOrLabel;
impl PartialEq<&str> for MedicalGuidelineRecommendationIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == MedicalGuidelineRecommendationIri
			|| *other == MEDICAL_GUIDELINE_RECOMMENDATION_LABEL
	}
}
impl PartialEq<MedicalGuidelineRecommendationIriOrLabel> for &str {
	fn eq(&self, other: &MedicalGuidelineRecommendationIriOrLabel) -> bool {
		*self == MedicalGuidelineRecommendationIri
			|| *self == MEDICAL_GUIDELINE_RECOMMENDATION_LABEL
	}
}
