/// <https://schema.org/loanType>
pub const LOAN_TYPE_PROPERTY_IRI_HTTP: &str = "http://schema.org/loanType";
/// <https://schema.org/loanType>
pub const LOAN_TYPE_PROPERTY_IRI_HTTPS: &str = "https://schema.org/loanType";
/// <https://schema.org/loanType>
pub const LOAN_TYPE_PROPERTY_LABEL: &str = "loanType";
pub struct LoanTypePropertyIri;
impl PartialEq<&str> for LoanTypePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == LOAN_TYPE_PROPERTY_IRI_HTTP || *other == LOAN_TYPE_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<LoanTypePropertyIri> for &str {
	fn eq(&self, other: &LoanTypePropertyIri) -> bool {
		*self == LOAN_TYPE_PROPERTY_IRI_HTTP || *self == LOAN_TYPE_PROPERTY_IRI_HTTPS
	}
}
pub struct LoanTypePropertyIriOrLabel;
impl PartialEq<&str> for LoanTypePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == LoanTypePropertyIri || *other == LOAN_TYPE_PROPERTY_LABEL
	}
}
impl PartialEq<LoanTypePropertyIriOrLabel> for &str {
	fn eq(&self, other: &LoanTypePropertyIriOrLabel) -> bool {
		*self == LoanTypePropertyIri || *self == LOAN_TYPE_PROPERTY_LABEL
	}
}
