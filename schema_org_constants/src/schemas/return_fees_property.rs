/// <https://schema.org/returnFees>
pub const RETURN_FEES_PROPERTY_IRI_HTTP: &str = "http://schema.org/returnFees";
/// <https://schema.org/returnFees>
pub const RETURN_FEES_PROPERTY_IRI_HTTPS: &str = "https://schema.org/returnFees";
/// <https://schema.org/returnFees>
pub const RETURN_FEES_PROPERTY_LABEL: &str = "returnFees";
pub struct ReturnFeesPropertyIri;
impl PartialEq<&str> for ReturnFeesPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == RETURN_FEES_PROPERTY_IRI_HTTP || *other == RETURN_FEES_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<ReturnFeesPropertyIri> for &str {
	fn eq(&self, other: &ReturnFeesPropertyIri) -> bool {
		*self == RETURN_FEES_PROPERTY_IRI_HTTP || *self == RETURN_FEES_PROPERTY_IRI_HTTPS
	}
}
pub struct ReturnFeesPropertyIriOrLabel;
impl PartialEq<&str> for ReturnFeesPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ReturnFeesPropertyIri || *other == RETURN_FEES_PROPERTY_LABEL
	}
}
impl PartialEq<ReturnFeesPropertyIriOrLabel> for &str {
	fn eq(&self, other: &ReturnFeesPropertyIriOrLabel) -> bool {
		*self == ReturnFeesPropertyIri || *self == RETURN_FEES_PROPERTY_LABEL
	}
}
