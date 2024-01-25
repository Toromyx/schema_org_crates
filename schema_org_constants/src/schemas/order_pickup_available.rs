/// <https://schema.org/OrderPickupAvailable>
pub const ORDER_PICKUP_AVAILABLE_IRI_HTTP: &str = "http://schema.org/OrderPickupAvailable";
/// <https://schema.org/OrderPickupAvailable>
pub const ORDER_PICKUP_AVAILABLE_IRI_HTTPS: &str = "https://schema.org/OrderPickupAvailable";
/// <https://schema.org/OrderPickupAvailable>
pub const ORDER_PICKUP_AVAILABLE_LABEL: &str = "OrderPickupAvailable";
pub struct OrderPickupAvailableIri;
impl PartialEq<&str> for OrderPickupAvailableIri {
	fn eq(&self, other: &&str) -> bool {
		*other == ORDER_PICKUP_AVAILABLE_IRI_HTTP || *other == ORDER_PICKUP_AVAILABLE_IRI_HTTPS
	}
}
impl PartialEq<OrderPickupAvailableIri> for &str {
	fn eq(&self, other: &OrderPickupAvailableIri) -> bool {
		*self == ORDER_PICKUP_AVAILABLE_IRI_HTTP || *self == ORDER_PICKUP_AVAILABLE_IRI_HTTPS
	}
}
pub struct OrderPickupAvailableIriOrLabel;
impl PartialEq<&str> for OrderPickupAvailableIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == OrderPickupAvailableIri || *other == ORDER_PICKUP_AVAILABLE_LABEL
	}
}
impl PartialEq<OrderPickupAvailableIriOrLabel> for &str {
	fn eq(&self, other: &OrderPickupAvailableIriOrLabel) -> bool {
		*self == OrderPickupAvailableIri || *self == ORDER_PICKUP_AVAILABLE_LABEL
	}
}
