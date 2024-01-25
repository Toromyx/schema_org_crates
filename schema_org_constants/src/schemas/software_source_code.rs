/// <https://schema.org/SoftwareSourceCode>
pub const SOFTWARE_SOURCE_CODE_IRI_HTTP: &str = "http://schema.org/SoftwareSourceCode";
/// <https://schema.org/SoftwareSourceCode>
pub const SOFTWARE_SOURCE_CODE_IRI_HTTPS: &str = "https://schema.org/SoftwareSourceCode";
/// <https://schema.org/SoftwareSourceCode>
pub const SOFTWARE_SOURCE_CODE_LABEL: &str = "SoftwareSourceCode";
pub struct SoftwareSourceCodeIri;
impl PartialEq<&str> for SoftwareSourceCodeIri {
	fn eq(&self, other: &&str) -> bool {
		*other == SOFTWARE_SOURCE_CODE_IRI_HTTP || *other == SOFTWARE_SOURCE_CODE_IRI_HTTPS
	}
}
impl PartialEq<SoftwareSourceCodeIri> for &str {
	fn eq(&self, other: &SoftwareSourceCodeIri) -> bool {
		*self == SOFTWARE_SOURCE_CODE_IRI_HTTP || *self == SOFTWARE_SOURCE_CODE_IRI_HTTPS
	}
}
pub struct SoftwareSourceCodeIriOrLabel;
impl PartialEq<&str> for SoftwareSourceCodeIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == SoftwareSourceCodeIri || *other == SOFTWARE_SOURCE_CODE_LABEL
	}
}
impl PartialEq<SoftwareSourceCodeIriOrLabel> for &str {
	fn eq(&self, other: &SoftwareSourceCodeIriOrLabel) -> bool {
		*self == SoftwareSourceCodeIri || *self == SOFTWARE_SOURCE_CODE_LABEL
	}
}
