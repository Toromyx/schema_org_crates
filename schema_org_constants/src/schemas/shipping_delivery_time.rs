/// <https://schema.org/ShippingDeliveryTime>
pub const SHIPPING_DELIVERY_TIME_IRI_HTTP: &str = "http://schema.org/ShippingDeliveryTime";
/// <https://schema.org/ShippingDeliveryTime>
pub const SHIPPING_DELIVERY_TIME_IRI_HTTPS: &str = "https://schema.org/ShippingDeliveryTime";
/// <https://schema.org/ShippingDeliveryTime>
pub const SHIPPING_DELIVERY_TIME_LABEL: &str = "ShippingDeliveryTime";
pub struct ShippingDeliveryTimeIri;
impl PartialEq<&str> for ShippingDeliveryTimeIri {
	fn eq(&self, other: &&str) -> bool {
		*other == SHIPPING_DELIVERY_TIME_IRI_HTTP || *other == SHIPPING_DELIVERY_TIME_IRI_HTTPS
	}
}
impl PartialEq<ShippingDeliveryTimeIri> for &str {
	fn eq(&self, other: &ShippingDeliveryTimeIri) -> bool {
		*self == SHIPPING_DELIVERY_TIME_IRI_HTTP || *self == SHIPPING_DELIVERY_TIME_IRI_HTTPS
	}
}
pub struct ShippingDeliveryTimeIriOrLabel;
impl PartialEq<&str> for ShippingDeliveryTimeIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ShippingDeliveryTimeIri || *other == SHIPPING_DELIVERY_TIME_LABEL
	}
}
impl PartialEq<ShippingDeliveryTimeIriOrLabel> for &str {
	fn eq(&self, other: &ShippingDeliveryTimeIriOrLabel) -> bool {
		*self == ShippingDeliveryTimeIri || *self == SHIPPING_DELIVERY_TIME_LABEL
	}
}
