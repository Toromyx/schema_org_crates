/// <https://schema.org/orderItemNumber>
pub const ORDER_ITEM_NUMBER_PROPERTY_IRI_HTTP: &str = "http://schema.org/orderItemNumber";
/// <https://schema.org/orderItemNumber>
pub const ORDER_ITEM_NUMBER_PROPERTY_IRI_HTTPS: &str = "https://schema.org/orderItemNumber";
/// <https://schema.org/orderItemNumber>
pub const ORDER_ITEM_NUMBER_PROPERTY_LABEL: &str = "orderItemNumber";
pub struct OrderItemNumberPropertyIri;
impl PartialEq<&str> for OrderItemNumberPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == ORDER_ITEM_NUMBER_PROPERTY_IRI_HTTP
			|| *other == ORDER_ITEM_NUMBER_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<OrderItemNumberPropertyIri> for &str {
	fn eq(&self, other: &OrderItemNumberPropertyIri) -> bool {
		*self == ORDER_ITEM_NUMBER_PROPERTY_IRI_HTTP
			|| *self == ORDER_ITEM_NUMBER_PROPERTY_IRI_HTTPS
	}
}
pub struct OrderItemNumberPropertyIriOrLabel;
impl PartialEq<&str> for OrderItemNumberPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == OrderItemNumberPropertyIri || *other == ORDER_ITEM_NUMBER_PROPERTY_LABEL
	}
}
impl PartialEq<OrderItemNumberPropertyIriOrLabel> for &str {
	fn eq(&self, other: &OrderItemNumberPropertyIriOrLabel) -> bool {
		*self == OrderItemNumberPropertyIri || *self == ORDER_ITEM_NUMBER_PROPERTY_LABEL
	}
}
