/// <https://schema.org/DownloadAction>
pub const DOWNLOAD_ACTION_IRI_HTTP: &str = "http://schema.org/DownloadAction";
/// <https://schema.org/DownloadAction>
pub const DOWNLOAD_ACTION_IRI_HTTPS: &str = "https://schema.org/DownloadAction";
/// <https://schema.org/DownloadAction>
pub const DOWNLOAD_ACTION_LABEL: &str = "DownloadAction";
pub struct DownloadActionIri;
impl PartialEq<&str> for DownloadActionIri {
	fn eq(&self, other: &&str) -> bool {
		*other == DOWNLOAD_ACTION_IRI_HTTP || *other == DOWNLOAD_ACTION_IRI_HTTPS
	}
}
impl PartialEq<DownloadActionIri> for &str {
	fn eq(&self, other: &DownloadActionIri) -> bool {
		*self == DOWNLOAD_ACTION_IRI_HTTP || *self == DOWNLOAD_ACTION_IRI_HTTPS
	}
}
pub struct DownloadActionIriOrLabel;
impl PartialEq<&str> for DownloadActionIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == DownloadActionIri || *other == DOWNLOAD_ACTION_LABEL
	}
}
impl PartialEq<DownloadActionIriOrLabel> for &str {
	fn eq(&self, other: &DownloadActionIriOrLabel) -> bool {
		*self == DownloadActionIri || *self == DOWNLOAD_ACTION_LABEL
	}
}
