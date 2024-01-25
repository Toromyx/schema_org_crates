/// <https://schema.org/OrderAction>
pub const ORDER_ACTION_IRI_HTTP: &str = "http://schema.org/OrderAction";
/// <https://schema.org/OrderAction>
pub const ORDER_ACTION_IRI_HTTPS: &str = "https://schema.org/OrderAction";
/// <https://schema.org/OrderAction>
pub const ORDER_ACTION_LABEL: &str = "OrderAction";
pub struct OrderActionIri;
impl PartialEq<&str> for OrderActionIri {
	fn eq(&self, other: &&str) -> bool {
		*other == ORDER_ACTION_IRI_HTTP || *other == ORDER_ACTION_IRI_HTTPS
	}
}
impl PartialEq<OrderActionIri> for &str {
	fn eq(&self, other: &OrderActionIri) -> bool {
		*self == ORDER_ACTION_IRI_HTTP || *self == ORDER_ACTION_IRI_HTTPS
	}
}
pub struct OrderActionIriOrLabel;
impl PartialEq<&str> for OrderActionIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == OrderActionIri || *other == ORDER_ACTION_LABEL
	}
}
impl PartialEq<OrderActionIriOrLabel> for &str {
	fn eq(&self, other: &OrderActionIriOrLabel) -> bool {
		*self == OrderActionIri || *self == ORDER_ACTION_LABEL
	}
}
