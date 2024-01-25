/// <https://schema.org/renegotiableLoan>
pub const RENEGOTIABLE_LOAN_PROPERTY_IRI_HTTP: &str = "http://schema.org/renegotiableLoan";
/// <https://schema.org/renegotiableLoan>
pub const RENEGOTIABLE_LOAN_PROPERTY_IRI_HTTPS: &str = "https://schema.org/renegotiableLoan";
/// <https://schema.org/renegotiableLoan>
pub const RENEGOTIABLE_LOAN_PROPERTY_LABEL: &str = "renegotiableLoan";
pub struct RenegotiableLoanPropertyIri;
impl PartialEq<&str> for RenegotiableLoanPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == RENEGOTIABLE_LOAN_PROPERTY_IRI_HTTP
			|| *other == RENEGOTIABLE_LOAN_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<RenegotiableLoanPropertyIri> for &str {
	fn eq(&self, other: &RenegotiableLoanPropertyIri) -> bool {
		*self == RENEGOTIABLE_LOAN_PROPERTY_IRI_HTTP
			|| *self == RENEGOTIABLE_LOAN_PROPERTY_IRI_HTTPS
	}
}
pub struct RenegotiableLoanPropertyIriOrLabel;
impl PartialEq<&str> for RenegotiableLoanPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == RenegotiableLoanPropertyIri || *other == RENEGOTIABLE_LOAN_PROPERTY_LABEL
	}
}
impl PartialEq<RenegotiableLoanPropertyIriOrLabel> for &str {
	fn eq(&self, other: &RenegotiableLoanPropertyIriOrLabel) -> bool {
		*self == RenegotiableLoanPropertyIri || *self == RENEGOTIABLE_LOAN_PROPERTY_LABEL
	}
}
