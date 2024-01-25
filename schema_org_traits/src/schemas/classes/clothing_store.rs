/// <https://schema.org/ClothingStore>
pub trait FindClothingStoreIds {
	type IdType;
	fn find_clothing_store_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindClothingStoreIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_clothing_store_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::CLOTHING_STORE_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::CLOTHING_STORE_IRI_HTTPS,
			})
		}
	}
}
