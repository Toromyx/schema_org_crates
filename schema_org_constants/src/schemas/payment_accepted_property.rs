/// <https://schema.org/paymentAccepted>
pub const PAYMENT_ACCEPTED_PROPERTY_IRI_HTTP: &str = "http://schema.org/paymentAccepted";
/// <https://schema.org/paymentAccepted>
pub const PAYMENT_ACCEPTED_PROPERTY_IRI_HTTPS: &str = "https://schema.org/paymentAccepted";
/// <https://schema.org/paymentAccepted>
pub const PAYMENT_ACCEPTED_PROPERTY_LABEL: &str = "paymentAccepted";
pub struct PaymentAcceptedPropertyIri;
impl PartialEq<&str> for PaymentAcceptedPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == PAYMENT_ACCEPTED_PROPERTY_IRI_HTTP
			|| *other == PAYMENT_ACCEPTED_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<PaymentAcceptedPropertyIri> for &str {
	fn eq(&self, other: &PaymentAcceptedPropertyIri) -> bool {
		*self == PAYMENT_ACCEPTED_PROPERTY_IRI_HTTP || *self == PAYMENT_ACCEPTED_PROPERTY_IRI_HTTPS
	}
}
pub struct PaymentAcceptedPropertyIriOrLabel;
impl PartialEq<&str> for PaymentAcceptedPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == PaymentAcceptedPropertyIri || *other == PAYMENT_ACCEPTED_PROPERTY_LABEL
	}
}
impl PartialEq<PaymentAcceptedPropertyIriOrLabel> for &str {
	fn eq(&self, other: &PaymentAcceptedPropertyIriOrLabel) -> bool {
		*self == PaymentAcceptedPropertyIri || *self == PAYMENT_ACCEPTED_PROPERTY_LABEL
	}
}
