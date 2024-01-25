/// <https://schema.org/ProductReturnNotPermitted>
#[deprecated = "This schema is archived, see <https://schema.org/docs/attic.home.html>. This schema is superseded by <https://schema.org/MerchantReturnNotPermitted>."]
pub trait FindProductReturnNotPermittedIds {
	type IdType;
	#[deprecated = "This schema is archived, see <https://schema.org/docs/attic.home.html>. This schema is superseded by <https://schema.org/MerchantReturnNotPermitted>."]
	fn find_product_return_not_permitted_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindProductReturnNotPermittedIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_product_return_not_permitted_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => {
					schema_org_constants::PRODUCT_RETURN_NOT_PERMITTED_IRI_HTTP
				}
				SchemaOrgNamespace::Https => {
					schema_org_constants::PRODUCT_RETURN_NOT_PERMITTED_IRI_HTTPS
				}
			})
		}
	}
}
