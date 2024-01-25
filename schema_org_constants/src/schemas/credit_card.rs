/// <https://schema.org/CreditCard>
pub const CREDIT_CARD_IRI_HTTP: &str = "http://schema.org/CreditCard";
/// <https://schema.org/CreditCard>
pub const CREDIT_CARD_IRI_HTTPS: &str = "https://schema.org/CreditCard";
/// <https://schema.org/CreditCard>
pub const CREDIT_CARD_LABEL: &str = "CreditCard";
pub struct CreditCardIri;
impl PartialEq<&str> for CreditCardIri {
	fn eq(&self, other: &&str) -> bool {
		*other == CREDIT_CARD_IRI_HTTP || *other == CREDIT_CARD_IRI_HTTPS
	}
}
impl PartialEq<CreditCardIri> for &str {
	fn eq(&self, other: &CreditCardIri) -> bool {
		*self == CREDIT_CARD_IRI_HTTP || *self == CREDIT_CARD_IRI_HTTPS
	}
}
pub struct CreditCardIriOrLabel;
impl PartialEq<&str> for CreditCardIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == CreditCardIri || *other == CREDIT_CARD_LABEL
	}
}
impl PartialEq<CreditCardIriOrLabel> for &str {
	fn eq(&self, other: &CreditCardIriOrLabel) -> bool {
		*self == CreditCardIri || *self == CREDIT_CARD_LABEL
	}
}
