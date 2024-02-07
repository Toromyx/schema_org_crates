/// <https://schema.org/BrokerageAccount>
pub trait FindBrokerageAccountIds {
	type IdType;
	/// <https://schema.org/BrokerageAccount>
	fn find_brokerage_account_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindBrokerageAccountIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_brokerage_account_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::BROKERAGE_ACCOUNT_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::BROKERAGE_ACCOUNT_IRI_HTTPS,
			})
		}
	}
}
