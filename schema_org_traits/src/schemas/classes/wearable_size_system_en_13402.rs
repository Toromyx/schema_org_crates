/// <https://schema.org/WearableSizeSystemEN13402>
pub trait FindWearableSizeSystemEn13402Ids {
	type IdType;
	fn find_wearable_size_system_en_13402_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindWearableSizeSystemEn13402Ids for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_wearable_size_system_en_13402_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => {
					schema_org_constants::WEARABLE_SIZE_SYSTEM_EN_13402_IRI_HTTP
				}
				SchemaOrgNamespace::Https => {
					schema_org_constants::WEARABLE_SIZE_SYSTEM_EN_13402_IRI_HTTPS
				}
			})
		}
	}
}
