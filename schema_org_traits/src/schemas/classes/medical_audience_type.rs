/// <https://schema.org/MedicalAudienceType>
pub trait FindMedicalAudienceTypeIds {
	type IdType;
	fn find_medical_audience_type_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindMedicalAudienceTypeIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_medical_audience_type_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::MEDICAL_AUDIENCE_TYPE_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::MEDICAL_AUDIENCE_TYPE_IRI_HTTPS,
			})
		}
	}
}
