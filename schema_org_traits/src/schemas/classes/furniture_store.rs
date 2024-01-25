/// <https://schema.org/FurnitureStore>
pub trait FindFurnitureStoreIds {
	type IdType;
	fn find_furniture_store_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindFurnitureStoreIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_furniture_store_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::FURNITURE_STORE_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::FURNITURE_STORE_IRI_HTTPS,
			})
		}
	}
}
