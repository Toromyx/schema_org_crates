/// <https://schema.org/ShoppingCenter>
pub trait FindShoppingCenterIds {
	type IdType;
	fn find_shopping_center_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindShoppingCenterIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_shopping_center_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::SHOPPING_CENTER_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::SHOPPING_CENTER_IRI_HTTPS,
			})
		}
	}
}
