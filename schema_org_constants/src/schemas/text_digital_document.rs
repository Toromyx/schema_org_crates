/// <https://schema.org/TextDigitalDocument>
pub const TEXT_DIGITAL_DOCUMENT_IRI_HTTP: &str = "http://schema.org/TextDigitalDocument";
/// <https://schema.org/TextDigitalDocument>
pub const TEXT_DIGITAL_DOCUMENT_IRI_HTTPS: &str = "https://schema.org/TextDigitalDocument";
/// <https://schema.org/TextDigitalDocument>
pub const TEXT_DIGITAL_DOCUMENT_LABEL: &str = "TextDigitalDocument";
pub struct TextDigitalDocumentIri;
impl PartialEq<&str> for TextDigitalDocumentIri {
	fn eq(&self, other: &&str) -> bool {
		*other == TEXT_DIGITAL_DOCUMENT_IRI_HTTP || *other == TEXT_DIGITAL_DOCUMENT_IRI_HTTPS
	}
}
impl PartialEq<TextDigitalDocumentIri> for &str {
	fn eq(&self, other: &TextDigitalDocumentIri) -> bool {
		*self == TEXT_DIGITAL_DOCUMENT_IRI_HTTP || *self == TEXT_DIGITAL_DOCUMENT_IRI_HTTPS
	}
}
pub struct TextDigitalDocumentIriOrLabel;
impl PartialEq<&str> for TextDigitalDocumentIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == TextDigitalDocumentIri || *other == TEXT_DIGITAL_DOCUMENT_LABEL
	}
}
impl PartialEq<TextDigitalDocumentIriOrLabel> for &str {
	fn eq(&self, other: &TextDigitalDocumentIriOrLabel) -> bool {
		*self == TextDigitalDocumentIri || *self == TEXT_DIGITAL_DOCUMENT_LABEL
	}
}
