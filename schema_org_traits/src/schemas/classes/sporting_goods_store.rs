/// <https://schema.org/SportingGoodsStore>
pub trait FindSportingGoodsStoreIds {
	type IdType;
	fn find_sporting_goods_store_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindSportingGoodsStoreIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_sporting_goods_store_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::SPORTING_GOODS_STORE_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::SPORTING_GOODS_STORE_IRI_HTTPS,
			})
		}
	}
}
