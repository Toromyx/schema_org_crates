/// <https://schema.org/contactlessPayment>
pub const CONTACTLESS_PAYMENT_PROPERTY_IRI_HTTP: &str = "http://schema.org/contactlessPayment";
/// <https://schema.org/contactlessPayment>
pub const CONTACTLESS_PAYMENT_PROPERTY_IRI_HTTPS: &str = "https://schema.org/contactlessPayment";
/// <https://schema.org/contactlessPayment>
pub const CONTACTLESS_PAYMENT_PROPERTY_LABEL: &str = "contactlessPayment";
pub struct ContactlessPaymentPropertyIri;
impl PartialEq<&str> for ContactlessPaymentPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == CONTACTLESS_PAYMENT_PROPERTY_IRI_HTTP
			|| *other == CONTACTLESS_PAYMENT_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<ContactlessPaymentPropertyIri> for &str {
	fn eq(&self, other: &ContactlessPaymentPropertyIri) -> bool {
		*self == CONTACTLESS_PAYMENT_PROPERTY_IRI_HTTP
			|| *self == CONTACTLESS_PAYMENT_PROPERTY_IRI_HTTPS
	}
}
pub struct ContactlessPaymentPropertyIriOrLabel;
impl PartialEq<&str> for ContactlessPaymentPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ContactlessPaymentPropertyIri || *other == CONTACTLESS_PAYMENT_PROPERTY_LABEL
	}
}
impl PartialEq<ContactlessPaymentPropertyIriOrLabel> for &str {
	fn eq(&self, other: &ContactlessPaymentPropertyIriOrLabel) -> bool {
		*self == ContactlessPaymentPropertyIri || *self == CONTACTLESS_PAYMENT_PROPERTY_LABEL
	}
}
