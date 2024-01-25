/// <https://schema.org/orderItemStatus>
pub const ORDER_ITEM_STATUS_PROPERTY_IRI_HTTP: &str = "http://schema.org/orderItemStatus";
/// <https://schema.org/orderItemStatus>
pub const ORDER_ITEM_STATUS_PROPERTY_IRI_HTTPS: &str = "https://schema.org/orderItemStatus";
/// <https://schema.org/orderItemStatus>
pub const ORDER_ITEM_STATUS_PROPERTY_LABEL: &str = "orderItemStatus";
pub struct OrderItemStatusPropertyIri;
impl PartialEq<&str> for OrderItemStatusPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == ORDER_ITEM_STATUS_PROPERTY_IRI_HTTP
			|| *other == ORDER_ITEM_STATUS_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<OrderItemStatusPropertyIri> for &str {
	fn eq(&self, other: &OrderItemStatusPropertyIri) -> bool {
		*self == ORDER_ITEM_STATUS_PROPERTY_IRI_HTTP
			|| *self == ORDER_ITEM_STATUS_PROPERTY_IRI_HTTPS
	}
}
pub struct OrderItemStatusPropertyIriOrLabel;
impl PartialEq<&str> for OrderItemStatusPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == OrderItemStatusPropertyIri || *other == ORDER_ITEM_STATUS_PROPERTY_LABEL
	}
}
impl PartialEq<OrderItemStatusPropertyIriOrLabel> for &str {
	fn eq(&self, other: &OrderItemStatusPropertyIriOrLabel) -> bool {
		*self == OrderItemStatusPropertyIri || *self == ORDER_ITEM_STATUS_PROPERTY_LABEL
	}
}
