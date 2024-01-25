/// <https://schema.org/shippingLabel>
pub const SHIPPING_LABEL_PROPERTY_IRI_HTTP: &str = "http://schema.org/shippingLabel";
/// <https://schema.org/shippingLabel>
pub const SHIPPING_LABEL_PROPERTY_IRI_HTTPS: &str = "https://schema.org/shippingLabel";
/// <https://schema.org/shippingLabel>
pub const SHIPPING_LABEL_PROPERTY_LABEL: &str = "shippingLabel";
pub struct ShippingLabelPropertyIri;
impl PartialEq<&str> for ShippingLabelPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == SHIPPING_LABEL_PROPERTY_IRI_HTTP || *other == SHIPPING_LABEL_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<ShippingLabelPropertyIri> for &str {
	fn eq(&self, other: &ShippingLabelPropertyIri) -> bool {
		*self == SHIPPING_LABEL_PROPERTY_IRI_HTTP || *self == SHIPPING_LABEL_PROPERTY_IRI_HTTPS
	}
}
pub struct ShippingLabelPropertyIriOrLabel;
impl PartialEq<&str> for ShippingLabelPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ShippingLabelPropertyIri || *other == SHIPPING_LABEL_PROPERTY_LABEL
	}
}
impl PartialEq<ShippingLabelPropertyIriOrLabel> for &str {
	fn eq(&self, other: &ShippingLabelPropertyIriOrLabel) -> bool {
		*self == ShippingLabelPropertyIri || *self == SHIPPING_LABEL_PROPERTY_LABEL
	}
}
