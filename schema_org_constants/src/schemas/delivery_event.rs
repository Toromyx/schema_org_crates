/// <https://schema.org/DeliveryEvent>
pub const DELIVERY_EVENT_IRI_HTTP: &str = "http://schema.org/DeliveryEvent";
/// <https://schema.org/DeliveryEvent>
pub const DELIVERY_EVENT_IRI_HTTPS: &str = "https://schema.org/DeliveryEvent";
/// <https://schema.org/DeliveryEvent>
pub const DELIVERY_EVENT_LABEL: &str = "DeliveryEvent";
pub struct DeliveryEventIri;
impl PartialEq<&str> for DeliveryEventIri {
	fn eq(&self, other: &&str) -> bool {
		*other == DELIVERY_EVENT_IRI_HTTP || *other == DELIVERY_EVENT_IRI_HTTPS
	}
}
impl PartialEq<DeliveryEventIri> for &str {
	fn eq(&self, other: &DeliveryEventIri) -> bool {
		*self == DELIVERY_EVENT_IRI_HTTP || *self == DELIVERY_EVENT_IRI_HTTPS
	}
}
pub struct DeliveryEventIriOrLabel;
impl PartialEq<&str> for DeliveryEventIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == DeliveryEventIri || *other == DELIVERY_EVENT_LABEL
	}
}
impl PartialEq<DeliveryEventIriOrLabel> for &str {
	fn eq(&self, other: &DeliveryEventIriOrLabel) -> bool {
		*self == DeliveryEventIri || *self == DELIVERY_EVENT_LABEL
	}
}
