/// <https://schema.org/PresentationDigitalDocument>
pub const PRESENTATION_DIGITAL_DOCUMENT_IRI_HTTP: &str =
	"http://schema.org/PresentationDigitalDocument";
/// <https://schema.org/PresentationDigitalDocument>
pub const PRESENTATION_DIGITAL_DOCUMENT_IRI_HTTPS: &str =
	"https://schema.org/PresentationDigitalDocument";
/// <https://schema.org/PresentationDigitalDocument>
pub const PRESENTATION_DIGITAL_DOCUMENT_LABEL: &str = "PresentationDigitalDocument";
pub struct PresentationDigitalDocumentIri;
impl PartialEq<&str> for PresentationDigitalDocumentIri {
	fn eq(&self, other: &&str) -> bool {
		*other == PRESENTATION_DIGITAL_DOCUMENT_IRI_HTTP
			|| *other == PRESENTATION_DIGITAL_DOCUMENT_IRI_HTTPS
	}
}
impl PartialEq<PresentationDigitalDocumentIri> for &str {
	fn eq(&self, other: &PresentationDigitalDocumentIri) -> bool {
		*self == PRESENTATION_DIGITAL_DOCUMENT_IRI_HTTP
			|| *self == PRESENTATION_DIGITAL_DOCUMENT_IRI_HTTPS
	}
}
pub struct PresentationDigitalDocumentIriOrLabel;
impl PartialEq<&str> for PresentationDigitalDocumentIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == PresentationDigitalDocumentIri || *other == PRESENTATION_DIGITAL_DOCUMENT_LABEL
	}
}
impl PartialEq<PresentationDigitalDocumentIriOrLabel> for &str {
	fn eq(&self, other: &PresentationDigitalDocumentIriOrLabel) -> bool {
		*self == PresentationDigitalDocumentIri || *self == PRESENTATION_DIGITAL_DOCUMENT_LABEL
	}
}
