/// <https://schema.org/orderStatus>
pub const ORDER_STATUS_PROPERTY_IRI_HTTP: &str = "http://schema.org/orderStatus";
/// <https://schema.org/orderStatus>
pub const ORDER_STATUS_PROPERTY_IRI_HTTPS: &str = "https://schema.org/orderStatus";
/// <https://schema.org/orderStatus>
pub const ORDER_STATUS_PROPERTY_LABEL: &str = "orderStatus";
pub struct OrderStatusPropertyIri;
impl PartialEq<&str> for OrderStatusPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == ORDER_STATUS_PROPERTY_IRI_HTTP || *other == ORDER_STATUS_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<OrderStatusPropertyIri> for &str {
	fn eq(&self, other: &OrderStatusPropertyIri) -> bool {
		*self == ORDER_STATUS_PROPERTY_IRI_HTTP || *self == ORDER_STATUS_PROPERTY_IRI_HTTPS
	}
}
pub struct OrderStatusPropertyIriOrLabel;
impl PartialEq<&str> for OrderStatusPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == OrderStatusPropertyIri || *other == ORDER_STATUS_PROPERTY_LABEL
	}
}
impl PartialEq<OrderStatusPropertyIriOrLabel> for &str {
	fn eq(&self, other: &OrderStatusPropertyIriOrLabel) -> bool {
		*self == OrderStatusPropertyIri || *self == ORDER_STATUS_PROPERTY_LABEL
	}
}
