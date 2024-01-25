/// <https://schema.org/deliveryAddress>
pub const DELIVERY_ADDRESS_PROPERTY_IRI_HTTP: &str = "http://schema.org/deliveryAddress";
/// <https://schema.org/deliveryAddress>
pub const DELIVERY_ADDRESS_PROPERTY_IRI_HTTPS: &str = "https://schema.org/deliveryAddress";
/// <https://schema.org/deliveryAddress>
pub const DELIVERY_ADDRESS_PROPERTY_LABEL: &str = "deliveryAddress";
pub struct DeliveryAddressPropertyIri;
impl PartialEq<&str> for DeliveryAddressPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == DELIVERY_ADDRESS_PROPERTY_IRI_HTTP
			|| *other == DELIVERY_ADDRESS_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<DeliveryAddressPropertyIri> for &str {
	fn eq(&self, other: &DeliveryAddressPropertyIri) -> bool {
		*self == DELIVERY_ADDRESS_PROPERTY_IRI_HTTP || *self == DELIVERY_ADDRESS_PROPERTY_IRI_HTTPS
	}
}
pub struct DeliveryAddressPropertyIriOrLabel;
impl PartialEq<&str> for DeliveryAddressPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == DeliveryAddressPropertyIri || *other == DELIVERY_ADDRESS_PROPERTY_LABEL
	}
}
impl PartialEq<DeliveryAddressPropertyIriOrLabel> for &str {
	fn eq(&self, other: &DeliveryAddressPropertyIriOrLabel) -> bool {
		*self == DeliveryAddressPropertyIri || *self == DELIVERY_ADDRESS_PROPERTY_LABEL
	}
}
