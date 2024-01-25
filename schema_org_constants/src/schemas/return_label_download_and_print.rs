/// <https://schema.org/ReturnLabelDownloadAndPrint>
pub const RETURN_LABEL_DOWNLOAD_AND_PRINT_IRI_HTTP: &str =
	"http://schema.org/ReturnLabelDownloadAndPrint";
/// <https://schema.org/ReturnLabelDownloadAndPrint>
pub const RETURN_LABEL_DOWNLOAD_AND_PRINT_IRI_HTTPS: &str =
	"https://schema.org/ReturnLabelDownloadAndPrint";
/// <https://schema.org/ReturnLabelDownloadAndPrint>
pub const RETURN_LABEL_DOWNLOAD_AND_PRINT_LABEL: &str = "ReturnLabelDownloadAndPrint";
pub struct ReturnLabelDownloadAndPrintIri;
impl PartialEq<&str> for ReturnLabelDownloadAndPrintIri {
	fn eq(&self, other: &&str) -> bool {
		*other == RETURN_LABEL_DOWNLOAD_AND_PRINT_IRI_HTTP
			|| *other == RETURN_LABEL_DOWNLOAD_AND_PRINT_IRI_HTTPS
	}
}
impl PartialEq<ReturnLabelDownloadAndPrintIri> for &str {
	fn eq(&self, other: &ReturnLabelDownloadAndPrintIri) -> bool {
		*self == RETURN_LABEL_DOWNLOAD_AND_PRINT_IRI_HTTP
			|| *self == RETURN_LABEL_DOWNLOAD_AND_PRINT_IRI_HTTPS
	}
}
pub struct ReturnLabelDownloadAndPrintIriOrLabel;
impl PartialEq<&str> for ReturnLabelDownloadAndPrintIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ReturnLabelDownloadAndPrintIri || *other == RETURN_LABEL_DOWNLOAD_AND_PRINT_LABEL
	}
}
impl PartialEq<ReturnLabelDownloadAndPrintIriOrLabel> for &str {
	fn eq(&self, other: &ReturnLabelDownloadAndPrintIriOrLabel) -> bool {
		*self == ReturnLabelDownloadAndPrintIri || *self == RETURN_LABEL_DOWNLOAD_AND_PRINT_LABEL
	}
}
