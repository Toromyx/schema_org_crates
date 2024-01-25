/// <https://schema.org/code>
pub const CODE_PROPERTY_IRI_HTTP: &str = "http://schema.org/code";
/// <https://schema.org/code>
pub const CODE_PROPERTY_IRI_HTTPS: &str = "https://schema.org/code";
/// <https://schema.org/code>
pub const CODE_PROPERTY_LABEL: &str = "code";
pub struct CodePropertyIri;
impl PartialEq<&str> for CodePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == CODE_PROPERTY_IRI_HTTP || *other == CODE_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<CodePropertyIri> for &str {
	fn eq(&self, other: &CodePropertyIri) -> bool {
		*self == CODE_PROPERTY_IRI_HTTP || *self == CODE_PROPERTY_IRI_HTTPS
	}
}
pub struct CodePropertyIriOrLabel;
impl PartialEq<&str> for CodePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == CodePropertyIri || *other == CODE_PROPERTY_LABEL
	}
}
impl PartialEq<CodePropertyIriOrLabel> for &str {
	fn eq(&self, other: &CodePropertyIriOrLabel) -> bool {
		*self == CodePropertyIri || *self == CODE_PROPERTY_LABEL
	}
}
