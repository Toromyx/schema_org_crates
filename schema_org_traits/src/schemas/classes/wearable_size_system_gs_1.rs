/// <https://schema.org/WearableSizeSystemGS1>
pub trait FindWearableSizeSystemGs1Ids {
	type IdType;
	fn find_wearable_size_system_gs_1_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindWearableSizeSystemGs1Ids for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_wearable_size_system_gs_1_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => {
					schema_org_constants::WEARABLE_SIZE_SYSTEM_GS_1_IRI_HTTP
				}
				SchemaOrgNamespace::Https => {
					schema_org_constants::WEARABLE_SIZE_SYSTEM_GS_1_IRI_HTTPS
				}
			})
		}
	}
}
