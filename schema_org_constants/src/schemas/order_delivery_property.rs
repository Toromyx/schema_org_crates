/// <https://schema.org/orderDelivery>
pub const ORDER_DELIVERY_PROPERTY_IRI_HTTP: &str = "http://schema.org/orderDelivery";
/// <https://schema.org/orderDelivery>
pub const ORDER_DELIVERY_PROPERTY_IRI_HTTPS: &str = "https://schema.org/orderDelivery";
/// <https://schema.org/orderDelivery>
pub const ORDER_DELIVERY_PROPERTY_LABEL: &str = "orderDelivery";
pub struct OrderDeliveryPropertyIri;
impl PartialEq<&str> for OrderDeliveryPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == ORDER_DELIVERY_PROPERTY_IRI_HTTP || *other == ORDER_DELIVERY_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<OrderDeliveryPropertyIri> for &str {
	fn eq(&self, other: &OrderDeliveryPropertyIri) -> bool {
		*self == ORDER_DELIVERY_PROPERTY_IRI_HTTP || *self == ORDER_DELIVERY_PROPERTY_IRI_HTTPS
	}
}
pub struct OrderDeliveryPropertyIriOrLabel;
impl PartialEq<&str> for OrderDeliveryPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == OrderDeliveryPropertyIri || *other == ORDER_DELIVERY_PROPERTY_LABEL
	}
}
impl PartialEq<OrderDeliveryPropertyIriOrLabel> for &str {
	fn eq(&self, other: &OrderDeliveryPropertyIriOrLabel) -> bool {
		*self == OrderDeliveryPropertyIri || *self == ORDER_DELIVERY_PROPERTY_LABEL
	}
}
