/// <https://schema.org/ProductReturnUnlimitedWindow>
#[deprecated = "This schema is archived, see <https://schema.org/docs/attic.home.html>. This schema is superseded by <https://schema.org/MerchantReturnUnlimitedWindow>."]
pub trait FindProductReturnUnlimitedWindowIds {
	type IdType;
	/// <https://schema.org/ProductReturnUnlimitedWindow>
	#[deprecated = "This schema is archived, see <https://schema.org/docs/attic.home.html>. This schema is superseded by <https://schema.org/MerchantReturnUnlimitedWindow>."]
	fn find_product_return_unlimited_window_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindProductReturnUnlimitedWindowIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_product_return_unlimited_window_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => {
					schema_org_constants::PRODUCT_RETURN_UNLIMITED_WINDOW_IRI_HTTP
				}
				SchemaOrgNamespace::Https => {
					schema_org_constants::PRODUCT_RETURN_UNLIMITED_WINDOW_IRI_HTTPS
				}
			})
		}
	}
}
