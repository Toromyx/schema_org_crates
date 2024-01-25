/// <https://schema.org/orderNumber>
pub const ORDER_NUMBER_PROPERTY_IRI_HTTP: &str = "http://schema.org/orderNumber";
/// <https://schema.org/orderNumber>
pub const ORDER_NUMBER_PROPERTY_IRI_HTTPS: &str = "https://schema.org/orderNumber";
/// <https://schema.org/orderNumber>
pub const ORDER_NUMBER_PROPERTY_LABEL: &str = "orderNumber";
pub struct OrderNumberPropertyIri;
impl PartialEq<&str> for OrderNumberPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == ORDER_NUMBER_PROPERTY_IRI_HTTP || *other == ORDER_NUMBER_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<OrderNumberPropertyIri> for &str {
	fn eq(&self, other: &OrderNumberPropertyIri) -> bool {
		*self == ORDER_NUMBER_PROPERTY_IRI_HTTP || *self == ORDER_NUMBER_PROPERTY_IRI_HTTPS
	}
}
pub struct OrderNumberPropertyIriOrLabel;
impl PartialEq<&str> for OrderNumberPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == OrderNumberPropertyIri || *other == ORDER_NUMBER_PROPERTY_LABEL
	}
}
impl PartialEq<OrderNumberPropertyIriOrLabel> for &str {
	fn eq(&self, other: &OrderNumberPropertyIriOrLabel) -> bool {
		*self == OrderNumberPropertyIri || *self == ORDER_NUMBER_PROPERTY_LABEL
	}
}
