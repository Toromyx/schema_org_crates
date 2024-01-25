/// <https://schema.org/ExchangeRefund>
pub const EXCHANGE_REFUND_IRI_HTTP: &str = "http://schema.org/ExchangeRefund";
/// <https://schema.org/ExchangeRefund>
pub const EXCHANGE_REFUND_IRI_HTTPS: &str = "https://schema.org/ExchangeRefund";
/// <https://schema.org/ExchangeRefund>
pub const EXCHANGE_REFUND_LABEL: &str = "ExchangeRefund";
pub struct ExchangeRefundIri;
impl PartialEq<&str> for ExchangeRefundIri {
	fn eq(&self, other: &&str) -> bool {
		*other == EXCHANGE_REFUND_IRI_HTTP || *other == EXCHANGE_REFUND_IRI_HTTPS
	}
}
impl PartialEq<ExchangeRefundIri> for &str {
	fn eq(&self, other: &ExchangeRefundIri) -> bool {
		*self == EXCHANGE_REFUND_IRI_HTTP || *self == EXCHANGE_REFUND_IRI_HTTPS
	}
}
pub struct ExchangeRefundIriOrLabel;
impl PartialEq<&str> for ExchangeRefundIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ExchangeRefundIri || *other == EXCHANGE_REFUND_LABEL
	}
}
impl PartialEq<ExchangeRefundIriOrLabel> for &str {
	fn eq(&self, other: &ExchangeRefundIriOrLabel) -> bool {
		*self == ExchangeRefundIri || *self == EXCHANGE_REFUND_LABEL
	}
}
