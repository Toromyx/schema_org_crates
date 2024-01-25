/// <https://schema.org/SpeechPathology>
pub const SPEECH_PATHOLOGY_IRI_HTTP: &str = "http://schema.org/SpeechPathology";
/// <https://schema.org/SpeechPathology>
pub const SPEECH_PATHOLOGY_IRI_HTTPS: &str = "https://schema.org/SpeechPathology";
/// <https://schema.org/SpeechPathology>
pub const SPEECH_PATHOLOGY_LABEL: &str = "SpeechPathology";
pub struct SpeechPathologyIri;
impl PartialEq<&str> for SpeechPathologyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == SPEECH_PATHOLOGY_IRI_HTTP || *other == SPEECH_PATHOLOGY_IRI_HTTPS
	}
}
impl PartialEq<SpeechPathologyIri> for &str {
	fn eq(&self, other: &SpeechPathologyIri) -> bool {
		*self == SPEECH_PATHOLOGY_IRI_HTTP || *self == SPEECH_PATHOLOGY_IRI_HTTPS
	}
}
pub struct SpeechPathologyIriOrLabel;
impl PartialEq<&str> for SpeechPathologyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == SpeechPathologyIri || *other == SPEECH_PATHOLOGY_LABEL
	}
}
impl PartialEq<SpeechPathologyIriOrLabel> for &str {
	fn eq(&self, other: &SpeechPathologyIriOrLabel) -> bool {
		*self == SpeechPathologyIri || *self == SPEECH_PATHOLOGY_LABEL
	}
}
