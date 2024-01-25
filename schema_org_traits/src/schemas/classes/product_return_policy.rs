/// <https://schema.org/ProductReturnPolicy>
#[deprecated = "This schema is archived, see <https://schema.org/docs/attic.home.html>. This schema is superseded by <https://schema.org/MerchantReturnPolicy>."]
pub trait FindProductReturnPolicyIds {
	type IdType;
	#[deprecated = "This schema is archived, see <https://schema.org/docs/attic.home.html>. This schema is superseded by <https://schema.org/MerchantReturnPolicy>."]
	fn find_product_return_policy_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindProductReturnPolicyIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_product_return_policy_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::PRODUCT_RETURN_POLICY_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::PRODUCT_RETURN_POLICY_IRI_HTTPS,
			})
		}
	}
}
