/// <https://schema.org/OutOfStock>
pub trait FindOutOfStockIds {
	type IdType;
	/// <https://schema.org/OutOfStock>
	fn find_out_of_stock_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindOutOfStockIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_out_of_stock_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::OUT_OF_STOCK_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::OUT_OF_STOCK_IRI_HTTPS,
			})
		}
	}
}
