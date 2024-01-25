/// <https://schema.org/PaymentMethod>
pub const PAYMENT_METHOD_IRI_HTTP: &str = "http://schema.org/PaymentMethod";
/// <https://schema.org/PaymentMethod>
pub const PAYMENT_METHOD_IRI_HTTPS: &str = "https://schema.org/PaymentMethod";
/// <https://schema.org/PaymentMethod>
pub const PAYMENT_METHOD_LABEL: &str = "PaymentMethod";
pub struct PaymentMethodIri;
impl PartialEq<&str> for PaymentMethodIri {
	fn eq(&self, other: &&str) -> bool {
		*other == PAYMENT_METHOD_IRI_HTTP || *other == PAYMENT_METHOD_IRI_HTTPS
	}
}
impl PartialEq<PaymentMethodIri> for &str {
	fn eq(&self, other: &PaymentMethodIri) -> bool {
		*self == PAYMENT_METHOD_IRI_HTTP || *self == PAYMENT_METHOD_IRI_HTTPS
	}
}
pub struct PaymentMethodIriOrLabel;
impl PartialEq<&str> for PaymentMethodIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == PaymentMethodIri || *other == PAYMENT_METHOD_LABEL
	}
}
impl PartialEq<PaymentMethodIriOrLabel> for &str {
	fn eq(&self, other: &PaymentMethodIriOrLabel) -> bool {
		*self == PaymentMethodIri || *self == PAYMENT_METHOD_LABEL
	}
}
