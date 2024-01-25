/// <https://schema.org/Code>
#[deprecated = "This schema is superseded by <https://schema.org/SoftwareSourceCode>."]
pub const CODE_IRI_HTTP: &str = "http://schema.org/Code";
/// <https://schema.org/Code>
#[deprecated = "This schema is superseded by <https://schema.org/SoftwareSourceCode>."]
pub const CODE_IRI_HTTPS: &str = "https://schema.org/Code";
/// <https://schema.org/Code>
#[deprecated = "This schema is superseded by <https://schema.org/SoftwareSourceCode>."]
pub const CODE_LABEL: &str = "Code";
pub struct CodeIri;
impl PartialEq<&str> for CodeIri {
	fn eq(&self, other: &&str) -> bool {
		*other == CODE_IRI_HTTP || *other == CODE_IRI_HTTPS
	}
}
impl PartialEq<CodeIri> for &str {
	fn eq(&self, other: &CodeIri) -> bool {
		*self == CODE_IRI_HTTP || *self == CODE_IRI_HTTPS
	}
}
pub struct CodeIriOrLabel;
impl PartialEq<&str> for CodeIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == CodeIri || *other == CODE_LABEL
	}
}
impl PartialEq<CodeIriOrLabel> for &str {
	fn eq(&self, other: &CodeIriOrLabel) -> bool {
		*self == CodeIri || *self == CODE_LABEL
	}
}
