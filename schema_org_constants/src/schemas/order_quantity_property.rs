/// <https://schema.org/orderQuantity>
pub const ORDER_QUANTITY_PROPERTY_IRI_HTTP: &str = "http://schema.org/orderQuantity";
/// <https://schema.org/orderQuantity>
pub const ORDER_QUANTITY_PROPERTY_IRI_HTTPS: &str = "https://schema.org/orderQuantity";
/// <https://schema.org/orderQuantity>
pub const ORDER_QUANTITY_PROPERTY_LABEL: &str = "orderQuantity";
pub struct OrderQuantityPropertyIri;
impl PartialEq<&str> for OrderQuantityPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == ORDER_QUANTITY_PROPERTY_IRI_HTTP || *other == ORDER_QUANTITY_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<OrderQuantityPropertyIri> for &str {
	fn eq(&self, other: &OrderQuantityPropertyIri) -> bool {
		*self == ORDER_QUANTITY_PROPERTY_IRI_HTTP || *self == ORDER_QUANTITY_PROPERTY_IRI_HTTPS
	}
}
pub struct OrderQuantityPropertyIriOrLabel;
impl PartialEq<&str> for OrderQuantityPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == OrderQuantityPropertyIri || *other == ORDER_QUANTITY_PROPERTY_LABEL
	}
}
impl PartialEq<OrderQuantityPropertyIriOrLabel> for &str {
	fn eq(&self, other: &OrderQuantityPropertyIriOrLabel) -> bool {
		*self == OrderQuantityPropertyIri || *self == ORDER_QUANTITY_PROPERTY_LABEL
	}
}
