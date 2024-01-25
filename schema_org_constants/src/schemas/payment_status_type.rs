/// <https://schema.org/PaymentStatusType>
pub const PAYMENT_STATUS_TYPE_IRI_HTTP: &str = "http://schema.org/PaymentStatusType";
/// <https://schema.org/PaymentStatusType>
pub const PAYMENT_STATUS_TYPE_IRI_HTTPS: &str = "https://schema.org/PaymentStatusType";
/// <https://schema.org/PaymentStatusType>
pub const PAYMENT_STATUS_TYPE_LABEL: &str = "PaymentStatusType";
pub struct PaymentStatusTypeIri;
impl PartialEq<&str> for PaymentStatusTypeIri {
	fn eq(&self, other: &&str) -> bool {
		*other == PAYMENT_STATUS_TYPE_IRI_HTTP || *other == PAYMENT_STATUS_TYPE_IRI_HTTPS
	}
}
impl PartialEq<PaymentStatusTypeIri> for &str {
	fn eq(&self, other: &PaymentStatusTypeIri) -> bool {
		*self == PAYMENT_STATUS_TYPE_IRI_HTTP || *self == PAYMENT_STATUS_TYPE_IRI_HTTPS
	}
}
pub struct PaymentStatusTypeIriOrLabel;
impl PartialEq<&str> for PaymentStatusTypeIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == PaymentStatusTypeIri || *other == PAYMENT_STATUS_TYPE_LABEL
	}
}
impl PartialEq<PaymentStatusTypeIriOrLabel> for &str {
	fn eq(&self, other: &PaymentStatusTypeIriOrLabel) -> bool {
		*self == PaymentStatusTypeIri || *self == PAYMENT_STATUS_TYPE_LABEL
	}
}
