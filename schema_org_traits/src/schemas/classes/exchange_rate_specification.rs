/// <https://schema.org/ExchangeRateSpecification>
pub trait FindExchangeRateSpecificationIds {
	type IdType;
	fn find_exchange_rate_specification_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindExchangeRateSpecificationIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_exchange_rate_specification_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => {
					schema_org_constants::EXCHANGE_RATE_SPECIFICATION_IRI_HTTP
				}
				SchemaOrgNamespace::Https => {
					schema_org_constants::EXCHANGE_RATE_SPECIFICATION_IRI_HTTPS
				}
			})
		}
	}
}
