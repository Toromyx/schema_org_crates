/// <https://schema.org/WearableSizeGroupGirls>
pub trait FindWearableSizeGroupGirlsIds {
	type IdType;
	/// <https://schema.org/WearableSizeGroupGirls>
	fn find_wearable_size_group_girls_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindWearableSizeGroupGirlsIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_wearable_size_group_girls_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => {
					schema_org_constants::WEARABLE_SIZE_GROUP_GIRLS_IRI_HTTP
				}
				SchemaOrgNamespace::Https => {
					schema_org_constants::WEARABLE_SIZE_GROUP_GIRLS_IRI_HTTPS
				}
			})
		}
	}
}
