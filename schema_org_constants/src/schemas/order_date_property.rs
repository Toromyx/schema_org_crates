/// <https://schema.org/orderDate>
pub const ORDER_DATE_PROPERTY_IRI_HTTP: &str = "http://schema.org/orderDate";
/// <https://schema.org/orderDate>
pub const ORDER_DATE_PROPERTY_IRI_HTTPS: &str = "https://schema.org/orderDate";
/// <https://schema.org/orderDate>
pub const ORDER_DATE_PROPERTY_LABEL: &str = "orderDate";
pub struct OrderDatePropertyIri;
impl PartialEq<&str> for OrderDatePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == ORDER_DATE_PROPERTY_IRI_HTTP || *other == ORDER_DATE_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<OrderDatePropertyIri> for &str {
	fn eq(&self, other: &OrderDatePropertyIri) -> bool {
		*self == ORDER_DATE_PROPERTY_IRI_HTTP || *self == ORDER_DATE_PROPERTY_IRI_HTTPS
	}
}
pub struct OrderDatePropertyIriOrLabel;
impl PartialEq<&str> for OrderDatePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == OrderDatePropertyIri || *other == ORDER_DATE_PROPERTY_LABEL
	}
}
impl PartialEq<OrderDatePropertyIriOrLabel> for &str {
	fn eq(&self, other: &OrderDatePropertyIriOrLabel) -> bool {
		*self == OrderDatePropertyIri || *self == ORDER_DATE_PROPERTY_LABEL
	}
}
