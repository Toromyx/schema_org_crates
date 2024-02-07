/// <https://schema.org/OfflineTemporarily>
pub trait FindOfflineTemporarilyIds {
	type IdType;
	/// <https://schema.org/OfflineTemporarily>
	fn find_offline_temporarily_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindOfflineTemporarilyIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_offline_temporarily_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::OFFLINE_TEMPORARILY_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::OFFLINE_TEMPORARILY_IRI_HTTPS,
			})
		}
	}
}
