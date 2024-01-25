/// <https://schema.org/StoreCreditRefund>
pub const STORE_CREDIT_REFUND_IRI_HTTP: &str = "http://schema.org/StoreCreditRefund";
/// <https://schema.org/StoreCreditRefund>
pub const STORE_CREDIT_REFUND_IRI_HTTPS: &str = "https://schema.org/StoreCreditRefund";
/// <https://schema.org/StoreCreditRefund>
pub const STORE_CREDIT_REFUND_LABEL: &str = "StoreCreditRefund";
pub struct StoreCreditRefundIri;
impl PartialEq<&str> for StoreCreditRefundIri {
	fn eq(&self, other: &&str) -> bool {
		*other == STORE_CREDIT_REFUND_IRI_HTTP || *other == STORE_CREDIT_REFUND_IRI_HTTPS
	}
}
impl PartialEq<StoreCreditRefundIri> for &str {
	fn eq(&self, other: &StoreCreditRefundIri) -> bool {
		*self == STORE_CREDIT_REFUND_IRI_HTTP || *self == STORE_CREDIT_REFUND_IRI_HTTPS
	}
}
pub struct StoreCreditRefundIriOrLabel;
impl PartialEq<&str> for StoreCreditRefundIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == StoreCreditRefundIri || *other == STORE_CREDIT_REFUND_LABEL
	}
}
impl PartialEq<StoreCreditRefundIriOrLabel> for &str {
	fn eq(&self, other: &StoreCreditRefundIriOrLabel) -> bool {
		*self == StoreCreditRefundIri || *self == STORE_CREDIT_REFUND_LABEL
	}
}
