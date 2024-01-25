/// <https://schema.org/loanTerm>
pub const LOAN_TERM_PROPERTY_IRI_HTTP: &str = "http://schema.org/loanTerm";
/// <https://schema.org/loanTerm>
pub const LOAN_TERM_PROPERTY_IRI_HTTPS: &str = "https://schema.org/loanTerm";
/// <https://schema.org/loanTerm>
pub const LOAN_TERM_PROPERTY_LABEL: &str = "loanTerm";
pub struct LoanTermPropertyIri;
impl PartialEq<&str> for LoanTermPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == LOAN_TERM_PROPERTY_IRI_HTTP || *other == LOAN_TERM_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<LoanTermPropertyIri> for &str {
	fn eq(&self, other: &LoanTermPropertyIri) -> bool {
		*self == LOAN_TERM_PROPERTY_IRI_HTTP || *self == LOAN_TERM_PROPERTY_IRI_HTTPS
	}
}
pub struct LoanTermPropertyIriOrLabel;
impl PartialEq<&str> for LoanTermPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == LoanTermPropertyIri || *other == LOAN_TERM_PROPERTY_LABEL
	}
}
impl PartialEq<LoanTermPropertyIriOrLabel> for &str {
	fn eq(&self, other: &LoanTermPropertyIriOrLabel) -> bool {
		*self == LoanTermPropertyIri || *self == LOAN_TERM_PROPERTY_LABEL
	}
}
