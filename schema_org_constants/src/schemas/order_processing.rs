/// <https://schema.org/OrderProcessing>
pub const ORDER_PROCESSING_IRI_HTTP: &str = "http://schema.org/OrderProcessing";
/// <https://schema.org/OrderProcessing>
pub const ORDER_PROCESSING_IRI_HTTPS: &str = "https://schema.org/OrderProcessing";
/// <https://schema.org/OrderProcessing>
pub const ORDER_PROCESSING_LABEL: &str = "OrderProcessing";
pub struct OrderProcessingIri;
impl PartialEq<&str> for OrderProcessingIri {
	fn eq(&self, other: &&str) -> bool {
		*other == ORDER_PROCESSING_IRI_HTTP || *other == ORDER_PROCESSING_IRI_HTTPS
	}
}
impl PartialEq<OrderProcessingIri> for &str {
	fn eq(&self, other: &OrderProcessingIri) -> bool {
		*self == ORDER_PROCESSING_IRI_HTTP || *self == ORDER_PROCESSING_IRI_HTTPS
	}
}
pub struct OrderProcessingIriOrLabel;
impl PartialEq<&str> for OrderProcessingIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == OrderProcessingIri || *other == ORDER_PROCESSING_LABEL
	}
}
impl PartialEq<OrderProcessingIriOrLabel> for &str {
	fn eq(&self, other: &OrderProcessingIriOrLabel) -> bool {
		*self == OrderProcessingIri || *self == ORDER_PROCESSING_LABEL
	}
}
