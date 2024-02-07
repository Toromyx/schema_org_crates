/// <https://schema.org/MedicalSignOrSymptom>
pub trait FindMedicalSignOrSymptomIds {
	type IdType;
	/// <https://schema.org/MedicalSignOrSymptom>
	fn find_medical_sign_or_symptom_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindMedicalSignOrSymptomIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_medical_sign_or_symptom_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::MEDICAL_SIGN_OR_SYMPTOM_IRI_HTTP,
				SchemaOrgNamespace::Https => {
					schema_org_constants::MEDICAL_SIGN_OR_SYMPTOM_IRI_HTTPS
				}
			})
		}
	}
}
