/// <https://schema.org/ReturnLabelSourceEnumeration>
pub const RETURN_LABEL_SOURCE_ENUMERATION_IRI_HTTP: &str =
	"http://schema.org/ReturnLabelSourceEnumeration";
/// <https://schema.org/ReturnLabelSourceEnumeration>
pub const RETURN_LABEL_SOURCE_ENUMERATION_IRI_HTTPS: &str =
	"https://schema.org/ReturnLabelSourceEnumeration";
/// <https://schema.org/ReturnLabelSourceEnumeration>
pub const RETURN_LABEL_SOURCE_ENUMERATION_LABEL: &str = "ReturnLabelSourceEnumeration";
pub struct ReturnLabelSourceEnumerationIri;
impl PartialEq<&str> for ReturnLabelSourceEnumerationIri {
	fn eq(&self, other: &&str) -> bool {
		*other == RETURN_LABEL_SOURCE_ENUMERATION_IRI_HTTP
			|| *other == RETURN_LABEL_SOURCE_ENUMERATION_IRI_HTTPS
	}
}
impl PartialEq<ReturnLabelSourceEnumerationIri> for &str {
	fn eq(&self, other: &ReturnLabelSourceEnumerationIri) -> bool {
		*self == RETURN_LABEL_SOURCE_ENUMERATION_IRI_HTTP
			|| *self == RETURN_LABEL_SOURCE_ENUMERATION_IRI_HTTPS
	}
}
pub struct ReturnLabelSourceEnumerationIriOrLabel;
impl PartialEq<&str> for ReturnLabelSourceEnumerationIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ReturnLabelSourceEnumerationIri || *other == RETURN_LABEL_SOURCE_ENUMERATION_LABEL
	}
}
impl PartialEq<ReturnLabelSourceEnumerationIriOrLabel> for &str {
	fn eq(&self, other: &ReturnLabelSourceEnumerationIriOrLabel) -> bool {
		*self == ReturnLabelSourceEnumerationIri || *self == RETURN_LABEL_SOURCE_ENUMERATION_LABEL
	}
}
