/// <https://schema.org/OrderPaymentDue>
pub const ORDER_PAYMENT_DUE_IRI_HTTP: &str = "http://schema.org/OrderPaymentDue";
/// <https://schema.org/OrderPaymentDue>
pub const ORDER_PAYMENT_DUE_IRI_HTTPS: &str = "https://schema.org/OrderPaymentDue";
/// <https://schema.org/OrderPaymentDue>
pub const ORDER_PAYMENT_DUE_LABEL: &str = "OrderPaymentDue";
pub struct OrderPaymentDueIri;
impl PartialEq<&str> for OrderPaymentDueIri {
	fn eq(&self, other: &&str) -> bool {
		*other == ORDER_PAYMENT_DUE_IRI_HTTP || *other == ORDER_PAYMENT_DUE_IRI_HTTPS
	}
}
impl PartialEq<OrderPaymentDueIri> for &str {
	fn eq(&self, other: &OrderPaymentDueIri) -> bool {
		*self == ORDER_PAYMENT_DUE_IRI_HTTP || *self == ORDER_PAYMENT_DUE_IRI_HTTPS
	}
}
pub struct OrderPaymentDueIriOrLabel;
impl PartialEq<&str> for OrderPaymentDueIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == OrderPaymentDueIri || *other == ORDER_PAYMENT_DUE_LABEL
	}
}
impl PartialEq<OrderPaymentDueIriOrLabel> for &str {
	fn eq(&self, other: &OrderPaymentDueIriOrLabel) -> bool {
		*self == OrderPaymentDueIri || *self == ORDER_PAYMENT_DUE_LABEL
	}
}
