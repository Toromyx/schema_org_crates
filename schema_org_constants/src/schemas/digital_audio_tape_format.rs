/// <https://schema.org/DigitalAudioTapeFormat>
pub const DIGITAL_AUDIO_TAPE_FORMAT_IRI_HTTP: &str = "http://schema.org/DigitalAudioTapeFormat";
/// <https://schema.org/DigitalAudioTapeFormat>
pub const DIGITAL_AUDIO_TAPE_FORMAT_IRI_HTTPS: &str = "https://schema.org/DigitalAudioTapeFormat";
/// <https://schema.org/DigitalAudioTapeFormat>
pub const DIGITAL_AUDIO_TAPE_FORMAT_LABEL: &str = "DigitalAudioTapeFormat";
pub struct DigitalAudioTapeFormatIri;
impl PartialEq<&str> for DigitalAudioTapeFormatIri {
	fn eq(&self, other: &&str) -> bool {
		*other == DIGITAL_AUDIO_TAPE_FORMAT_IRI_HTTP
			|| *other == DIGITAL_AUDIO_TAPE_FORMAT_IRI_HTTPS
	}
}
impl PartialEq<DigitalAudioTapeFormatIri> for &str {
	fn eq(&self, other: &DigitalAudioTapeFormatIri) -> bool {
		*self == DIGITAL_AUDIO_TAPE_FORMAT_IRI_HTTP || *self == DIGITAL_AUDIO_TAPE_FORMAT_IRI_HTTPS
	}
}
pub struct DigitalAudioTapeFormatIriOrLabel;
impl PartialEq<&str> for DigitalAudioTapeFormatIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == DigitalAudioTapeFormatIri || *other == DIGITAL_AUDIO_TAPE_FORMAT_LABEL
	}
}
impl PartialEq<DigitalAudioTapeFormatIriOrLabel> for &str {
	fn eq(&self, other: &DigitalAudioTapeFormatIriOrLabel) -> bool {
		*self == DigitalAudioTapeFormatIri || *self == DIGITAL_AUDIO_TAPE_FORMAT_LABEL
	}
}
