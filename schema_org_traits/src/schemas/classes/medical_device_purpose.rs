/// <https://schema.org/MedicalDevicePurpose>
pub trait FindMedicalDevicePurposeIds {
	type IdType;
	fn find_medical_device_purpose_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindMedicalDevicePurposeIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_medical_device_purpose_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::MEDICAL_DEVICE_PURPOSE_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::MEDICAL_DEVICE_PURPOSE_IRI_HTTPS,
			})
		}
	}
}
