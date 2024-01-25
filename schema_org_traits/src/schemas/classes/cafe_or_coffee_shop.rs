/// <https://schema.org/CafeOrCoffeeShop>
pub trait FindCafeOrCoffeeShopIds {
	type IdType;
	fn find_cafe_or_coffee_shop_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindCafeOrCoffeeShopIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_cafe_or_coffee_shop_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::CAFE_OR_COFFEE_SHOP_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::CAFE_OR_COFFEE_SHOP_IRI_HTTPS,
			})
		}
	}
}
