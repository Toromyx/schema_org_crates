/// <https://schema.org/speechToTextMarkup>
pub const SPEECH_TO_TEXT_MARKUP_PROPERTY_IRI_HTTP: &str = "http://schema.org/speechToTextMarkup";
/// <https://schema.org/speechToTextMarkup>
pub const SPEECH_TO_TEXT_MARKUP_PROPERTY_IRI_HTTPS: &str = "https://schema.org/speechToTextMarkup";
/// <https://schema.org/speechToTextMarkup>
pub const SPEECH_TO_TEXT_MARKUP_PROPERTY_LABEL: &str = "speechToTextMarkup";
pub struct SpeechToTextMarkupPropertyIri;
impl PartialEq<&str> for SpeechToTextMarkupPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == SPEECH_TO_TEXT_MARKUP_PROPERTY_IRI_HTTP
			|| *other == SPEECH_TO_TEXT_MARKUP_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<SpeechToTextMarkupPropertyIri> for &str {
	fn eq(&self, other: &SpeechToTextMarkupPropertyIri) -> bool {
		*self == SPEECH_TO_TEXT_MARKUP_PROPERTY_IRI_HTTP
			|| *self == SPEECH_TO_TEXT_MARKUP_PROPERTY_IRI_HTTPS
	}
}
pub struct SpeechToTextMarkupPropertyIriOrLabel;
impl PartialEq<&str> for SpeechToTextMarkupPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == SpeechToTextMarkupPropertyIri || *other == SPEECH_TO_TEXT_MARKUP_PROPERTY_LABEL
	}
}
impl PartialEq<SpeechToTextMarkupPropertyIriOrLabel> for &str {
	fn eq(&self, other: &SpeechToTextMarkupPropertyIriOrLabel) -> bool {
		*self == SpeechToTextMarkupPropertyIri || *self == SPEECH_TO_TEXT_MARKUP_PROPERTY_LABEL
	}
}
