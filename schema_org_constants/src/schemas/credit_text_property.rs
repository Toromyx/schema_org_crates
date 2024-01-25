/// <https://schema.org/creditText>
pub const CREDIT_TEXT_PROPERTY_IRI_HTTP: &str = "http://schema.org/creditText";
/// <https://schema.org/creditText>
pub const CREDIT_TEXT_PROPERTY_IRI_HTTPS: &str = "https://schema.org/creditText";
/// <https://schema.org/creditText>
pub const CREDIT_TEXT_PROPERTY_LABEL: &str = "creditText";
pub struct CreditTextPropertyIri;
impl PartialEq<&str> for CreditTextPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == CREDIT_TEXT_PROPERTY_IRI_HTTP || *other == CREDIT_TEXT_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<CreditTextPropertyIri> for &str {
	fn eq(&self, other: &CreditTextPropertyIri) -> bool {
		*self == CREDIT_TEXT_PROPERTY_IRI_HTTP || *self == CREDIT_TEXT_PROPERTY_IRI_HTTPS
	}
}
pub struct CreditTextPropertyIriOrLabel;
impl PartialEq<&str> for CreditTextPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == CreditTextPropertyIri || *other == CREDIT_TEXT_PROPERTY_LABEL
	}
}
impl PartialEq<CreditTextPropertyIriOrLabel> for &str {
	fn eq(&self, other: &CreditTextPropertyIriOrLabel) -> bool {
		*self == CreditTextPropertyIri || *self == CREDIT_TEXT_PROPERTY_LABEL
	}
}
