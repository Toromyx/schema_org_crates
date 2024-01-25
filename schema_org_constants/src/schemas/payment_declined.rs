/// <https://schema.org/PaymentDeclined>
pub const PAYMENT_DECLINED_IRI_HTTP: &str = "http://schema.org/PaymentDeclined";
/// <https://schema.org/PaymentDeclined>
pub const PAYMENT_DECLINED_IRI_HTTPS: &str = "https://schema.org/PaymentDeclined";
/// <https://schema.org/PaymentDeclined>
pub const PAYMENT_DECLINED_LABEL: &str = "PaymentDeclined";
pub struct PaymentDeclinedIri;
impl PartialEq<&str> for PaymentDeclinedIri {
	fn eq(&self, other: &&str) -> bool {
		*other == PAYMENT_DECLINED_IRI_HTTP || *other == PAYMENT_DECLINED_IRI_HTTPS
	}
}
impl PartialEq<PaymentDeclinedIri> for &str {
	fn eq(&self, other: &PaymentDeclinedIri) -> bool {
		*self == PAYMENT_DECLINED_IRI_HTTP || *self == PAYMENT_DECLINED_IRI_HTTPS
	}
}
pub struct PaymentDeclinedIriOrLabel;
impl PartialEq<&str> for PaymentDeclinedIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == PaymentDeclinedIri || *other == PAYMENT_DECLINED_LABEL
	}
}
impl PartialEq<PaymentDeclinedIriOrLabel> for &str {
	fn eq(&self, other: &PaymentDeclinedIriOrLabel) -> bool {
		*self == PaymentDeclinedIri || *self == PAYMENT_DECLINED_LABEL
	}
}
