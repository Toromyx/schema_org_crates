/// <https://schema.org/MedicalStudy>
pub trait FindMedicalStudyIds {
	type IdType;
	/// <https://schema.org/MedicalStudy>
	fn find_medical_study_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindMedicalStudyIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_medical_study_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::MEDICAL_STUDY_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::MEDICAL_STUDY_IRI_HTTPS,
			})
		}
	}
}
