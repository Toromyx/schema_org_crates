/// <https://schema.org/deliveryTime>
pub const DELIVERY_TIME_PROPERTY_IRI_HTTP: &str = "http://schema.org/deliveryTime";
/// <https://schema.org/deliveryTime>
pub const DELIVERY_TIME_PROPERTY_IRI_HTTPS: &str = "https://schema.org/deliveryTime";
/// <https://schema.org/deliveryTime>
pub const DELIVERY_TIME_PROPERTY_LABEL: &str = "deliveryTime";
pub struct DeliveryTimePropertyIri;
impl PartialEq<&str> for DeliveryTimePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == DELIVERY_TIME_PROPERTY_IRI_HTTP || *other == DELIVERY_TIME_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<DeliveryTimePropertyIri> for &str {
	fn eq(&self, other: &DeliveryTimePropertyIri) -> bool {
		*self == DELIVERY_TIME_PROPERTY_IRI_HTTP || *self == DELIVERY_TIME_PROPERTY_IRI_HTTPS
	}
}
pub struct DeliveryTimePropertyIriOrLabel;
impl PartialEq<&str> for DeliveryTimePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == DeliveryTimePropertyIri || *other == DELIVERY_TIME_PROPERTY_LABEL
	}
}
impl PartialEq<DeliveryTimePropertyIriOrLabel> for &str {
	fn eq(&self, other: &DeliveryTimePropertyIriOrLabel) -> bool {
		*self == DeliveryTimePropertyIri || *self == DELIVERY_TIME_PROPERTY_LABEL
	}
}
