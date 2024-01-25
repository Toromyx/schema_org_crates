/// <https://schema.org/PaymentCard>
pub const PAYMENT_CARD_IRI_HTTP: &str = "http://schema.org/PaymentCard";
/// <https://schema.org/PaymentCard>
pub const PAYMENT_CARD_IRI_HTTPS: &str = "https://schema.org/PaymentCard";
/// <https://schema.org/PaymentCard>
pub const PAYMENT_CARD_LABEL: &str = "PaymentCard";
pub struct PaymentCardIri;
impl PartialEq<&str> for PaymentCardIri {
	fn eq(&self, other: &&str) -> bool {
		*other == PAYMENT_CARD_IRI_HTTP || *other == PAYMENT_CARD_IRI_HTTPS
	}
}
impl PartialEq<PaymentCardIri> for &str {
	fn eq(&self, other: &PaymentCardIri) -> bool {
		*self == PAYMENT_CARD_IRI_HTTP || *self == PAYMENT_CARD_IRI_HTTPS
	}
}
pub struct PaymentCardIriOrLabel;
impl PartialEq<&str> for PaymentCardIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == PaymentCardIri || *other == PAYMENT_CARD_LABEL
	}
}
impl PartialEq<PaymentCardIriOrLabel> for &str {
	fn eq(&self, other: &PaymentCardIriOrLabel) -> bool {
		*self == PaymentCardIri || *self == PAYMENT_CARD_LABEL
	}
}
