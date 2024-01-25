/// <https://schema.org/codeRepository>
pub const CODE_REPOSITORY_PROPERTY_IRI_HTTP: &str = "http://schema.org/codeRepository";
/// <https://schema.org/codeRepository>
pub const CODE_REPOSITORY_PROPERTY_IRI_HTTPS: &str = "https://schema.org/codeRepository";
/// <https://schema.org/codeRepository>
pub const CODE_REPOSITORY_PROPERTY_LABEL: &str = "codeRepository";
pub struct CodeRepositoryPropertyIri;
impl PartialEq<&str> for CodeRepositoryPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == CODE_REPOSITORY_PROPERTY_IRI_HTTP || *other == CODE_REPOSITORY_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<CodeRepositoryPropertyIri> for &str {
	fn eq(&self, other: &CodeRepositoryPropertyIri) -> bool {
		*self == CODE_REPOSITORY_PROPERTY_IRI_HTTP || *self == CODE_REPOSITORY_PROPERTY_IRI_HTTPS
	}
}
pub struct CodeRepositoryPropertyIriOrLabel;
impl PartialEq<&str> for CodeRepositoryPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == CodeRepositoryPropertyIri || *other == CODE_REPOSITORY_PROPERTY_LABEL
	}
}
impl PartialEq<CodeRepositoryPropertyIriOrLabel> for &str {
	fn eq(&self, other: &CodeRepositoryPropertyIriOrLabel) -> bool {
		*self == CodeRepositoryPropertyIri || *self == CODE_REPOSITORY_PROPERTY_LABEL
	}
}
