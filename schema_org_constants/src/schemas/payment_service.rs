/// <https://schema.org/PaymentService>
pub const PAYMENT_SERVICE_IRI_HTTP: &str = "http://schema.org/PaymentService";
/// <https://schema.org/PaymentService>
pub const PAYMENT_SERVICE_IRI_HTTPS: &str = "https://schema.org/PaymentService";
/// <https://schema.org/PaymentService>
pub const PAYMENT_SERVICE_LABEL: &str = "PaymentService";
pub struct PaymentServiceIri;
impl PartialEq<&str> for PaymentServiceIri {
	fn eq(&self, other: &&str) -> bool {
		*other == PAYMENT_SERVICE_IRI_HTTP || *other == PAYMENT_SERVICE_IRI_HTTPS
	}
}
impl PartialEq<PaymentServiceIri> for &str {
	fn eq(&self, other: &PaymentServiceIri) -> bool {
		*self == PAYMENT_SERVICE_IRI_HTTP || *self == PAYMENT_SERVICE_IRI_HTTPS
	}
}
pub struct PaymentServiceIriOrLabel;
impl PartialEq<&str> for PaymentServiceIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == PaymentServiceIri || *other == PAYMENT_SERVICE_LABEL
	}
}
impl PartialEq<PaymentServiceIriOrLabel> for &str {
	fn eq(&self, other: &PaymentServiceIriOrLabel) -> bool {
		*self == PaymentServiceIri || *self == PAYMENT_SERVICE_LABEL
	}
}
