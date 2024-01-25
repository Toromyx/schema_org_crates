/// <https://schema.org/fileSize>
pub const FILE_SIZE_PROPERTY_IRI_HTTP: &str = "http://schema.org/fileSize";
/// <https://schema.org/fileSize>
pub const FILE_SIZE_PROPERTY_IRI_HTTPS: &str = "https://schema.org/fileSize";
/// <https://schema.org/fileSize>
pub const FILE_SIZE_PROPERTY_LABEL: &str = "fileSize";
pub struct FileSizePropertyIri;
impl PartialEq<&str> for FileSizePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == FILE_SIZE_PROPERTY_IRI_HTTP || *other == FILE_SIZE_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<FileSizePropertyIri> for &str {
	fn eq(&self, other: &FileSizePropertyIri) -> bool {
		*self == FILE_SIZE_PROPERTY_IRI_HTTP || *self == FILE_SIZE_PROPERTY_IRI_HTTPS
	}
}
pub struct FileSizePropertyIriOrLabel;
impl PartialEq<&str> for FileSizePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == FileSizePropertyIri || *other == FILE_SIZE_PROPERTY_LABEL
	}
}
impl PartialEq<FileSizePropertyIriOrLabel> for &str {
	fn eq(&self, other: &FileSizePropertyIriOrLabel) -> bool {
		*self == FileSizePropertyIri || *self == FILE_SIZE_PROPERTY_LABEL
	}
}
