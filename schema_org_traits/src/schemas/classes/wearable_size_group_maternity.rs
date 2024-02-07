/// <https://schema.org/WearableSizeGroupMaternity>
pub trait FindWearableSizeGroupMaternityIds {
	type IdType;
	/// <https://schema.org/WearableSizeGroupMaternity>
	fn find_wearable_size_group_maternity_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindWearableSizeGroupMaternityIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_wearable_size_group_maternity_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => {
					schema_org_constants::WEARABLE_SIZE_GROUP_MATERNITY_IRI_HTTP
				}
				SchemaOrgNamespace::Https => {
					schema_org_constants::WEARABLE_SIZE_GROUP_MATERNITY_IRI_HTTPS
				}
			})
		}
	}
}
