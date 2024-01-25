/// <https://schema.org/OneTimePayments>
pub const ONE_TIME_PAYMENTS_IRI_HTTP: &str = "http://schema.org/OneTimePayments";
/// <https://schema.org/OneTimePayments>
pub const ONE_TIME_PAYMENTS_IRI_HTTPS: &str = "https://schema.org/OneTimePayments";
/// <https://schema.org/OneTimePayments>
pub const ONE_TIME_PAYMENTS_LABEL: &str = "OneTimePayments";
pub struct OneTimePaymentsIri;
impl PartialEq<&str> for OneTimePaymentsIri {
	fn eq(&self, other: &&str) -> bool {
		*other == ONE_TIME_PAYMENTS_IRI_HTTP || *other == ONE_TIME_PAYMENTS_IRI_HTTPS
	}
}
impl PartialEq<OneTimePaymentsIri> for &str {
	fn eq(&self, other: &OneTimePaymentsIri) -> bool {
		*self == ONE_TIME_PAYMENTS_IRI_HTTP || *self == ONE_TIME_PAYMENTS_IRI_HTTPS
	}
}
pub struct OneTimePaymentsIriOrLabel;
impl PartialEq<&str> for OneTimePaymentsIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == OneTimePaymentsIri || *other == ONE_TIME_PAYMENTS_LABEL
	}
}
impl PartialEq<OneTimePaymentsIriOrLabel> for &str {
	fn eq(&self, other: &OneTimePaymentsIriOrLabel) -> bool {
		*self == OneTimePaymentsIri || *self == ONE_TIME_PAYMENTS_LABEL
	}
}
