/// <https://schema.org/LoanOrCredit>
pub const LOAN_OR_CREDIT_IRI_HTTP: &str = "http://schema.org/LoanOrCredit";
/// <https://schema.org/LoanOrCredit>
pub const LOAN_OR_CREDIT_IRI_HTTPS: &str = "https://schema.org/LoanOrCredit";
/// <https://schema.org/LoanOrCredit>
pub const LOAN_OR_CREDIT_LABEL: &str = "LoanOrCredit";
pub struct LoanOrCreditIri;
impl PartialEq<&str> for LoanOrCreditIri {
	fn eq(&self, other: &&str) -> bool {
		*other == LOAN_OR_CREDIT_IRI_HTTP || *other == LOAN_OR_CREDIT_IRI_HTTPS
	}
}
impl PartialEq<LoanOrCreditIri> for &str {
	fn eq(&self, other: &LoanOrCreditIri) -> bool {
		*self == LOAN_OR_CREDIT_IRI_HTTP || *self == LOAN_OR_CREDIT_IRI_HTTPS
	}
}
pub struct LoanOrCreditIriOrLabel;
impl PartialEq<&str> for LoanOrCreditIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == LoanOrCreditIri || *other == LOAN_OR_CREDIT_LABEL
	}
}
impl PartialEq<LoanOrCreditIriOrLabel> for &str {
	fn eq(&self, other: &LoanOrCreditIriOrLabel) -> bool {
		*self == LoanOrCreditIri || *self == LOAN_OR_CREDIT_LABEL
	}
}
