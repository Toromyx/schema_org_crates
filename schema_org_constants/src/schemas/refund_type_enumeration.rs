/// <https://schema.org/RefundTypeEnumeration>
pub const REFUND_TYPE_ENUMERATION_IRI_HTTP: &str = "http://schema.org/RefundTypeEnumeration";
/// <https://schema.org/RefundTypeEnumeration>
pub const REFUND_TYPE_ENUMERATION_IRI_HTTPS: &str = "https://schema.org/RefundTypeEnumeration";
/// <https://schema.org/RefundTypeEnumeration>
pub const REFUND_TYPE_ENUMERATION_LABEL: &str = "RefundTypeEnumeration";
pub struct RefundTypeEnumerationIri;
impl PartialEq<&str> for RefundTypeEnumerationIri {
	fn eq(&self, other: &&str) -> bool {
		*other == REFUND_TYPE_ENUMERATION_IRI_HTTP || *other == REFUND_TYPE_ENUMERATION_IRI_HTTPS
	}
}
impl PartialEq<RefundTypeEnumerationIri> for &str {
	fn eq(&self, other: &RefundTypeEnumerationIri) -> bool {
		*self == REFUND_TYPE_ENUMERATION_IRI_HTTP || *self == REFUND_TYPE_ENUMERATION_IRI_HTTPS
	}
}
pub struct RefundTypeEnumerationIriOrLabel;
impl PartialEq<&str> for RefundTypeEnumerationIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == RefundTypeEnumerationIri || *other == REFUND_TYPE_ENUMERATION_LABEL
	}
}
impl PartialEq<RefundTypeEnumerationIriOrLabel> for &str {
	fn eq(&self, other: &RefundTypeEnumerationIriOrLabel) -> bool {
		*self == RefundTypeEnumerationIri || *self == REFUND_TYPE_ENUMERATION_LABEL
	}
}
