/// <https://schema.org/WearableSizeGroupHusky>
pub trait FindWearableSizeGroupHuskyIds {
	type IdType;
	/// <https://schema.org/WearableSizeGroupHusky>
	fn find_wearable_size_group_husky_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindWearableSizeGroupHuskyIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_wearable_size_group_husky_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => {
					schema_org_constants::WEARABLE_SIZE_GROUP_HUSKY_IRI_HTTP
				}
				SchemaOrgNamespace::Https => {
					schema_org_constants::WEARABLE_SIZE_GROUP_HUSKY_IRI_HTTPS
				}
			})
		}
	}
}
#[cfg(any(feature = "json-ld_0_16", doc))]
mod json_ld_0_16 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindWearableSizeGroupHuskyIds for crate::json_ld_0_16::JsonLdStore {
		type IdType = json_ld_0_16::ValidId;
		fn find_wearable_size_group_husky_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => {
					schema_org_constants::WEARABLE_SIZE_GROUP_HUSKY_IRI_HTTP
				}
				SchemaOrgNamespace::Https => {
					schema_org_constants::WEARABLE_SIZE_GROUP_HUSKY_IRI_HTTPS
				}
			})
		}
	}
}
