/// <https://schema.org/SizeGroupEnumeration>
pub const SIZE_GROUP_ENUMERATION_IRI_HTTP: &str = "http://schema.org/SizeGroupEnumeration";
/// <https://schema.org/SizeGroupEnumeration>
pub const SIZE_GROUP_ENUMERATION_IRI_HTTPS: &str = "https://schema.org/SizeGroupEnumeration";
/// <https://schema.org/SizeGroupEnumeration>
pub const SIZE_GROUP_ENUMERATION_LABEL: &str = "SizeGroupEnumeration";
pub struct SizeGroupEnumerationIri;
impl PartialEq<&str> for SizeGroupEnumerationIri {
	fn eq(&self, other: &&str) -> bool {
		*other == SIZE_GROUP_ENUMERATION_IRI_HTTP || *other == SIZE_GROUP_ENUMERATION_IRI_HTTPS
	}
}
impl PartialEq<SizeGroupEnumerationIri> for &str {
	fn eq(&self, other: &SizeGroupEnumerationIri) -> bool {
		*self == SIZE_GROUP_ENUMERATION_IRI_HTTP || *self == SIZE_GROUP_ENUMERATION_IRI_HTTPS
	}
}
pub struct SizeGroupEnumerationIriOrLabel;
impl PartialEq<&str> for SizeGroupEnumerationIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == SizeGroupEnumerationIri || *other == SIZE_GROUP_ENUMERATION_LABEL
	}
}
impl PartialEq<SizeGroupEnumerationIriOrLabel> for &str {
	fn eq(&self, other: &SizeGroupEnumerationIriOrLabel) -> bool {
		*self == SizeGroupEnumerationIri || *self == SIZE_GROUP_ENUMERATION_LABEL
	}
}
