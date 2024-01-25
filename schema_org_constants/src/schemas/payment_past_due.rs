/// <https://schema.org/PaymentPastDue>
pub const PAYMENT_PAST_DUE_IRI_HTTP: &str = "http://schema.org/PaymentPastDue";
/// <https://schema.org/PaymentPastDue>
pub const PAYMENT_PAST_DUE_IRI_HTTPS: &str = "https://schema.org/PaymentPastDue";
/// <https://schema.org/PaymentPastDue>
pub const PAYMENT_PAST_DUE_LABEL: &str = "PaymentPastDue";
pub struct PaymentPastDueIri;
impl PartialEq<&str> for PaymentPastDueIri {
	fn eq(&self, other: &&str) -> bool {
		*other == PAYMENT_PAST_DUE_IRI_HTTP || *other == PAYMENT_PAST_DUE_IRI_HTTPS
	}
}
impl PartialEq<PaymentPastDueIri> for &str {
	fn eq(&self, other: &PaymentPastDueIri) -> bool {
		*self == PAYMENT_PAST_DUE_IRI_HTTP || *self == PAYMENT_PAST_DUE_IRI_HTTPS
	}
}
pub struct PaymentPastDueIriOrLabel;
impl PartialEq<&str> for PaymentPastDueIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == PaymentPastDueIri || *other == PAYMENT_PAST_DUE_LABEL
	}
}
impl PartialEq<PaymentPastDueIriOrLabel> for &str {
	fn eq(&self, other: &PaymentPastDueIriOrLabel) -> bool {
		*self == PaymentPastDueIri || *self == PAYMENT_PAST_DUE_LABEL
	}
}
