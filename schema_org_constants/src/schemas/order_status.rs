/// <https://schema.org/OrderStatus>
pub const ORDER_STATUS_IRI_HTTP: &str = "http://schema.org/OrderStatus";
/// <https://schema.org/OrderStatus>
pub const ORDER_STATUS_IRI_HTTPS: &str = "https://schema.org/OrderStatus";
/// <https://schema.org/OrderStatus>
pub const ORDER_STATUS_LABEL: &str = "OrderStatus";
pub struct OrderStatusIri;
impl PartialEq<&str> for OrderStatusIri {
	fn eq(&self, other: &&str) -> bool {
		*other == ORDER_STATUS_IRI_HTTP || *other == ORDER_STATUS_IRI_HTTPS
	}
}
impl PartialEq<OrderStatusIri> for &str {
	fn eq(&self, other: &OrderStatusIri) -> bool {
		*self == ORDER_STATUS_IRI_HTTP || *self == ORDER_STATUS_IRI_HTTPS
	}
}
pub struct OrderStatusIriOrLabel;
impl PartialEq<&str> for OrderStatusIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == OrderStatusIri || *other == ORDER_STATUS_LABEL
	}
}
impl PartialEq<OrderStatusIriOrLabel> for &str {
	fn eq(&self, other: &OrderStatusIriOrLabel) -> bool {
		*self == OrderStatusIri || *self == ORDER_STATUS_LABEL
	}
}
