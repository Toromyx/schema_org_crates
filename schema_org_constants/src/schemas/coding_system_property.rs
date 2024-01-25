/// <https://schema.org/codingSystem>
pub const CODING_SYSTEM_PROPERTY_IRI_HTTP: &str = "http://schema.org/codingSystem";
/// <https://schema.org/codingSystem>
pub const CODING_SYSTEM_PROPERTY_IRI_HTTPS: &str = "https://schema.org/codingSystem";
/// <https://schema.org/codingSystem>
pub const CODING_SYSTEM_PROPERTY_LABEL: &str = "codingSystem";
pub struct CodingSystemPropertyIri;
impl PartialEq<&str> for CodingSystemPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == CODING_SYSTEM_PROPERTY_IRI_HTTP || *other == CODING_SYSTEM_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<CodingSystemPropertyIri> for &str {
	fn eq(&self, other: &CodingSystemPropertyIri) -> bool {
		*self == CODING_SYSTEM_PROPERTY_IRI_HTTP || *self == CODING_SYSTEM_PROPERTY_IRI_HTTPS
	}
}
pub struct CodingSystemPropertyIriOrLabel;
impl PartialEq<&str> for CodingSystemPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == CodingSystemPropertyIri || *other == CODING_SYSTEM_PROPERTY_LABEL
	}
}
impl PartialEq<CodingSystemPropertyIriOrLabel> for &str {
	fn eq(&self, other: &CodingSystemPropertyIriOrLabel) -> bool {
		*self == CodingSystemPropertyIri || *self == CODING_SYSTEM_PROPERTY_LABEL
	}
}
