/// <https://schema.org/DigitalDocument>
pub const DIGITAL_DOCUMENT_IRI_HTTP: &str = "http://schema.org/DigitalDocument";
/// <https://schema.org/DigitalDocument>
pub const DIGITAL_DOCUMENT_IRI_HTTPS: &str = "https://schema.org/DigitalDocument";
/// <https://schema.org/DigitalDocument>
pub const DIGITAL_DOCUMENT_LABEL: &str = "DigitalDocument";
pub struct DigitalDocumentIri;
impl PartialEq<&str> for DigitalDocumentIri {
	fn eq(&self, other: &&str) -> bool {
		*other == DIGITAL_DOCUMENT_IRI_HTTP || *other == DIGITAL_DOCUMENT_IRI_HTTPS
	}
}
impl PartialEq<DigitalDocumentIri> for &str {
	fn eq(&self, other: &DigitalDocumentIri) -> bool {
		*self == DIGITAL_DOCUMENT_IRI_HTTP || *self == DIGITAL_DOCUMENT_IRI_HTTPS
	}
}
pub struct DigitalDocumentIriOrLabel;
impl PartialEq<&str> for DigitalDocumentIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == DigitalDocumentIri || *other == DIGITAL_DOCUMENT_LABEL
	}
}
impl PartialEq<DigitalDocumentIriOrLabel> for &str {
	fn eq(&self, other: &DigitalDocumentIriOrLabel) -> bool {
		*self == DigitalDocumentIri || *self == DIGITAL_DOCUMENT_LABEL
	}
}
