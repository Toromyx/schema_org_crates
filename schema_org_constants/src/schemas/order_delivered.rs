/// <https://schema.org/OrderDelivered>
pub const ORDER_DELIVERED_IRI_HTTP: &str = "http://schema.org/OrderDelivered";
/// <https://schema.org/OrderDelivered>
pub const ORDER_DELIVERED_IRI_HTTPS: &str = "https://schema.org/OrderDelivered";
/// <https://schema.org/OrderDelivered>
pub const ORDER_DELIVERED_LABEL: &str = "OrderDelivered";
pub struct OrderDeliveredIri;
impl PartialEq<&str> for OrderDeliveredIri {
	fn eq(&self, other: &&str) -> bool {
		*other == ORDER_DELIVERED_IRI_HTTP || *other == ORDER_DELIVERED_IRI_HTTPS
	}
}
impl PartialEq<OrderDeliveredIri> for &str {
	fn eq(&self, other: &OrderDeliveredIri) -> bool {
		*self == ORDER_DELIVERED_IRI_HTTP || *self == ORDER_DELIVERED_IRI_HTTPS
	}
}
pub struct OrderDeliveredIriOrLabel;
impl PartialEq<&str> for OrderDeliveredIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == OrderDeliveredIri || *other == ORDER_DELIVERED_LABEL
	}
}
impl PartialEq<OrderDeliveredIriOrLabel> for &str {
	fn eq(&self, other: &OrderDeliveredIriOrLabel) -> bool {
		*self == OrderDeliveredIri || *self == ORDER_DELIVERED_LABEL
	}
}
