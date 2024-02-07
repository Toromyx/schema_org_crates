/// <https://schema.org/WholesaleStore>
pub trait FindWholesaleStoreIds {
	type IdType;
	/// <https://schema.org/WholesaleStore>
	fn find_wholesale_store_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindWholesaleStoreIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_wholesale_store_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::WHOLESALE_STORE_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::WHOLESALE_STORE_IRI_HTTPS,
			})
		}
	}
}
