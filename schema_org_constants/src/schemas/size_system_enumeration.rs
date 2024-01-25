/// <https://schema.org/SizeSystemEnumeration>
pub const SIZE_SYSTEM_ENUMERATION_IRI_HTTP: &str = "http://schema.org/SizeSystemEnumeration";
/// <https://schema.org/SizeSystemEnumeration>
pub const SIZE_SYSTEM_ENUMERATION_IRI_HTTPS: &str = "https://schema.org/SizeSystemEnumeration";
/// <https://schema.org/SizeSystemEnumeration>
pub const SIZE_SYSTEM_ENUMERATION_LABEL: &str = "SizeSystemEnumeration";
pub struct SizeSystemEnumerationIri;
impl PartialEq<&str> for SizeSystemEnumerationIri {
	fn eq(&self, other: &&str) -> bool {
		*other == SIZE_SYSTEM_ENUMERATION_IRI_HTTP || *other == SIZE_SYSTEM_ENUMERATION_IRI_HTTPS
	}
}
impl PartialEq<SizeSystemEnumerationIri> for &str {
	fn eq(&self, other: &SizeSystemEnumerationIri) -> bool {
		*self == SIZE_SYSTEM_ENUMERATION_IRI_HTTP || *self == SIZE_SYSTEM_ENUMERATION_IRI_HTTPS
	}
}
pub struct SizeSystemEnumerationIriOrLabel;
impl PartialEq<&str> for SizeSystemEnumerationIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == SizeSystemEnumerationIri || *other == SIZE_SYSTEM_ENUMERATION_LABEL
	}
}
impl PartialEq<SizeSystemEnumerationIriOrLabel> for &str {
	fn eq(&self, other: &SizeSystemEnumerationIriOrLabel) -> bool {
		*self == SizeSystemEnumerationIri || *self == SIZE_SYSTEM_ENUMERATION_LABEL
	}
}
