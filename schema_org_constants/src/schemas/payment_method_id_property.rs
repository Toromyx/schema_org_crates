/// <https://schema.org/paymentMethodId>
pub const PAYMENT_METHOD_ID_PROPERTY_IRI_HTTP: &str = "http://schema.org/paymentMethodId";
/// <https://schema.org/paymentMethodId>
pub const PAYMENT_METHOD_ID_PROPERTY_IRI_HTTPS: &str = "https://schema.org/paymentMethodId";
/// <https://schema.org/paymentMethodId>
pub const PAYMENT_METHOD_ID_PROPERTY_LABEL: &str = "paymentMethodId";
pub struct PaymentMethodIdPropertyIri;
impl PartialEq<&str> for PaymentMethodIdPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == PAYMENT_METHOD_ID_PROPERTY_IRI_HTTP
			|| *other == PAYMENT_METHOD_ID_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<PaymentMethodIdPropertyIri> for &str {
	fn eq(&self, other: &PaymentMethodIdPropertyIri) -> bool {
		*self == PAYMENT_METHOD_ID_PROPERTY_IRI_HTTP
			|| *self == PAYMENT_METHOD_ID_PROPERTY_IRI_HTTPS
	}
}
pub struct PaymentMethodIdPropertyIriOrLabel;
impl PartialEq<&str> for PaymentMethodIdPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == PaymentMethodIdPropertyIri || *other == PAYMENT_METHOD_ID_PROPERTY_LABEL
	}
}
impl PartialEq<PaymentMethodIdPropertyIriOrLabel> for &str {
	fn eq(&self, other: &PaymentMethodIdPropertyIriOrLabel) -> bool {
		*self == PaymentMethodIdPropertyIri || *self == PAYMENT_METHOD_ID_PROPERTY_LABEL
	}
}
