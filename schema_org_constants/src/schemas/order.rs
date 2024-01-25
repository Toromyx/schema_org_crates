/// <https://schema.org/Order>
pub const ORDER_IRI_HTTP: &str = "http://schema.org/Order";
/// <https://schema.org/Order>
pub const ORDER_IRI_HTTPS: &str = "https://schema.org/Order";
/// <https://schema.org/Order>
pub const ORDER_LABEL: &str = "Order";
pub struct OrderIri;
impl PartialEq<&str> for OrderIri {
	fn eq(&self, other: &&str) -> bool {
		*other == ORDER_IRI_HTTP || *other == ORDER_IRI_HTTPS
	}
}
impl PartialEq<OrderIri> for &str {
	fn eq(&self, other: &OrderIri) -> bool {
		*self == ORDER_IRI_HTTP || *self == ORDER_IRI_HTTPS
	}
}
pub struct OrderIriOrLabel;
impl PartialEq<&str> for OrderIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == OrderIri || *other == ORDER_LABEL
	}
}
impl PartialEq<OrderIriOrLabel> for &str {
	fn eq(&self, other: &OrderIriOrLabel) -> bool {
		*self == OrderIri || *self == ORDER_LABEL
	}
}
