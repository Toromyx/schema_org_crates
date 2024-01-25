/// <https://schema.org/paymentMethod>
pub const PAYMENT_METHOD_PROPERTY_IRI_HTTP: &str = "http://schema.org/paymentMethod";
/// <https://schema.org/paymentMethod>
pub const PAYMENT_METHOD_PROPERTY_IRI_HTTPS: &str = "https://schema.org/paymentMethod";
/// <https://schema.org/paymentMethod>
pub const PAYMENT_METHOD_PROPERTY_LABEL: &str = "paymentMethod";
pub struct PaymentMethodPropertyIri;
impl PartialEq<&str> for PaymentMethodPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == PAYMENT_METHOD_PROPERTY_IRI_HTTP || *other == PAYMENT_METHOD_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<PaymentMethodPropertyIri> for &str {
	fn eq(&self, other: &PaymentMethodPropertyIri) -> bool {
		*self == PAYMENT_METHOD_PROPERTY_IRI_HTTP || *self == PAYMENT_METHOD_PROPERTY_IRI_HTTPS
	}
}
pub struct PaymentMethodPropertyIriOrLabel;
impl PartialEq<&str> for PaymentMethodPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == PaymentMethodPropertyIri || *other == PAYMENT_METHOD_PROPERTY_LABEL
	}
}
impl PartialEq<PaymentMethodPropertyIriOrLabel> for &str {
	fn eq(&self, other: &PaymentMethodPropertyIriOrLabel) -> bool {
		*self == PaymentMethodPropertyIri || *self == PAYMENT_METHOD_PROPERTY_LABEL
	}
}
