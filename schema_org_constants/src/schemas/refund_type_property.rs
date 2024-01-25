/// <https://schema.org/refundType>
pub const REFUND_TYPE_PROPERTY_IRI_HTTP: &str = "http://schema.org/refundType";
/// <https://schema.org/refundType>
pub const REFUND_TYPE_PROPERTY_IRI_HTTPS: &str = "https://schema.org/refundType";
/// <https://schema.org/refundType>
pub const REFUND_TYPE_PROPERTY_LABEL: &str = "refundType";
pub struct RefundTypePropertyIri;
impl PartialEq<&str> for RefundTypePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == REFUND_TYPE_PROPERTY_IRI_HTTP || *other == REFUND_TYPE_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<RefundTypePropertyIri> for &str {
	fn eq(&self, other: &RefundTypePropertyIri) -> bool {
		*self == REFUND_TYPE_PROPERTY_IRI_HTTP || *self == REFUND_TYPE_PROPERTY_IRI_HTTPS
	}
}
pub struct RefundTypePropertyIriOrLabel;
impl PartialEq<&str> for RefundTypePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == RefundTypePropertyIri || *other == REFUND_TYPE_PROPERTY_LABEL
	}
}
impl PartialEq<RefundTypePropertyIriOrLabel> for &str {
	fn eq(&self, other: &RefundTypePropertyIriOrLabel) -> bool {
		*self == RefundTypePropertyIri || *self == REFUND_TYPE_PROPERTY_LABEL
	}
}
