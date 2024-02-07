/// <https://schema.org/ReturnInStore>
pub trait FindReturnInStoreIds {
	type IdType;
	/// <https://schema.org/ReturnInStore>
	fn find_return_in_store_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindReturnInStoreIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_return_in_store_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::RETURN_IN_STORE_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::RETURN_IN_STORE_IRI_HTTPS,
			})
		}
	}
}
