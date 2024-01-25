/// <https://schema.org/shippingRate>
pub const SHIPPING_RATE_PROPERTY_IRI_HTTP: &str = "http://schema.org/shippingRate";
/// <https://schema.org/shippingRate>
pub const SHIPPING_RATE_PROPERTY_IRI_HTTPS: &str = "https://schema.org/shippingRate";
/// <https://schema.org/shippingRate>
pub const SHIPPING_RATE_PROPERTY_LABEL: &str = "shippingRate";
pub struct ShippingRatePropertyIri;
impl PartialEq<&str> for ShippingRatePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == SHIPPING_RATE_PROPERTY_IRI_HTTP || *other == SHIPPING_RATE_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<ShippingRatePropertyIri> for &str {
	fn eq(&self, other: &ShippingRatePropertyIri) -> bool {
		*self == SHIPPING_RATE_PROPERTY_IRI_HTTP || *self == SHIPPING_RATE_PROPERTY_IRI_HTTPS
	}
}
pub struct ShippingRatePropertyIriOrLabel;
impl PartialEq<&str> for ShippingRatePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ShippingRatePropertyIri || *other == SHIPPING_RATE_PROPERTY_LABEL
	}
}
impl PartialEq<ShippingRatePropertyIriOrLabel> for &str {
	fn eq(&self, other: &ShippingRatePropertyIriOrLabel) -> bool {
		*self == ShippingRatePropertyIri || *self == SHIPPING_RATE_PROPERTY_LABEL
	}
}
