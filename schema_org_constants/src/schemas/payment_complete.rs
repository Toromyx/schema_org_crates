/// <https://schema.org/PaymentComplete>
pub const PAYMENT_COMPLETE_IRI_HTTP: &str = "http://schema.org/PaymentComplete";
/// <https://schema.org/PaymentComplete>
pub const PAYMENT_COMPLETE_IRI_HTTPS: &str = "https://schema.org/PaymentComplete";
/// <https://schema.org/PaymentComplete>
pub const PAYMENT_COMPLETE_LABEL: &str = "PaymentComplete";
pub struct PaymentCompleteIri;
impl PartialEq<&str> for PaymentCompleteIri {
	fn eq(&self, other: &&str) -> bool {
		*other == PAYMENT_COMPLETE_IRI_HTTP || *other == PAYMENT_COMPLETE_IRI_HTTPS
	}
}
impl PartialEq<PaymentCompleteIri> for &str {
	fn eq(&self, other: &PaymentCompleteIri) -> bool {
		*self == PAYMENT_COMPLETE_IRI_HTTP || *self == PAYMENT_COMPLETE_IRI_HTTPS
	}
}
pub struct PaymentCompleteIriOrLabel;
impl PartialEq<&str> for PaymentCompleteIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == PaymentCompleteIri || *other == PAYMENT_COMPLETE_LABEL
	}
}
impl PartialEq<PaymentCompleteIriOrLabel> for &str {
	fn eq(&self, other: &PaymentCompleteIriOrLabel) -> bool {
		*self == PaymentCompleteIri || *self == PAYMENT_COMPLETE_LABEL
	}
}
