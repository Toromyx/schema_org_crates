/// <https://schema.org/SomeProducts>
pub trait FindSomeProductsIds {
	type IdType;
	/// <https://schema.org/SomeProducts>
	fn find_some_products_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindSomeProductsIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_some_products_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::SOME_PRODUCTS_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::SOME_PRODUCTS_IRI_HTTPS,
			})
		}
	}
}
