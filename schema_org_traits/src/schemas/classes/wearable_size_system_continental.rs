/// <https://schema.org/WearableSizeSystemContinental>
pub trait FindWearableSizeSystemContinentalIds {
	type IdType;
	/// <https://schema.org/WearableSizeSystemContinental>
	fn find_wearable_size_system_continental_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindWearableSizeSystemContinentalIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_wearable_size_system_continental_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => {
					schema_org_constants::WEARABLE_SIZE_SYSTEM_CONTINENTAL_IRI_HTTP
				}
				SchemaOrgNamespace::Https => {
					schema_org_constants::WEARABLE_SIZE_SYSTEM_CONTINENTAL_IRI_HTTPS
				}
			})
		}
	}
}
