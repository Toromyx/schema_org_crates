/// <https://schema.org/OrderItem>
pub const ORDER_ITEM_IRI_HTTP: &str = "http://schema.org/OrderItem";
/// <https://schema.org/OrderItem>
pub const ORDER_ITEM_IRI_HTTPS: &str = "https://schema.org/OrderItem";
/// <https://schema.org/OrderItem>
pub const ORDER_ITEM_LABEL: &str = "OrderItem";
pub struct OrderItemIri;
impl PartialEq<&str> for OrderItemIri {
	fn eq(&self, other: &&str) -> bool {
		*other == ORDER_ITEM_IRI_HTTP || *other == ORDER_ITEM_IRI_HTTPS
	}
}
impl PartialEq<OrderItemIri> for &str {
	fn eq(&self, other: &OrderItemIri) -> bool {
		*self == ORDER_ITEM_IRI_HTTP || *self == ORDER_ITEM_IRI_HTTPS
	}
}
pub struct OrderItemIriOrLabel;
impl PartialEq<&str> for OrderItemIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == OrderItemIri || *other == ORDER_ITEM_LABEL
	}
}
impl PartialEq<OrderItemIriOrLabel> for &str {
	fn eq(&self, other: &OrderItemIriOrLabel) -> bool {
		*self == OrderItemIri || *self == ORDER_ITEM_LABEL
	}
}
