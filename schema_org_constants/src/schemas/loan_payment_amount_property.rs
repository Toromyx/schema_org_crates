/// <https://schema.org/loanPaymentAmount>
pub const LOAN_PAYMENT_AMOUNT_PROPERTY_IRI_HTTP: &str = "http://schema.org/loanPaymentAmount";
/// <https://schema.org/loanPaymentAmount>
pub const LOAN_PAYMENT_AMOUNT_PROPERTY_IRI_HTTPS: &str = "https://schema.org/loanPaymentAmount";
/// <https://schema.org/loanPaymentAmount>
pub const LOAN_PAYMENT_AMOUNT_PROPERTY_LABEL: &str = "loanPaymentAmount";
pub struct LoanPaymentAmountPropertyIri;
impl PartialEq<&str> for LoanPaymentAmountPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == LOAN_PAYMENT_AMOUNT_PROPERTY_IRI_HTTP
			|| *other == LOAN_PAYMENT_AMOUNT_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<LoanPaymentAmountPropertyIri> for &str {
	fn eq(&self, other: &LoanPaymentAmountPropertyIri) -> bool {
		*self == LOAN_PAYMENT_AMOUNT_PROPERTY_IRI_HTTP
			|| *self == LOAN_PAYMENT_AMOUNT_PROPERTY_IRI_HTTPS
	}
}
pub struct LoanPaymentAmountPropertyIriOrLabel;
impl PartialEq<&str> for LoanPaymentAmountPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == LoanPaymentAmountPropertyIri || *other == LOAN_PAYMENT_AMOUNT_PROPERTY_LABEL
	}
}
impl PartialEq<LoanPaymentAmountPropertyIriOrLabel> for &str {
	fn eq(&self, other: &LoanPaymentAmountPropertyIriOrLabel) -> bool {
		*self == LoanPaymentAmountPropertyIri || *self == LOAN_PAYMENT_AMOUNT_PROPERTY_LABEL
	}
}
