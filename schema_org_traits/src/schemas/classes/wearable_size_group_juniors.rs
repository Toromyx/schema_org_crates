/// <https://schema.org/WearableSizeGroupJuniors>
pub trait FindWearableSizeGroupJuniorsIds {
	type IdType;
	/// <https://schema.org/WearableSizeGroupJuniors>
	fn find_wearable_size_group_juniors_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindWearableSizeGroupJuniorsIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_wearable_size_group_juniors_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => {
					schema_org_constants::WEARABLE_SIZE_GROUP_JUNIORS_IRI_HTTP
				}
				SchemaOrgNamespace::Https => {
					schema_org_constants::WEARABLE_SIZE_GROUP_JUNIORS_IRI_HTTPS
				}
			})
		}
	}
}
