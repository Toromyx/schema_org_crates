/// <https://schema.org/loanMortgageMandateAmount>
pub const LOAN_MORTGAGE_MANDATE_AMOUNT_PROPERTY_IRI_HTTP: &str =
	"http://schema.org/loanMortgageMandateAmount";
/// <https://schema.org/loanMortgageMandateAmount>
pub const LOAN_MORTGAGE_MANDATE_AMOUNT_PROPERTY_IRI_HTTPS: &str =
	"https://schema.org/loanMortgageMandateAmount";
/// <https://schema.org/loanMortgageMandateAmount>
pub const LOAN_MORTGAGE_MANDATE_AMOUNT_PROPERTY_LABEL: &str = "loanMortgageMandateAmount";
pub struct LoanMortgageMandateAmountPropertyIri;
impl PartialEq<&str> for LoanMortgageMandateAmountPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == LOAN_MORTGAGE_MANDATE_AMOUNT_PROPERTY_IRI_HTTP
			|| *other == LOAN_MORTGAGE_MANDATE_AMOUNT_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<LoanMortgageMandateAmountPropertyIri> for &str {
	fn eq(&self, other: &LoanMortgageMandateAmountPropertyIri) -> bool {
		*self == LOAN_MORTGAGE_MANDATE_AMOUNT_PROPERTY_IRI_HTTP
			|| *self == LOAN_MORTGAGE_MANDATE_AMOUNT_PROPERTY_IRI_HTTPS
	}
}
pub struct LoanMortgageMandateAmountPropertyIriOrLabel;
impl PartialEq<&str> for LoanMortgageMandateAmountPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == LoanMortgageMandateAmountPropertyIri
			|| *other == LOAN_MORTGAGE_MANDATE_AMOUNT_PROPERTY_LABEL
	}
}
impl PartialEq<LoanMortgageMandateAmountPropertyIriOrLabel> for &str {
	fn eq(&self, other: &LoanMortgageMandateAmountPropertyIriOrLabel) -> bool {
		*self == LoanMortgageMandateAmountPropertyIri
			|| *self == LOAN_MORTGAGE_MANDATE_AMOUNT_PROPERTY_LABEL
	}
}
