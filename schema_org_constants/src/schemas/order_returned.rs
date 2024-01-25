/// <https://schema.org/OrderReturned>
pub const ORDER_RETURNED_IRI_HTTP: &str = "http://schema.org/OrderReturned";
/// <https://schema.org/OrderReturned>
pub const ORDER_RETURNED_IRI_HTTPS: &str = "https://schema.org/OrderReturned";
/// <https://schema.org/OrderReturned>
pub const ORDER_RETURNED_LABEL: &str = "OrderReturned";
pub struct OrderReturnedIri;
impl PartialEq<&str> for OrderReturnedIri {
	fn eq(&self, other: &&str) -> bool {
		*other == ORDER_RETURNED_IRI_HTTP || *other == ORDER_RETURNED_IRI_HTTPS
	}
}
impl PartialEq<OrderReturnedIri> for &str {
	fn eq(&self, other: &OrderReturnedIri) -> bool {
		*self == ORDER_RETURNED_IRI_HTTP || *self == ORDER_RETURNED_IRI_HTTPS
	}
}
pub struct OrderReturnedIriOrLabel;
impl PartialEq<&str> for OrderReturnedIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == OrderReturnedIri || *other == ORDER_RETURNED_LABEL
	}
}
impl PartialEq<OrderReturnedIriOrLabel> for &str {
	fn eq(&self, other: &OrderReturnedIriOrLabel) -> bool {
		*self == OrderReturnedIri || *self == ORDER_RETURNED_LABEL
	}
}
