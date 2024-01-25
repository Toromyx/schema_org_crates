/// <https://schema.org/recourseLoan>
pub const RECOURSE_LOAN_PROPERTY_IRI_HTTP: &str = "http://schema.org/recourseLoan";
/// <https://schema.org/recourseLoan>
pub const RECOURSE_LOAN_PROPERTY_IRI_HTTPS: &str = "https://schema.org/recourseLoan";
/// <https://schema.org/recourseLoan>
pub const RECOURSE_LOAN_PROPERTY_LABEL: &str = "recourseLoan";
pub struct RecourseLoanPropertyIri;
impl PartialEq<&str> for RecourseLoanPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == RECOURSE_LOAN_PROPERTY_IRI_HTTP || *other == RECOURSE_LOAN_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<RecourseLoanPropertyIri> for &str {
	fn eq(&self, other: &RecourseLoanPropertyIri) -> bool {
		*self == RECOURSE_LOAN_PROPERTY_IRI_HTTP || *self == RECOURSE_LOAN_PROPERTY_IRI_HTTPS
	}
}
pub struct RecourseLoanPropertyIriOrLabel;
impl PartialEq<&str> for RecourseLoanPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == RecourseLoanPropertyIri || *other == RECOURSE_LOAN_PROPERTY_LABEL
	}
}
impl PartialEq<RecourseLoanPropertyIriOrLabel> for &str {
	fn eq(&self, other: &RecourseLoanPropertyIriOrLabel) -> bool {
		*self == RecourseLoanPropertyIri || *self == RECOURSE_LOAN_PROPERTY_LABEL
	}
}
