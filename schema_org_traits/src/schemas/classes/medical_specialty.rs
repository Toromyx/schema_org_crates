/// <https://schema.org/MedicalSpecialty>
pub trait FindMedicalSpecialtyIds {
	type IdType;
	/// <https://schema.org/MedicalSpecialty>
	fn find_medical_specialty_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindMedicalSpecialtyIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_medical_specialty_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::MEDICAL_SPECIALTY_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::MEDICAL_SPECIALTY_IRI_HTTPS,
			})
		}
	}
}
