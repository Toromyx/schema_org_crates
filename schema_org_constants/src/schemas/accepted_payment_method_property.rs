/// <https://schema.org/acceptedPaymentMethod>
pub const ACCEPTED_PAYMENT_METHOD_PROPERTY_IRI_HTTP: &str =
	"http://schema.org/acceptedPaymentMethod";
/// <https://schema.org/acceptedPaymentMethod>
pub const ACCEPTED_PAYMENT_METHOD_PROPERTY_IRI_HTTPS: &str =
	"https://schema.org/acceptedPaymentMethod";
/// <https://schema.org/acceptedPaymentMethod>
pub const ACCEPTED_PAYMENT_METHOD_PROPERTY_LABEL: &str = "acceptedPaymentMethod";
pub struct AcceptedPaymentMethodPropertyIri;
impl PartialEq<&str> for AcceptedPaymentMethodPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == ACCEPTED_PAYMENT_METHOD_PROPERTY_IRI_HTTP
			|| *other == ACCEPTED_PAYMENT_METHOD_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<AcceptedPaymentMethodPropertyIri> for &str {
	fn eq(&self, other: &AcceptedPaymentMethodPropertyIri) -> bool {
		*self == ACCEPTED_PAYMENT_METHOD_PROPERTY_IRI_HTTP
			|| *self == ACCEPTED_PAYMENT_METHOD_PROPERTY_IRI_HTTPS
	}
}
pub struct AcceptedPaymentMethodPropertyIriOrLabel;
impl PartialEq<&str> for AcceptedPaymentMethodPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == AcceptedPaymentMethodPropertyIri
			|| *other == ACCEPTED_PAYMENT_METHOD_PROPERTY_LABEL
	}
}
impl PartialEq<AcceptedPaymentMethodPropertyIriOrLabel> for &str {
	fn eq(&self, other: &AcceptedPaymentMethodPropertyIriOrLabel) -> bool {
		*self == AcceptedPaymentMethodPropertyIri || *self == ACCEPTED_PAYMENT_METHOD_PROPERTY_LABEL
	}
}
