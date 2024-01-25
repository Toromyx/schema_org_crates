/// <https://schema.org/ReturnFeesEnumeration>
pub const RETURN_FEES_ENUMERATION_IRI_HTTP: &str = "http://schema.org/ReturnFeesEnumeration";
/// <https://schema.org/ReturnFeesEnumeration>
pub const RETURN_FEES_ENUMERATION_IRI_HTTPS: &str = "https://schema.org/ReturnFeesEnumeration";
/// <https://schema.org/ReturnFeesEnumeration>
pub const RETURN_FEES_ENUMERATION_LABEL: &str = "ReturnFeesEnumeration";
pub struct ReturnFeesEnumerationIri;
impl PartialEq<&str> for ReturnFeesEnumerationIri {
	fn eq(&self, other: &&str) -> bool {
		*other == RETURN_FEES_ENUMERATION_IRI_HTTP || *other == RETURN_FEES_ENUMERATION_IRI_HTTPS
	}
}
impl PartialEq<ReturnFeesEnumerationIri> for &str {
	fn eq(&self, other: &ReturnFeesEnumerationIri) -> bool {
		*self == RETURN_FEES_ENUMERATION_IRI_HTTP || *self == RETURN_FEES_ENUMERATION_IRI_HTTPS
	}
}
pub struct ReturnFeesEnumerationIriOrLabel;
impl PartialEq<&str> for ReturnFeesEnumerationIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ReturnFeesEnumerationIri || *other == RETURN_FEES_ENUMERATION_LABEL
	}
}
impl PartialEq<ReturnFeesEnumerationIriOrLabel> for &str {
	fn eq(&self, other: &ReturnFeesEnumerationIriOrLabel) -> bool {
		*self == ReturnFeesEnumerationIri || *self == RETURN_FEES_ENUMERATION_LABEL
	}
}
