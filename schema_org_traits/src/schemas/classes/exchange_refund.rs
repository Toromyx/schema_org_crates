/// <https://schema.org/ExchangeRefund>
pub trait FindExchangeRefundIds {
	type IdType;
	fn find_exchange_refund_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindExchangeRefundIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_exchange_refund_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::EXCHANGE_REFUND_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::EXCHANGE_REFUND_IRI_HTTPS,
			})
		}
	}
}
