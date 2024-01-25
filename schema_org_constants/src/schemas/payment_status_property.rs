/// <https://schema.org/paymentStatus>
pub const PAYMENT_STATUS_PROPERTY_IRI_HTTP: &str = "http://schema.org/paymentStatus";
/// <https://schema.org/paymentStatus>
pub const PAYMENT_STATUS_PROPERTY_IRI_HTTPS: &str = "https://schema.org/paymentStatus";
/// <https://schema.org/paymentStatus>
pub const PAYMENT_STATUS_PROPERTY_LABEL: &str = "paymentStatus";
pub struct PaymentStatusPropertyIri;
impl PartialEq<&str> for PaymentStatusPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == PAYMENT_STATUS_PROPERTY_IRI_HTTP || *other == PAYMENT_STATUS_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<PaymentStatusPropertyIri> for &str {
	fn eq(&self, other: &PaymentStatusPropertyIri) -> bool {
		*self == PAYMENT_STATUS_PROPERTY_IRI_HTTP || *self == PAYMENT_STATUS_PROPERTY_IRI_HTTPS
	}
}
pub struct PaymentStatusPropertyIriOrLabel;
impl PartialEq<&str> for PaymentStatusPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == PaymentStatusPropertyIri || *other == PAYMENT_STATUS_PROPERTY_LABEL
	}
}
impl PartialEq<PaymentStatusPropertyIriOrLabel> for &str {
	fn eq(&self, other: &PaymentStatusPropertyIriOrLabel) -> bool {
		*self == PaymentStatusPropertyIri || *self == PAYMENT_STATUS_PROPERTY_LABEL
	}
}
