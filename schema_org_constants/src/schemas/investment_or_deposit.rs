/// <https://schema.org/InvestmentOrDeposit>
pub const INVESTMENT_OR_DEPOSIT_IRI_HTTP: &str = "http://schema.org/InvestmentOrDeposit";
/// <https://schema.org/InvestmentOrDeposit>
pub const INVESTMENT_OR_DEPOSIT_IRI_HTTPS: &str = "https://schema.org/InvestmentOrDeposit";
/// <https://schema.org/InvestmentOrDeposit>
pub const INVESTMENT_OR_DEPOSIT_LABEL: &str = "InvestmentOrDeposit";
pub struct InvestmentOrDepositIri;
impl PartialEq<&str> for InvestmentOrDepositIri {
	fn eq(&self, other: &&str) -> bool {
		*other == INVESTMENT_OR_DEPOSIT_IRI_HTTP || *other == INVESTMENT_OR_DEPOSIT_IRI_HTTPS
	}
}
impl PartialEq<InvestmentOrDepositIri> for &str {
	fn eq(&self, other: &InvestmentOrDepositIri) -> bool {
		*self == INVESTMENT_OR_DEPOSIT_IRI_HTTP || *self == INVESTMENT_OR_DEPOSIT_IRI_HTTPS
	}
}
pub struct InvestmentOrDepositIriOrLabel;
impl PartialEq<&str> for InvestmentOrDepositIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == InvestmentOrDepositIri || *other == INVESTMENT_OR_DEPOSIT_LABEL
	}
}
impl PartialEq<InvestmentOrDepositIriOrLabel> for &str {
	fn eq(&self, other: &InvestmentOrDepositIriOrLabel) -> bool {
		*self == InvestmentOrDepositIri || *self == INVESTMENT_OR_DEPOSIT_LABEL
	}
}
