/// <https://schema.org/PaymentAutomaticallyApplied>
pub const PAYMENT_AUTOMATICALLY_APPLIED_IRI_HTTP: &str =
	"http://schema.org/PaymentAutomaticallyApplied";
/// <https://schema.org/PaymentAutomaticallyApplied>
pub const PAYMENT_AUTOMATICALLY_APPLIED_IRI_HTTPS: &str =
	"https://schema.org/PaymentAutomaticallyApplied";
/// <https://schema.org/PaymentAutomaticallyApplied>
pub const PAYMENT_AUTOMATICALLY_APPLIED_LABEL: &str = "PaymentAutomaticallyApplied";
pub struct PaymentAutomaticallyAppliedIri;
impl PartialEq<&str> for PaymentAutomaticallyAppliedIri {
	fn eq(&self, other: &&str) -> bool {
		*other == PAYMENT_AUTOMATICALLY_APPLIED_IRI_HTTP
			|| *other == PAYMENT_AUTOMATICALLY_APPLIED_IRI_HTTPS
	}
}
impl PartialEq<PaymentAutomaticallyAppliedIri> for &str {
	fn eq(&self, other: &PaymentAutomaticallyAppliedIri) -> bool {
		*self == PAYMENT_AUTOMATICALLY_APPLIED_IRI_HTTP
			|| *self == PAYMENT_AUTOMATICALLY_APPLIED_IRI_HTTPS
	}
}
pub struct PaymentAutomaticallyAppliedIriOrLabel;
impl PartialEq<&str> for PaymentAutomaticallyAppliedIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == PaymentAutomaticallyAppliedIri || *other == PAYMENT_AUTOMATICALLY_APPLIED_LABEL
	}
}
impl PartialEq<PaymentAutomaticallyAppliedIriOrLabel> for &str {
	fn eq(&self, other: &PaymentAutomaticallyAppliedIriOrLabel) -> bool {
		*self == PaymentAutomaticallyAppliedIri || *self == PAYMENT_AUTOMATICALLY_APPLIED_LABEL
	}
}
