/// <https://schema.org/codeValue>
pub const CODE_VALUE_PROPERTY_IRI_HTTP: &str = "http://schema.org/codeValue";
/// <https://schema.org/codeValue>
pub const CODE_VALUE_PROPERTY_IRI_HTTPS: &str = "https://schema.org/codeValue";
/// <https://schema.org/codeValue>
pub const CODE_VALUE_PROPERTY_LABEL: &str = "codeValue";
pub struct CodeValuePropertyIri;
impl PartialEq<&str> for CodeValuePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == CODE_VALUE_PROPERTY_IRI_HTTP || *other == CODE_VALUE_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<CodeValuePropertyIri> for &str {
	fn eq(&self, other: &CodeValuePropertyIri) -> bool {
		*self == CODE_VALUE_PROPERTY_IRI_HTTP || *self == CODE_VALUE_PROPERTY_IRI_HTTPS
	}
}
pub struct CodeValuePropertyIriOrLabel;
impl PartialEq<&str> for CodeValuePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == CodeValuePropertyIri || *other == CODE_VALUE_PROPERTY_LABEL
	}
}
impl PartialEq<CodeValuePropertyIriOrLabel> for &str {
	fn eq(&self, other: &CodeValuePropertyIriOrLabel) -> bool {
		*self == CodeValuePropertyIri || *self == CODE_VALUE_PROPERTY_LABEL
	}
}
