/// <https://schema.org/SpreadsheetDigitalDocument>
pub const SPREADSHEET_DIGITAL_DOCUMENT_IRI_HTTP: &str =
	"http://schema.org/SpreadsheetDigitalDocument";
/// <https://schema.org/SpreadsheetDigitalDocument>
pub const SPREADSHEET_DIGITAL_DOCUMENT_IRI_HTTPS: &str =
	"https://schema.org/SpreadsheetDigitalDocument";
/// <https://schema.org/SpreadsheetDigitalDocument>
pub const SPREADSHEET_DIGITAL_DOCUMENT_LABEL: &str = "SpreadsheetDigitalDocument";
pub struct SpreadsheetDigitalDocumentIri;
impl PartialEq<&str> for SpreadsheetDigitalDocumentIri {
	fn eq(&self, other: &&str) -> bool {
		*other == SPREADSHEET_DIGITAL_DOCUMENT_IRI_HTTP
			|| *other == SPREADSHEET_DIGITAL_DOCUMENT_IRI_HTTPS
	}
}
impl PartialEq<SpreadsheetDigitalDocumentIri> for &str {
	fn eq(&self, other: &SpreadsheetDigitalDocumentIri) -> bool {
		*self == SPREADSHEET_DIGITAL_DOCUMENT_IRI_HTTP
			|| *self == SPREADSHEET_DIGITAL_DOCUMENT_IRI_HTTPS
	}
}
pub struct SpreadsheetDigitalDocumentIriOrLabel;
impl PartialEq<&str> for SpreadsheetDigitalDocumentIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == SpreadsheetDigitalDocumentIri || *other == SPREADSHEET_DIGITAL_DOCUMENT_LABEL
	}
}
impl PartialEq<SpreadsheetDigitalDocumentIriOrLabel> for &str {
	fn eq(&self, other: &SpreadsheetDigitalDocumentIriOrLabel) -> bool {
		*self == SpreadsheetDigitalDocumentIri || *self == SPREADSHEET_DIGITAL_DOCUMENT_LABEL
	}
}
