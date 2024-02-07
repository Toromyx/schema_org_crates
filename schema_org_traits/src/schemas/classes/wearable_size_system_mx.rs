/// <https://schema.org/WearableSizeSystemMX>
pub trait FindWearableSizeSystemMxIds {
	type IdType;
	/// <https://schema.org/WearableSizeSystemMX>
	fn find_wearable_size_system_mx_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindWearableSizeSystemMxIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_wearable_size_system_mx_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::WEARABLE_SIZE_SYSTEM_MX_IRI_HTTP,
				SchemaOrgNamespace::Https => {
					schema_org_constants::WEARABLE_SIZE_SYSTEM_MX_IRI_HTTPS
				}
			})
		}
	}
}
