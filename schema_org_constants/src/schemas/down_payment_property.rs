/// <https://schema.org/downPayment>
pub const DOWN_PAYMENT_PROPERTY_IRI_HTTP: &str = "http://schema.org/downPayment";
/// <https://schema.org/downPayment>
pub const DOWN_PAYMENT_PROPERTY_IRI_HTTPS: &str = "https://schema.org/downPayment";
/// <https://schema.org/downPayment>
pub const DOWN_PAYMENT_PROPERTY_LABEL: &str = "downPayment";
pub struct DownPaymentPropertyIri;
impl PartialEq<&str> for DownPaymentPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == DOWN_PAYMENT_PROPERTY_IRI_HTTP || *other == DOWN_PAYMENT_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<DownPaymentPropertyIri> for &str {
	fn eq(&self, other: &DownPaymentPropertyIri) -> bool {
		*self == DOWN_PAYMENT_PROPERTY_IRI_HTTP || *self == DOWN_PAYMENT_PROPERTY_IRI_HTTPS
	}
}
pub struct DownPaymentPropertyIriOrLabel;
impl PartialEq<&str> for DownPaymentPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == DownPaymentPropertyIri || *other == DOWN_PAYMENT_PROPERTY_LABEL
	}
}
impl PartialEq<DownPaymentPropertyIriOrLabel> for &str {
	fn eq(&self, other: &DownPaymentPropertyIriOrLabel) -> bool {
		*self == DownPaymentPropertyIri || *self == DOWN_PAYMENT_PROPERTY_LABEL
	}
}
