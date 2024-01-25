/// <https://schema.org/checkoutTime>
pub const CHECKOUT_TIME_PROPERTY_IRI_HTTP: &str = "http://schema.org/checkoutTime";
/// <https://schema.org/checkoutTime>
pub const CHECKOUT_TIME_PROPERTY_IRI_HTTPS: &str = "https://schema.org/checkoutTime";
/// <https://schema.org/checkoutTime>
pub const CHECKOUT_TIME_PROPERTY_LABEL: &str = "checkoutTime";
pub struct CheckoutTimePropertyIri;
impl PartialEq<&str> for CheckoutTimePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == CHECKOUT_TIME_PROPERTY_IRI_HTTP || *other == CHECKOUT_TIME_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<CheckoutTimePropertyIri> for &str {
	fn eq(&self, other: &CheckoutTimePropertyIri) -> bool {
		*self == CHECKOUT_TIME_PROPERTY_IRI_HTTP || *self == CHECKOUT_TIME_PROPERTY_IRI_HTTPS
	}
}
pub struct CheckoutTimePropertyIriOrLabel;
impl PartialEq<&str> for CheckoutTimePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == CheckoutTimePropertyIri || *other == CHECKOUT_TIME_PROPERTY_LABEL
	}
}
impl PartialEq<CheckoutTimePropertyIriOrLabel> for &str {
	fn eq(&self, other: &CheckoutTimePropertyIriOrLabel) -> bool {
		*self == CheckoutTimePropertyIri || *self == CHECKOUT_TIME_PROPERTY_LABEL
	}
}
