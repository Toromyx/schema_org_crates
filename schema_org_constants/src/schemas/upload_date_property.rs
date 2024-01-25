/// <https://schema.org/uploadDate>
pub const UPLOAD_DATE_PROPERTY_IRI_HTTP: &str = "http://schema.org/uploadDate";
/// <https://schema.org/uploadDate>
pub const UPLOAD_DATE_PROPERTY_IRI_HTTPS: &str = "https://schema.org/uploadDate";
/// <https://schema.org/uploadDate>
pub const UPLOAD_DATE_PROPERTY_LABEL: &str = "uploadDate";
pub struct UploadDatePropertyIri;
impl PartialEq<&str> for UploadDatePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == UPLOAD_DATE_PROPERTY_IRI_HTTP || *other == UPLOAD_DATE_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<UploadDatePropertyIri> for &str {
	fn eq(&self, other: &UploadDatePropertyIri) -> bool {
		*self == UPLOAD_DATE_PROPERTY_IRI_HTTP || *self == UPLOAD_DATE_PROPERTY_IRI_HTTPS
	}
}
pub struct UploadDatePropertyIriOrLabel;
impl PartialEq<&str> for UploadDatePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == UploadDatePropertyIri || *other == UPLOAD_DATE_PROPERTY_LABEL
	}
}
impl PartialEq<UploadDatePropertyIriOrLabel> for &str {
	fn eq(&self, other: &UploadDatePropertyIriOrLabel) -> bool {
		*self == UploadDatePropertyIri || *self == UPLOAD_DATE_PROPERTY_LABEL
	}
}
