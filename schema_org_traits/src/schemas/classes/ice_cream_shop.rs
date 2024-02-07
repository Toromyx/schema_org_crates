/// <https://schema.org/IceCreamShop>
pub trait FindIceCreamShopIds {
	type IdType;
	/// <https://schema.org/IceCreamShop>
	fn find_ice_cream_shop_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindIceCreamShopIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_ice_cream_shop_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::ICE_CREAM_SHOP_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::ICE_CREAM_SHOP_IRI_HTTPS,
			})
		}
	}
}
