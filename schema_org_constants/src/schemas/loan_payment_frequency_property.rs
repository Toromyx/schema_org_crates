/// <https://schema.org/loanPaymentFrequency>
pub const LOAN_PAYMENT_FREQUENCY_PROPERTY_IRI_HTTP: &str = "http://schema.org/loanPaymentFrequency";
/// <https://schema.org/loanPaymentFrequency>
pub const LOAN_PAYMENT_FREQUENCY_PROPERTY_IRI_HTTPS: &str =
	"https://schema.org/loanPaymentFrequency";
/// <https://schema.org/loanPaymentFrequency>
pub const LOAN_PAYMENT_FREQUENCY_PROPERTY_LABEL: &str = "loanPaymentFrequency";
pub struct LoanPaymentFrequencyPropertyIri;
impl PartialEq<&str> for LoanPaymentFrequencyPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == LOAN_PAYMENT_FREQUENCY_PROPERTY_IRI_HTTP
			|| *other == LOAN_PAYMENT_FREQUENCY_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<LoanPaymentFrequencyPropertyIri> for &str {
	fn eq(&self, other: &LoanPaymentFrequencyPropertyIri) -> bool {
		*self == LOAN_PAYMENT_FREQUENCY_PROPERTY_IRI_HTTP
			|| *self == LOAN_PAYMENT_FREQUENCY_PROPERTY_IRI_HTTPS
	}
}
pub struct LoanPaymentFrequencyPropertyIriOrLabel;
impl PartialEq<&str> for LoanPaymentFrequencyPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == LoanPaymentFrequencyPropertyIri || *other == LOAN_PAYMENT_FREQUENCY_PROPERTY_LABEL
	}
}
impl PartialEq<LoanPaymentFrequencyPropertyIriOrLabel> for &str {
	fn eq(&self, other: &LoanPaymentFrequencyPropertyIriOrLabel) -> bool {
		*self == LoanPaymentFrequencyPropertyIri || *self == LOAN_PAYMENT_FREQUENCY_PROPERTY_LABEL
	}
}
