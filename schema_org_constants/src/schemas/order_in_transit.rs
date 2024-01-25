/// <https://schema.org/OrderInTransit>
pub const ORDER_IN_TRANSIT_IRI_HTTP: &str = "http://schema.org/OrderInTransit";
/// <https://schema.org/OrderInTransit>
pub const ORDER_IN_TRANSIT_IRI_HTTPS: &str = "https://schema.org/OrderInTransit";
/// <https://schema.org/OrderInTransit>
pub const ORDER_IN_TRANSIT_LABEL: &str = "OrderInTransit";
pub struct OrderInTransitIri;
impl PartialEq<&str> for OrderInTransitIri {
	fn eq(&self, other: &&str) -> bool {
		*other == ORDER_IN_TRANSIT_IRI_HTTP || *other == ORDER_IN_TRANSIT_IRI_HTTPS
	}
}
impl PartialEq<OrderInTransitIri> for &str {
	fn eq(&self, other: &OrderInTransitIri) -> bool {
		*self == ORDER_IN_TRANSIT_IRI_HTTP || *self == ORDER_IN_TRANSIT_IRI_HTTPS
	}
}
pub struct OrderInTransitIriOrLabel;
impl PartialEq<&str> for OrderInTransitIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == OrderInTransitIri || *other == ORDER_IN_TRANSIT_LABEL
	}
}
impl PartialEq<OrderInTransitIriOrLabel> for &str {
	fn eq(&self, other: &OrderInTransitIriOrLabel) -> bool {
		*self == OrderInTransitIri || *self == ORDER_IN_TRANSIT_LABEL
	}
}
