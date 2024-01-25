/// <https://schema.org/ProductReturnUnspecified>
#[deprecated = "This schema is archived, see <https://schema.org/docs/attic.home.html>. This schema is superseded by <https://schema.org/MerchantReturnUnspecified>."]
pub trait FindProductReturnUnspecifiedIds {
	type IdType;
	#[deprecated = "This schema is archived, see <https://schema.org/docs/attic.home.html>. This schema is superseded by <https://schema.org/MerchantReturnUnspecified>."]
	fn find_product_return_unspecified_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindProductReturnUnspecifiedIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_product_return_unspecified_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => {
					schema_org_constants::PRODUCT_RETURN_UNSPECIFIED_IRI_HTTP
				}
				SchemaOrgNamespace::Https => {
					schema_org_constants::PRODUCT_RETURN_UNSPECIFIED_IRI_HTTPS
				}
			})
		}
	}
}
