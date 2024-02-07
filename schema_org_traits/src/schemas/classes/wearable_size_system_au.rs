/// <https://schema.org/WearableSizeSystemAU>
pub trait FindWearableSizeSystemAuIds {
	type IdType;
	/// <https://schema.org/WearableSizeSystemAU>
	fn find_wearable_size_system_au_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindWearableSizeSystemAuIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_wearable_size_system_au_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::WEARABLE_SIZE_SYSTEM_AU_IRI_HTTP,
				SchemaOrgNamespace::Https => {
					schema_org_constants::WEARABLE_SIZE_SYSTEM_AU_IRI_HTTPS
				}
			})
		}
	}
}
