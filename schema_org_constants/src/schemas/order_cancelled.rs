/// <https://schema.org/OrderCancelled>
pub const ORDER_CANCELLED_IRI_HTTP: &str = "http://schema.org/OrderCancelled";
/// <https://schema.org/OrderCancelled>
pub const ORDER_CANCELLED_IRI_HTTPS: &str = "https://schema.org/OrderCancelled";
/// <https://schema.org/OrderCancelled>
pub const ORDER_CANCELLED_LABEL: &str = "OrderCancelled";
pub struct OrderCancelledIri;
impl PartialEq<&str> for OrderCancelledIri {
	fn eq(&self, other: &&str) -> bool {
		*other == ORDER_CANCELLED_IRI_HTTP || *other == ORDER_CANCELLED_IRI_HTTPS
	}
}
impl PartialEq<OrderCancelledIri> for &str {
	fn eq(&self, other: &OrderCancelledIri) -> bool {
		*self == ORDER_CANCELLED_IRI_HTTP || *self == ORDER_CANCELLED_IRI_HTTPS
	}
}
pub struct OrderCancelledIriOrLabel;
impl PartialEq<&str> for OrderCancelledIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == OrderCancelledIri || *other == ORDER_CANCELLED_LABEL
	}
}
impl PartialEq<OrderCancelledIriOrLabel> for &str {
	fn eq(&self, other: &OrderCancelledIriOrLabel) -> bool {
		*self == OrderCancelledIri || *self == ORDER_CANCELLED_LABEL
	}
}
