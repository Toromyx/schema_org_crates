/// <https://schema.org/audio>
pub const AUDIO_PROPERTY_IRI_HTTP: &str = "http://schema.org/audio";
/// <https://schema.org/audio>
pub const AUDIO_PROPERTY_IRI_HTTPS: &str = "https://schema.org/audio";
/// <https://schema.org/audio>
pub const AUDIO_PROPERTY_LABEL: &str = "audio";
pub struct AudioPropertyIri;
impl PartialEq<&str> for AudioPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == AUDIO_PROPERTY_IRI_HTTP || *other == AUDIO_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<AudioPropertyIri> for &str {
	fn eq(&self, other: &AudioPropertyIri) -> bool {
		*self == AUDIO_PROPERTY_IRI_HTTP || *self == AUDIO_PROPERTY_IRI_HTTPS
	}
}
pub struct AudioPropertyIriOrLabel;
impl PartialEq<&str> for AudioPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == AudioPropertyIri || *other == AUDIO_PROPERTY_LABEL
	}
}
impl PartialEq<AudioPropertyIriOrLabel> for &str {
	fn eq(&self, other: &AudioPropertyIriOrLabel) -> bool {
		*self == AudioPropertyIri || *self == AUDIO_PROPERTY_LABEL
	}
}
