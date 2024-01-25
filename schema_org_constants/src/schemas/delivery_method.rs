/// <https://schema.org/DeliveryMethod>
pub const DELIVERY_METHOD_IRI_HTTP: &str = "http://schema.org/DeliveryMethod";
/// <https://schema.org/DeliveryMethod>
pub const DELIVERY_METHOD_IRI_HTTPS: &str = "https://schema.org/DeliveryMethod";
/// <https://schema.org/DeliveryMethod>
pub const DELIVERY_METHOD_LABEL: &str = "DeliveryMethod";
pub struct DeliveryMethodIri;
impl PartialEq<&str> for DeliveryMethodIri {
	fn eq(&self, other: &&str) -> bool {
		*other == DELIVERY_METHOD_IRI_HTTP || *other == DELIVERY_METHOD_IRI_HTTPS
	}
}
impl PartialEq<DeliveryMethodIri> for &str {
	fn eq(&self, other: &DeliveryMethodIri) -> bool {
		*self == DELIVERY_METHOD_IRI_HTTP || *self == DELIVERY_METHOD_IRI_HTTPS
	}
}
pub struct DeliveryMethodIriOrLabel;
impl PartialEq<&str> for DeliveryMethodIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == DeliveryMethodIri || *other == DELIVERY_METHOD_LABEL
	}
}
impl PartialEq<DeliveryMethodIriOrLabel> for &str {
	fn eq(&self, other: &DeliveryMethodIriOrLabel) -> bool {
		*self == DeliveryMethodIri || *self == DELIVERY_METHOD_LABEL
	}
}
