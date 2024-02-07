/// <https://schema.org/WearableSizeGroupRegular>
pub trait FindWearableSizeGroupRegularIds {
	type IdType;
	/// <https://schema.org/WearableSizeGroupRegular>
	fn find_wearable_size_group_regular_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindWearableSizeGroupRegularIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_wearable_size_group_regular_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => {
					schema_org_constants::WEARABLE_SIZE_GROUP_REGULAR_IRI_HTTP
				}
				SchemaOrgNamespace::Https => {
					schema_org_constants::WEARABLE_SIZE_GROUP_REGULAR_IRI_HTTPS
				}
			})
		}
	}
}
