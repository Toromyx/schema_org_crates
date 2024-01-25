/// <https://schema.org/MortgageLoan>
pub const MORTGAGE_LOAN_IRI_HTTP: &str = "http://schema.org/MortgageLoan";
/// <https://schema.org/MortgageLoan>
pub const MORTGAGE_LOAN_IRI_HTTPS: &str = "https://schema.org/MortgageLoan";
/// <https://schema.org/MortgageLoan>
pub const MORTGAGE_LOAN_LABEL: &str = "MortgageLoan";
pub struct MortgageLoanIri;
impl PartialEq<&str> for MortgageLoanIri {
	fn eq(&self, other: &&str) -> bool {
		*other == MORTGAGE_LOAN_IRI_HTTP || *other == MORTGAGE_LOAN_IRI_HTTPS
	}
}
impl PartialEq<MortgageLoanIri> for &str {
	fn eq(&self, other: &MortgageLoanIri) -> bool {
		*self == MORTGAGE_LOAN_IRI_HTTP || *self == MORTGAGE_LOAN_IRI_HTTPS
	}
}
pub struct MortgageLoanIriOrLabel;
impl PartialEq<&str> for MortgageLoanIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == MortgageLoanIri || *other == MORTGAGE_LOAN_LABEL
	}
}
impl PartialEq<MortgageLoanIriOrLabel> for &str {
	fn eq(&self, other: &MortgageLoanIriOrLabel) -> bool {
		*self == MortgageLoanIri || *self == MORTGAGE_LOAN_LABEL
	}
}
