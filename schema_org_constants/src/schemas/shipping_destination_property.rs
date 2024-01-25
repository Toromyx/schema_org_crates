/// <https://schema.org/shippingDestination>
pub const SHIPPING_DESTINATION_PROPERTY_IRI_HTTP: &str = "http://schema.org/shippingDestination";
/// <https://schema.org/shippingDestination>
pub const SHIPPING_DESTINATION_PROPERTY_IRI_HTTPS: &str = "https://schema.org/shippingDestination";
/// <https://schema.org/shippingDestination>
pub const SHIPPING_DESTINATION_PROPERTY_LABEL: &str = "shippingDestination";
pub struct ShippingDestinationPropertyIri;
impl PartialEq<&str> for ShippingDestinationPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == SHIPPING_DESTINATION_PROPERTY_IRI_HTTP
			|| *other == SHIPPING_DESTINATION_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<ShippingDestinationPropertyIri> for &str {
	fn eq(&self, other: &ShippingDestinationPropertyIri) -> bool {
		*self == SHIPPING_DESTINATION_PROPERTY_IRI_HTTP
			|| *self == SHIPPING_DESTINATION_PROPERTY_IRI_HTTPS
	}
}
pub struct ShippingDestinationPropertyIriOrLabel;
impl PartialEq<&str> for ShippingDestinationPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ShippingDestinationPropertyIri || *other == SHIPPING_DESTINATION_PROPERTY_LABEL
	}
}
impl PartialEq<ShippingDestinationPropertyIriOrLabel> for &str {
	fn eq(&self, other: &ShippingDestinationPropertyIriOrLabel) -> bool {
		*self == ShippingDestinationPropertyIri || *self == SHIPPING_DESTINATION_PROPERTY_LABEL
	}
}
