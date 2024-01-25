/// <https://schema.org/shippingDetails>
pub const SHIPPING_DETAILS_PROPERTY_IRI_HTTP: &str = "http://schema.org/shippingDetails";
/// <https://schema.org/shippingDetails>
pub const SHIPPING_DETAILS_PROPERTY_IRI_HTTPS: &str = "https://schema.org/shippingDetails";
/// <https://schema.org/shippingDetails>
pub const SHIPPING_DETAILS_PROPERTY_LABEL: &str = "shippingDetails";
pub struct ShippingDetailsPropertyIri;
impl PartialEq<&str> for ShippingDetailsPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == SHIPPING_DETAILS_PROPERTY_IRI_HTTP
			|| *other == SHIPPING_DETAILS_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<ShippingDetailsPropertyIri> for &str {
	fn eq(&self, other: &ShippingDetailsPropertyIri) -> bool {
		*self == SHIPPING_DETAILS_PROPERTY_IRI_HTTP || *self == SHIPPING_DETAILS_PROPERTY_IRI_HTTPS
	}
}
pub struct ShippingDetailsPropertyIriOrLabel;
impl PartialEq<&str> for ShippingDetailsPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ShippingDetailsPropertyIri || *other == SHIPPING_DETAILS_PROPERTY_LABEL
	}
}
impl PartialEq<ShippingDetailsPropertyIriOrLabel> for &str {
	fn eq(&self, other: &ShippingDetailsPropertyIriOrLabel) -> bool {
		*self == ShippingDetailsPropertyIri || *self == SHIPPING_DETAILS_PROPERTY_LABEL
	}
}
