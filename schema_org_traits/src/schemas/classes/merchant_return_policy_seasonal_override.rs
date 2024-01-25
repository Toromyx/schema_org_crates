/// <https://schema.org/MerchantReturnPolicySeasonalOverride>
pub trait FindMerchantReturnPolicySeasonalOverrideIds {
	type IdType;
	fn find_merchant_return_policy_seasonal_override_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindMerchantReturnPolicySeasonalOverrideIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_merchant_return_policy_seasonal_override_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => {
					schema_org_constants::MERCHANT_RETURN_POLICY_SEASONAL_OVERRIDE_IRI_HTTP
				}
				SchemaOrgNamespace::Https => {
					schema_org_constants::MERCHANT_RETURN_POLICY_SEASONAL_OVERRIDE_IRI_HTTPS
				}
			})
		}
	}
}
