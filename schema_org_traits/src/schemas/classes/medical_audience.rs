/// <https://schema.org/MedicalAudience>
pub trait FindMedicalAudienceIds {
	type IdType;
	fn find_medical_audience_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindMedicalAudienceIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_medical_audience_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::MEDICAL_AUDIENCE_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::MEDICAL_AUDIENCE_IRI_HTTPS,
			})
		}
	}
}
