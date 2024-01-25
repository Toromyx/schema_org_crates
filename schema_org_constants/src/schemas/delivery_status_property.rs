/// <https://schema.org/deliveryStatus>
pub const DELIVERY_STATUS_PROPERTY_IRI_HTTP: &str = "http://schema.org/deliveryStatus";
/// <https://schema.org/deliveryStatus>
pub const DELIVERY_STATUS_PROPERTY_IRI_HTTPS: &str = "https://schema.org/deliveryStatus";
/// <https://schema.org/deliveryStatus>
pub const DELIVERY_STATUS_PROPERTY_LABEL: &str = "deliveryStatus";
pub struct DeliveryStatusPropertyIri;
impl PartialEq<&str> for DeliveryStatusPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == DELIVERY_STATUS_PROPERTY_IRI_HTTP || *other == DELIVERY_STATUS_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<DeliveryStatusPropertyIri> for &str {
	fn eq(&self, other: &DeliveryStatusPropertyIri) -> bool {
		*self == DELIVERY_STATUS_PROPERTY_IRI_HTTP || *self == DELIVERY_STATUS_PROPERTY_IRI_HTTPS
	}
}
pub struct DeliveryStatusPropertyIriOrLabel;
impl PartialEq<&str> for DeliveryStatusPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == DeliveryStatusPropertyIri || *other == DELIVERY_STATUS_PROPERTY_LABEL
	}
}
impl PartialEq<DeliveryStatusPropertyIriOrLabel> for &str {
	fn eq(&self, other: &DeliveryStatusPropertyIriOrLabel) -> bool {
		*self == DeliveryStatusPropertyIri || *self == DELIVERY_STATUS_PROPERTY_LABEL
	}
}
