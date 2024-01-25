/// <https://schema.org/WearableSizeGroupExtraTall>
pub trait FindWearableSizeGroupExtraTallIds {
	type IdType;
	fn find_wearable_size_group_extra_tall_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindWearableSizeGroupExtraTallIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_wearable_size_group_extra_tall_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => {
					schema_org_constants::WEARABLE_SIZE_GROUP_EXTRA_TALL_IRI_HTTP
				}
				SchemaOrgNamespace::Https => {
					schema_org_constants::WEARABLE_SIZE_GROUP_EXTRA_TALL_IRI_HTTPS
				}
			})
		}
	}
}
