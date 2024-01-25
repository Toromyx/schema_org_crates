/// <https://schema.org/appliesToPaymentMethod>
pub const APPLIES_TO_PAYMENT_METHOD_PROPERTY_IRI_HTTP: &str =
	"http://schema.org/appliesToPaymentMethod";
/// <https://schema.org/appliesToPaymentMethod>
pub const APPLIES_TO_PAYMENT_METHOD_PROPERTY_IRI_HTTPS: &str =
	"https://schema.org/appliesToPaymentMethod";
/// <https://schema.org/appliesToPaymentMethod>
pub const APPLIES_TO_PAYMENT_METHOD_PROPERTY_LABEL: &str = "appliesToPaymentMethod";
pub struct AppliesToPaymentMethodPropertyIri;
impl PartialEq<&str> for AppliesToPaymentMethodPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == APPLIES_TO_PAYMENT_METHOD_PROPERTY_IRI_HTTP
			|| *other == APPLIES_TO_PAYMENT_METHOD_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<AppliesToPaymentMethodPropertyIri> for &str {
	fn eq(&self, other: &AppliesToPaymentMethodPropertyIri) -> bool {
		*self == APPLIES_TO_PAYMENT_METHOD_PROPERTY_IRI_HTTP
			|| *self == APPLIES_TO_PAYMENT_METHOD_PROPERTY_IRI_HTTPS
	}
}
pub struct AppliesToPaymentMethodPropertyIriOrLabel;
impl PartialEq<&str> for AppliesToPaymentMethodPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == AppliesToPaymentMethodPropertyIri
			|| *other == APPLIES_TO_PAYMENT_METHOD_PROPERTY_LABEL
	}
}
impl PartialEq<AppliesToPaymentMethodPropertyIriOrLabel> for &str {
	fn eq(&self, other: &AppliesToPaymentMethodPropertyIriOrLabel) -> bool {
		*self == AppliesToPaymentMethodPropertyIri
			|| *self == APPLIES_TO_PAYMENT_METHOD_PROPERTY_LABEL
	}
}
