/// <https://schema.org/PaymentDue>
pub const PAYMENT_DUE_IRI_HTTP: &str = "http://schema.org/PaymentDue";
/// <https://schema.org/PaymentDue>
pub const PAYMENT_DUE_IRI_HTTPS: &str = "https://schema.org/PaymentDue";
/// <https://schema.org/PaymentDue>
pub const PAYMENT_DUE_LABEL: &str = "PaymentDue";
pub struct PaymentDueIri;
impl PartialEq<&str> for PaymentDueIri {
	fn eq(&self, other: &&str) -> bool {
		*other == PAYMENT_DUE_IRI_HTTP || *other == PAYMENT_DUE_IRI_HTTPS
	}
}
impl PartialEq<PaymentDueIri> for &str {
	fn eq(&self, other: &PaymentDueIri) -> bool {
		*self == PAYMENT_DUE_IRI_HTTP || *self == PAYMENT_DUE_IRI_HTTPS
	}
}
pub struct PaymentDueIriOrLabel;
impl PartialEq<&str> for PaymentDueIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == PaymentDueIri || *other == PAYMENT_DUE_LABEL
	}
}
impl PartialEq<PaymentDueIriOrLabel> for &str {
	fn eq(&self, other: &PaymentDueIriOrLabel) -> bool {
		*self == PaymentDueIri || *self == PAYMENT_DUE_LABEL
	}
}
