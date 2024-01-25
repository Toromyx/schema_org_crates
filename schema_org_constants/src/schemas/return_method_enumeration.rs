/// <https://schema.org/ReturnMethodEnumeration>
pub const RETURN_METHOD_ENUMERATION_IRI_HTTP: &str = "http://schema.org/ReturnMethodEnumeration";
/// <https://schema.org/ReturnMethodEnumeration>
pub const RETURN_METHOD_ENUMERATION_IRI_HTTPS: &str = "https://schema.org/ReturnMethodEnumeration";
/// <https://schema.org/ReturnMethodEnumeration>
pub const RETURN_METHOD_ENUMERATION_LABEL: &str = "ReturnMethodEnumeration";
pub struct ReturnMethodEnumerationIri;
impl PartialEq<&str> for ReturnMethodEnumerationIri {
	fn eq(&self, other: &&str) -> bool {
		*other == RETURN_METHOD_ENUMERATION_IRI_HTTP
			|| *other == RETURN_METHOD_ENUMERATION_IRI_HTTPS
	}
}
impl PartialEq<ReturnMethodEnumerationIri> for &str {
	fn eq(&self, other: &ReturnMethodEnumerationIri) -> bool {
		*self == RETURN_METHOD_ENUMERATION_IRI_HTTP || *self == RETURN_METHOD_ENUMERATION_IRI_HTTPS
	}
}
pub struct ReturnMethodEnumerationIriOrLabel;
impl PartialEq<&str> for ReturnMethodEnumerationIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ReturnMethodEnumerationIri || *other == RETURN_METHOD_ENUMERATION_LABEL
	}
}
impl PartialEq<ReturnMethodEnumerationIriOrLabel> for &str {
	fn eq(&self, other: &ReturnMethodEnumerationIriOrLabel) -> bool {
		*self == ReturnMethodEnumerationIri || *self == RETURN_METHOD_ENUMERATION_LABEL
	}
}
