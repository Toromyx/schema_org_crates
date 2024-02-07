/// <https://schema.org/MedicalGuidelineRecommendation>
pub trait FindMedicalGuidelineRecommendationIds {
	type IdType;
	/// <https://schema.org/MedicalGuidelineRecommendation>
	fn find_medical_guideline_recommendation_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindMedicalGuidelineRecommendationIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_medical_guideline_recommendation_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => {
					schema_org_constants::MEDICAL_GUIDELINE_RECOMMENDATION_IRI_HTTP
				}
				SchemaOrgNamespace::Https => {
					schema_org_constants::MEDICAL_GUIDELINE_RECOMMENDATION_IRI_HTTPS
				}
			})
		}
	}
}
