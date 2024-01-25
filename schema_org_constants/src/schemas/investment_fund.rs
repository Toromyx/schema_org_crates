/// <https://schema.org/InvestmentFund>
pub const INVESTMENT_FUND_IRI_HTTP: &str = "http://schema.org/InvestmentFund";
/// <https://schema.org/InvestmentFund>
pub const INVESTMENT_FUND_IRI_HTTPS: &str = "https://schema.org/InvestmentFund";
/// <https://schema.org/InvestmentFund>
pub const INVESTMENT_FUND_LABEL: &str = "InvestmentFund";
pub struct InvestmentFundIri;
impl PartialEq<&str> for InvestmentFundIri {
	fn eq(&self, other: &&str) -> bool {
		*other == INVESTMENT_FUND_IRI_HTTP || *other == INVESTMENT_FUND_IRI_HTTPS
	}
}
impl PartialEq<InvestmentFundIri> for &str {
	fn eq(&self, other: &InvestmentFundIri) -> bool {
		*self == INVESTMENT_FUND_IRI_HTTP || *self == INVESTMENT_FUND_IRI_HTTPS
	}
}
pub struct InvestmentFundIriOrLabel;
impl PartialEq<&str> for InvestmentFundIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == InvestmentFundIri || *other == INVESTMENT_FUND_LABEL
	}
}
impl PartialEq<InvestmentFundIriOrLabel> for &str {
	fn eq(&self, other: &InvestmentFundIriOrLabel) -> bool {
		*self == InvestmentFundIri || *self == INVESTMENT_FUND_LABEL
	}
}
