/// <https://schema.org/NoteDigitalDocument>
pub const NOTE_DIGITAL_DOCUMENT_IRI_HTTP: &str = "http://schema.org/NoteDigitalDocument";
/// <https://schema.org/NoteDigitalDocument>
pub const NOTE_DIGITAL_DOCUMENT_IRI_HTTPS: &str = "https://schema.org/NoteDigitalDocument";
/// <https://schema.org/NoteDigitalDocument>
pub const NOTE_DIGITAL_DOCUMENT_LABEL: &str = "NoteDigitalDocument";
pub struct NoteDigitalDocumentIri;
impl PartialEq<&str> for NoteDigitalDocumentIri {
	fn eq(&self, other: &&str) -> bool {
		*other == NOTE_DIGITAL_DOCUMENT_IRI_HTTP || *other == NOTE_DIGITAL_DOCUMENT_IRI_HTTPS
	}
}
impl PartialEq<NoteDigitalDocumentIri> for &str {
	fn eq(&self, other: &NoteDigitalDocumentIri) -> bool {
		*self == NOTE_DIGITAL_DOCUMENT_IRI_HTTP || *self == NOTE_DIGITAL_DOCUMENT_IRI_HTTPS
	}
}
pub struct NoteDigitalDocumentIriOrLabel;
impl PartialEq<&str> for NoteDigitalDocumentIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == NoteDigitalDocumentIri || *other == NOTE_DIGITAL_DOCUMENT_LABEL
	}
}
impl PartialEq<NoteDigitalDocumentIriOrLabel> for &str {
	fn eq(&self, other: &NoteDigitalDocumentIriOrLabel) -> bool {
		*self == NoteDigitalDocumentIri || *self == NOTE_DIGITAL_DOCUMENT_LABEL
	}
}
