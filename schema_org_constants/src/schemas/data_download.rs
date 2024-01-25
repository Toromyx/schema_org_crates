/// <https://schema.org/DataDownload>
pub const DATA_DOWNLOAD_IRI_HTTP: &str = "http://schema.org/DataDownload";
/// <https://schema.org/DataDownload>
pub const DATA_DOWNLOAD_IRI_HTTPS: &str = "https://schema.org/DataDownload";
/// <https://schema.org/DataDownload>
pub const DATA_DOWNLOAD_LABEL: &str = "DataDownload";
pub struct DataDownloadIri;
impl PartialEq<&str> for DataDownloadIri {
	fn eq(&self, other: &&str) -> bool {
		*other == DATA_DOWNLOAD_IRI_HTTP || *other == DATA_DOWNLOAD_IRI_HTTPS
	}
}
impl PartialEq<DataDownloadIri> for &str {
	fn eq(&self, other: &DataDownloadIri) -> bool {
		*self == DATA_DOWNLOAD_IRI_HTTP || *self == DATA_DOWNLOAD_IRI_HTTPS
	}
}
pub struct DataDownloadIriOrLabel;
impl PartialEq<&str> for DataDownloadIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == DataDownloadIri || *other == DATA_DOWNLOAD_LABEL
	}
}
impl PartialEq<DataDownloadIriOrLabel> for &str {
	fn eq(&self, other: &DataDownloadIriOrLabel) -> bool {
		*self == DataDownloadIri || *self == DATA_DOWNLOAD_LABEL
	}
}
