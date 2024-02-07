/// <https://schema.org/SalePrice>
pub trait FindSalePriceIds {
	type IdType;
	/// <https://schema.org/SalePrice>
	fn find_sale_price_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindSalePriceIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_sale_price_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::SALE_PRICE_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::SALE_PRICE_IRI_HTTPS,
			})
		}
	}
}
