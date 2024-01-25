/// <https://schema.org/loanRepaymentForm>
pub const LOAN_REPAYMENT_FORM_PROPERTY_IRI_HTTP: &str = "http://schema.org/loanRepaymentForm";
/// <https://schema.org/loanRepaymentForm>
pub const LOAN_REPAYMENT_FORM_PROPERTY_IRI_HTTPS: &str = "https://schema.org/loanRepaymentForm";
/// <https://schema.org/loanRepaymentForm>
pub const LOAN_REPAYMENT_FORM_PROPERTY_LABEL: &str = "loanRepaymentForm";
pub struct LoanRepaymentFormPropertyIri;
impl PartialEq<&str> for LoanRepaymentFormPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == LOAN_REPAYMENT_FORM_PROPERTY_IRI_HTTP
			|| *other == LOAN_REPAYMENT_FORM_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<LoanRepaymentFormPropertyIri> for &str {
	fn eq(&self, other: &LoanRepaymentFormPropertyIri) -> bool {
		*self == LOAN_REPAYMENT_FORM_PROPERTY_IRI_HTTP
			|| *self == LOAN_REPAYMENT_FORM_PROPERTY_IRI_HTTPS
	}
}
pub struct LoanRepaymentFormPropertyIriOrLabel;
impl PartialEq<&str> for LoanRepaymentFormPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == LoanRepaymentFormPropertyIri || *other == LOAN_REPAYMENT_FORM_PROPERTY_LABEL
	}
}
impl PartialEq<LoanRepaymentFormPropertyIriOrLabel> for &str {
	fn eq(&self, other: &LoanRepaymentFormPropertyIriOrLabel) -> bool {
		*self == LoanRepaymentFormPropertyIri || *self == LOAN_REPAYMENT_FORM_PROPERTY_LABEL
	}
}
