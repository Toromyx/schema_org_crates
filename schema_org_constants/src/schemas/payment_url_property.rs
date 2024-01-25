/// <https://schema.org/paymentUrl>
pub const PAYMENT_URL_PROPERTY_IRI_HTTP: &str = "http://schema.org/paymentUrl";
/// <https://schema.org/paymentUrl>
pub const PAYMENT_URL_PROPERTY_IRI_HTTPS: &str = "https://schema.org/paymentUrl";
/// <https://schema.org/paymentUrl>
pub const PAYMENT_URL_PROPERTY_LABEL: &str = "paymentUrl";
pub struct PaymentUrlPropertyIri;
impl PartialEq<&str> for PaymentUrlPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == PAYMENT_URL_PROPERTY_IRI_HTTP || *other == PAYMENT_URL_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<PaymentUrlPropertyIri> for &str {
	fn eq(&self, other: &PaymentUrlPropertyIri) -> bool {
		*self == PAYMENT_URL_PROPERTY_IRI_HTTP || *self == PAYMENT_URL_PROPERTY_IRI_HTTPS
	}
}
pub struct PaymentUrlPropertyIriOrLabel;
impl PartialEq<&str> for PaymentUrlPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == PaymentUrlPropertyIri || *other == PAYMENT_URL_PROPERTY_LABEL
	}
}
impl PartialEq<PaymentUrlPropertyIriOrLabel> for &str {
	fn eq(&self, other: &PaymentUrlPropertyIriOrLabel) -> bool {
		*self == PaymentUrlPropertyIri || *self == PAYMENT_URL_PROPERTY_LABEL
	}
}
