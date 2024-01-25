/// <https://schema.org/recognizedBy>
pub const RECOGNIZED_BY_PROPERTY_IRI_HTTP: &str = "http://schema.org/recognizedBy";
/// <https://schema.org/recognizedBy>
pub const RECOGNIZED_BY_PROPERTY_IRI_HTTPS: &str = "https://schema.org/recognizedBy";
/// <https://schema.org/recognizedBy>
pub const RECOGNIZED_BY_PROPERTY_LABEL: &str = "recognizedBy";
pub struct RecognizedByPropertyIri;
impl PartialEq<&str> for RecognizedByPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == RECOGNIZED_BY_PROPERTY_IRI_HTTP || *other == RECOGNIZED_BY_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<RecognizedByPropertyIri> for &str {
	fn eq(&self, other: &RecognizedByPropertyIri) -> bool {
		*self == RECOGNIZED_BY_PROPERTY_IRI_HTTP || *self == RECOGNIZED_BY_PROPERTY_IRI_HTTPS
	}
}
pub struct RecognizedByPropertyIriOrLabel;
impl PartialEq<&str> for RecognizedByPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == RecognizedByPropertyIri || *other == RECOGNIZED_BY_PROPERTY_LABEL
	}
}
impl PartialEq<RecognizedByPropertyIriOrLabel> for &str {
	fn eq(&self, other: &RecognizedByPropertyIriOrLabel) -> bool {
		*self == RecognizedByPropertyIri || *self == RECOGNIZED_BY_PROPERTY_LABEL
	}
}
