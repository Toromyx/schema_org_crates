/// <https://schema.org/InvestmentOrDeposit>
pub trait FindInvestmentOrDepositIds {
	type IdType;
	/// <https://schema.org/InvestmentOrDeposit>
	fn find_investment_or_deposit_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindInvestmentOrDepositIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_investment_or_deposit_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::INVESTMENT_OR_DEPOSIT_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::INVESTMENT_OR_DEPOSIT_IRI_HTTPS,
			})
		}
	}
}
#[cfg(any(feature = "json-ld_0_16", doc))]
mod json_ld_0_16 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindInvestmentOrDepositIds for crate::json_ld_0_16::JsonLdStore {
		type IdType = json_ld_0_16::ValidId;
		fn find_investment_or_deposit_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::INVESTMENT_OR_DEPOSIT_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::INVESTMENT_OR_DEPOSIT_IRI_HTTPS,
			})
		}
	}
}
