/// <https://schema.org/deliveryMethod>
pub const DELIVERY_METHOD_PROPERTY_IRI_HTTP: &str = "http://schema.org/deliveryMethod";
/// <https://schema.org/deliveryMethod>
pub const DELIVERY_METHOD_PROPERTY_IRI_HTTPS: &str = "https://schema.org/deliveryMethod";
/// <https://schema.org/deliveryMethod>
pub const DELIVERY_METHOD_PROPERTY_LABEL: &str = "deliveryMethod";
pub struct DeliveryMethodPropertyIri;
impl PartialEq<&str> for DeliveryMethodPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == DELIVERY_METHOD_PROPERTY_IRI_HTTP || *other == DELIVERY_METHOD_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<DeliveryMethodPropertyIri> for &str {
	fn eq(&self, other: &DeliveryMethodPropertyIri) -> bool {
		*self == DELIVERY_METHOD_PROPERTY_IRI_HTTP || *self == DELIVERY_METHOD_PROPERTY_IRI_HTTPS
	}
}
pub struct DeliveryMethodPropertyIriOrLabel;
impl PartialEq<&str> for DeliveryMethodPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == DeliveryMethodPropertyIri || *other == DELIVERY_METHOD_PROPERTY_LABEL
	}
}
impl PartialEq<DeliveryMethodPropertyIriOrLabel> for &str {
	fn eq(&self, other: &DeliveryMethodPropertyIriOrLabel) -> bool {
		*self == DeliveryMethodPropertyIri || *self == DELIVERY_METHOD_PROPERTY_LABEL
	}
}
