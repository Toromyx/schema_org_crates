/// <https://schema.org/paymentDueDate>
pub const PAYMENT_DUE_DATE_PROPERTY_IRI_HTTP: &str = "http://schema.org/paymentDueDate";
/// <https://schema.org/paymentDueDate>
pub const PAYMENT_DUE_DATE_PROPERTY_IRI_HTTPS: &str = "https://schema.org/paymentDueDate";
/// <https://schema.org/paymentDueDate>
pub const PAYMENT_DUE_DATE_PROPERTY_LABEL: &str = "paymentDueDate";
pub struct PaymentDueDatePropertyIri;
impl PartialEq<&str> for PaymentDueDatePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == PAYMENT_DUE_DATE_PROPERTY_IRI_HTTP
			|| *other == PAYMENT_DUE_DATE_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<PaymentDueDatePropertyIri> for &str {
	fn eq(&self, other: &PaymentDueDatePropertyIri) -> bool {
		*self == PAYMENT_DUE_DATE_PROPERTY_IRI_HTTP || *self == PAYMENT_DUE_DATE_PROPERTY_IRI_HTTPS
	}
}
pub struct PaymentDueDatePropertyIriOrLabel;
impl PartialEq<&str> for PaymentDueDatePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == PaymentDueDatePropertyIri || *other == PAYMENT_DUE_DATE_PROPERTY_LABEL
	}
}
impl PartialEq<PaymentDueDatePropertyIriOrLabel> for &str {
	fn eq(&self, other: &PaymentDueDatePropertyIriOrLabel) -> bool {
		*self == PaymentDueDatePropertyIri || *self == PAYMENT_DUE_DATE_PROPERTY_LABEL
	}
}
