/// <https://schema.org/InvestmentFund>
pub trait FindInvestmentFundIds {
	type IdType;
	fn find_investment_fund_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindInvestmentFundIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_investment_fund_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::INVESTMENT_FUND_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::INVESTMENT_FUND_IRI_HTTPS,
			})
		}
	}
}
