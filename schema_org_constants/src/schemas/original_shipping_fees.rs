/// <https://schema.org/OriginalShippingFees>
pub const ORIGINAL_SHIPPING_FEES_IRI_HTTP: &str = "http://schema.org/OriginalShippingFees";
/// <https://schema.org/OriginalShippingFees>
pub const ORIGINAL_SHIPPING_FEES_IRI_HTTPS: &str = "https://schema.org/OriginalShippingFees";
/// <https://schema.org/OriginalShippingFees>
pub const ORIGINAL_SHIPPING_FEES_LABEL: &str = "OriginalShippingFees";
pub struct OriginalShippingFeesIri;
impl PartialEq<&str> for OriginalShippingFeesIri {
	fn eq(&self, other: &&str) -> bool {
		*other == ORIGINAL_SHIPPING_FEES_IRI_HTTP || *other == ORIGINAL_SHIPPING_FEES_IRI_HTTPS
	}
}
impl PartialEq<OriginalShippingFeesIri> for &str {
	fn eq(&self, other: &OriginalShippingFeesIri) -> bool {
		*self == ORIGINAL_SHIPPING_FEES_IRI_HTTP || *self == ORIGINAL_SHIPPING_FEES_IRI_HTTPS
	}
}
pub struct OriginalShippingFeesIriOrLabel;
impl PartialEq<&str> for OriginalShippingFeesIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == OriginalShippingFeesIri || *other == ORIGINAL_SHIPPING_FEES_LABEL
	}
}
impl PartialEq<OriginalShippingFeesIriOrLabel> for &str {
	fn eq(&self, other: &OriginalShippingFeesIriOrLabel) -> bool {
		*self == OriginalShippingFeesIri || *self == ORIGINAL_SHIPPING_FEES_LABEL
	}
}
