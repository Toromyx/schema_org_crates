/// <https://schema.org/shippingOrigin>
pub const SHIPPING_ORIGIN_PROPERTY_IRI_HTTP: &str = "http://schema.org/shippingOrigin";
/// <https://schema.org/shippingOrigin>
pub const SHIPPING_ORIGIN_PROPERTY_IRI_HTTPS: &str = "https://schema.org/shippingOrigin";
/// <https://schema.org/shippingOrigin>
pub const SHIPPING_ORIGIN_PROPERTY_LABEL: &str = "shippingOrigin";
pub struct ShippingOriginPropertyIri;
impl PartialEq<&str> for ShippingOriginPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == SHIPPING_ORIGIN_PROPERTY_IRI_HTTP || *other == SHIPPING_ORIGIN_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<ShippingOriginPropertyIri> for &str {
	fn eq(&self, other: &ShippingOriginPropertyIri) -> bool {
		*self == SHIPPING_ORIGIN_PROPERTY_IRI_HTTP || *self == SHIPPING_ORIGIN_PROPERTY_IRI_HTTPS
	}
}
pub struct ShippingOriginPropertyIriOrLabel;
impl PartialEq<&str> for ShippingOriginPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ShippingOriginPropertyIri || *other == SHIPPING_ORIGIN_PROPERTY_LABEL
	}
}
impl PartialEq<ShippingOriginPropertyIriOrLabel> for &str {
	fn eq(&self, other: &ShippingOriginPropertyIriOrLabel) -> bool {
		*self == ShippingOriginPropertyIri || *self == SHIPPING_ORIGIN_PROPERTY_LABEL
	}
}
