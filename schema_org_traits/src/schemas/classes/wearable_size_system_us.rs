/// <https://schema.org/WearableSizeSystemUS>
pub trait FindWearableSizeSystemUsIds {
	type IdType;
	/// <https://schema.org/WearableSizeSystemUS>
	fn find_wearable_size_system_us_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindWearableSizeSystemUsIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_wearable_size_system_us_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::WEARABLE_SIZE_SYSTEM_US_IRI_HTTP,
				SchemaOrgNamespace::Https => {
					schema_org_constants::WEARABLE_SIZE_SYSTEM_US_IRI_HTTPS
				}
			})
		}
	}
}
