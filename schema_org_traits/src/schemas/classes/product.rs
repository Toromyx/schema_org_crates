/// <https://schema.org/Product>
pub trait FindProductIds {
	type IdType;
	/// <https://schema.org/Product>
	fn find_product_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindProductIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_product_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::PRODUCT_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::PRODUCT_IRI_HTTPS,
			})
		}
	}
}
