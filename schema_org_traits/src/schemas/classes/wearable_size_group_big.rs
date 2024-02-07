/// <https://schema.org/WearableSizeGroupBig>
pub trait FindWearableSizeGroupBigIds {
	type IdType;
	/// <https://schema.org/WearableSizeGroupBig>
	fn find_wearable_size_group_big_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindWearableSizeGroupBigIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_wearable_size_group_big_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::WEARABLE_SIZE_GROUP_BIG_IRI_HTTP,
				SchemaOrgNamespace::Https => {
					schema_org_constants::WEARABLE_SIZE_GROUP_BIG_IRI_HTTPS
				}
			})
		}
	}
}
