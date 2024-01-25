/// <https://schema.org/ReturnShippingFees>
pub const RETURN_SHIPPING_FEES_IRI_HTTP: &str = "http://schema.org/ReturnShippingFees";
/// <https://schema.org/ReturnShippingFees>
pub const RETURN_SHIPPING_FEES_IRI_HTTPS: &str = "https://schema.org/ReturnShippingFees";
/// <https://schema.org/ReturnShippingFees>
pub const RETURN_SHIPPING_FEES_LABEL: &str = "ReturnShippingFees";
pub struct ReturnShippingFeesIri;
impl PartialEq<&str> for ReturnShippingFeesIri {
	fn eq(&self, other: &&str) -> bool {
		*other == RETURN_SHIPPING_FEES_IRI_HTTP || *other == RETURN_SHIPPING_FEES_IRI_HTTPS
	}
}
impl PartialEq<ReturnShippingFeesIri> for &str {
	fn eq(&self, other: &ReturnShippingFeesIri) -> bool {
		*self == RETURN_SHIPPING_FEES_IRI_HTTP || *self == RETURN_SHIPPING_FEES_IRI_HTTPS
	}
}
pub struct ReturnShippingFeesIriOrLabel;
impl PartialEq<&str> for ReturnShippingFeesIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ReturnShippingFeesIri || *other == RETURN_SHIPPING_FEES_LABEL
	}
}
impl PartialEq<ReturnShippingFeesIriOrLabel> for &str {
	fn eq(&self, other: &ReturnShippingFeesIriOrLabel) -> bool {
		*self == ReturnShippingFeesIri || *self == RETURN_SHIPPING_FEES_LABEL
	}
}
