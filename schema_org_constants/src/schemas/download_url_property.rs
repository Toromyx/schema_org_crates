/// <https://schema.org/downloadUrl>
pub const DOWNLOAD_URL_PROPERTY_IRI_HTTP: &str = "http://schema.org/downloadUrl";
/// <https://schema.org/downloadUrl>
pub const DOWNLOAD_URL_PROPERTY_IRI_HTTPS: &str = "https://schema.org/downloadUrl";
/// <https://schema.org/downloadUrl>
pub const DOWNLOAD_URL_PROPERTY_LABEL: &str = "downloadUrl";
pub struct DownloadUrlPropertyIri;
impl PartialEq<&str> for DownloadUrlPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == DOWNLOAD_URL_PROPERTY_IRI_HTTP || *other == DOWNLOAD_URL_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<DownloadUrlPropertyIri> for &str {
	fn eq(&self, other: &DownloadUrlPropertyIri) -> bool {
		*self == DOWNLOAD_URL_PROPERTY_IRI_HTTP || *self == DOWNLOAD_URL_PROPERTY_IRI_HTTPS
	}
}
pub struct DownloadUrlPropertyIriOrLabel;
impl PartialEq<&str> for DownloadUrlPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == DownloadUrlPropertyIri || *other == DOWNLOAD_URL_PROPERTY_LABEL
	}
}
impl PartialEq<DownloadUrlPropertyIriOrLabel> for &str {
	fn eq(&self, other: &DownloadUrlPropertyIriOrLabel) -> bool {
		*self == DownloadUrlPropertyIri || *self == DOWNLOAD_URL_PROPERTY_LABEL
	}
}
