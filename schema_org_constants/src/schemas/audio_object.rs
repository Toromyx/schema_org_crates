/// <https://schema.org/AudioObject>
pub const AUDIO_OBJECT_IRI_HTTP: &str = "http://schema.org/AudioObject";
/// <https://schema.org/AudioObject>
pub const AUDIO_OBJECT_IRI_HTTPS: &str = "https://schema.org/AudioObject";
/// <https://schema.org/AudioObject>
pub const AUDIO_OBJECT_LABEL: &str = "AudioObject";
pub struct AudioObjectIri;
impl PartialEq<&str> for AudioObjectIri {
	fn eq(&self, other: &&str) -> bool {
		*other == AUDIO_OBJECT_IRI_HTTP || *other == AUDIO_OBJECT_IRI_HTTPS
	}
}
impl PartialEq<AudioObjectIri> for &str {
	fn eq(&self, other: &AudioObjectIri) -> bool {
		*self == AUDIO_OBJECT_IRI_HTTP || *self == AUDIO_OBJECT_IRI_HTTPS
	}
}
pub struct AudioObjectIriOrLabel;
impl PartialEq<&str> for AudioObjectIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == AudioObjectIri || *other == AUDIO_OBJECT_LABEL
	}
}
impl PartialEq<AudioObjectIriOrLabel> for &str {
	fn eq(&self, other: &AudioObjectIriOrLabel) -> bool {
		*self == AudioObjectIri || *self == AUDIO_OBJECT_LABEL
	}
}
