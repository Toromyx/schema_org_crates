/// <https://schema.org/fileFormat>
#[deprecated = "This schema is superseded by <https://schema.org/encodingFormat>."]
pub const FILE_FORMAT_PROPERTY_IRI_HTTP: &str = "http://schema.org/fileFormat";
/// <https://schema.org/fileFormat>
#[deprecated = "This schema is superseded by <https://schema.org/encodingFormat>."]
pub const FILE_FORMAT_PROPERTY_IRI_HTTPS: &str = "https://schema.org/fileFormat";
/// <https://schema.org/fileFormat>
#[deprecated = "This schema is superseded by <https://schema.org/encodingFormat>."]
pub const FILE_FORMAT_PROPERTY_LABEL: &str = "fileFormat";
pub struct FileFormatPropertyIri;
impl PartialEq<&str> for FileFormatPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == FILE_FORMAT_PROPERTY_IRI_HTTP || *other == FILE_FORMAT_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<FileFormatPropertyIri> for &str {
	fn eq(&self, other: &FileFormatPropertyIri) -> bool {
		*self == FILE_FORMAT_PROPERTY_IRI_HTTP || *self == FILE_FORMAT_PROPERTY_IRI_HTTPS
	}
}
pub struct FileFormatPropertyIriOrLabel;
impl PartialEq<&str> for FileFormatPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == FileFormatPropertyIri || *other == FILE_FORMAT_PROPERTY_LABEL
	}
}
impl PartialEq<FileFormatPropertyIriOrLabel> for &str {
	fn eq(&self, other: &FileFormatPropertyIriOrLabel) -> bool {
		*self == FileFormatPropertyIri || *self == FILE_FORMAT_PROPERTY_LABEL
	}
}
