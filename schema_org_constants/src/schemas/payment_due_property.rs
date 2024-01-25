/// <https://schema.org/paymentDue>
#[deprecated = "This schema is superseded by <https://schema.org/paymentDueDate>."]
pub const PAYMENT_DUE_PROPERTY_IRI_HTTP: &str = "http://schema.org/paymentDue";
/// <https://schema.org/paymentDue>
#[deprecated = "This schema is superseded by <https://schema.org/paymentDueDate>."]
pub const PAYMENT_DUE_PROPERTY_IRI_HTTPS: &str = "https://schema.org/paymentDue";
/// <https://schema.org/paymentDue>
#[deprecated = "This schema is superseded by <https://schema.org/paymentDueDate>."]
pub const PAYMENT_DUE_PROPERTY_LABEL: &str = "paymentDue";
pub struct PaymentDuePropertyIri;
impl PartialEq<&str> for PaymentDuePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == PAYMENT_DUE_PROPERTY_IRI_HTTP || *other == PAYMENT_DUE_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<PaymentDuePropertyIri> for &str {
	fn eq(&self, other: &PaymentDuePropertyIri) -> bool {
		*self == PAYMENT_DUE_PROPERTY_IRI_HTTP || *self == PAYMENT_DUE_PROPERTY_IRI_HTTPS
	}
}
pub struct PaymentDuePropertyIriOrLabel;
impl PartialEq<&str> for PaymentDuePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == PaymentDuePropertyIri || *other == PAYMENT_DUE_PROPERTY_LABEL
	}
}
impl PartialEq<PaymentDuePropertyIriOrLabel> for &str {
	fn eq(&self, other: &PaymentDuePropertyIriOrLabel) -> bool {
		*self == PaymentDuePropertyIri || *self == PAYMENT_DUE_PROPERTY_LABEL
	}
}
