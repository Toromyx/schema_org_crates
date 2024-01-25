/// <https://schema.org/Enumeration>
pub const ENUMERATION_IRI_HTTP: &str = "http://schema.org/Enumeration";
/// <https://schema.org/Enumeration>
pub const ENUMERATION_IRI_HTTPS: &str = "https://schema.org/Enumeration";
/// <https://schema.org/Enumeration>
pub const ENUMERATION_LABEL: &str = "Enumeration";
pub struct EnumerationIri;
impl PartialEq<&str> for EnumerationIri {
	fn eq(&self, other: &&str) -> bool {
		*other == ENUMERATION_IRI_HTTP || *other == ENUMERATION_IRI_HTTPS
	}
}
impl PartialEq<EnumerationIri> for &str {
	fn eq(&self, other: &EnumerationIri) -> bool {
		*self == ENUMERATION_IRI_HTTP || *self == ENUMERATION_IRI_HTTPS
	}
}
pub struct EnumerationIriOrLabel;
impl PartialEq<&str> for EnumerationIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == EnumerationIri || *other == ENUMERATION_LABEL
	}
}
impl PartialEq<EnumerationIriOrLabel> for &str {
	fn eq(&self, other: &EnumerationIriOrLabel) -> bool {
		*self == EnumerationIri || *self == ENUMERATION_LABEL
	}
}
