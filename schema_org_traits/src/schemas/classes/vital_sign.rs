/// <https://schema.org/VitalSign>
pub trait FindVitalSignIds {
	type IdType;
	fn find_vital_sign_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindVitalSignIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_vital_sign_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::VITAL_SIGN_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::VITAL_SIGN_IRI_HTTPS,
			})
		}
	}
}
