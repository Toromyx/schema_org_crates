/// <https://schema.org/FullRefund>
pub const FULL_REFUND_IRI_HTTP: &str = "http://schema.org/FullRefund";
/// <https://schema.org/FullRefund>
pub const FULL_REFUND_IRI_HTTPS: &str = "https://schema.org/FullRefund";
/// <https://schema.org/FullRefund>
pub const FULL_REFUND_LABEL: &str = "FullRefund";
pub struct FullRefundIri;
impl PartialEq<&str> for FullRefundIri {
	fn eq(&self, other: &&str) -> bool {
		*other == FULL_REFUND_IRI_HTTP || *other == FULL_REFUND_IRI_HTTPS
	}
}
impl PartialEq<FullRefundIri> for &str {
	fn eq(&self, other: &FullRefundIri) -> bool {
		*self == FULL_REFUND_IRI_HTTP || *self == FULL_REFUND_IRI_HTTPS
	}
}
pub struct FullRefundIriOrLabel;
impl PartialEq<&str> for FullRefundIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == FullRefundIri || *other == FULL_REFUND_LABEL
	}
}
impl PartialEq<FullRefundIriOrLabel> for &str {
	fn eq(&self, other: &FullRefundIriOrLabel) -> bool {
		*self == FullRefundIri || *self == FULL_REFUND_LABEL
	}
}
