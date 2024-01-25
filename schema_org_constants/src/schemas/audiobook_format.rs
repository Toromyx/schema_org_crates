/// <https://schema.org/AudiobookFormat>
pub const AUDIOBOOK_FORMAT_IRI_HTTP: &str = "http://schema.org/AudiobookFormat";
/// <https://schema.org/AudiobookFormat>
pub const AUDIOBOOK_FORMAT_IRI_HTTPS: &str = "https://schema.org/AudiobookFormat";
/// <https://schema.org/AudiobookFormat>
pub const AUDIOBOOK_FORMAT_LABEL: &str = "AudiobookFormat";
pub struct AudiobookFormatIri;
impl PartialEq<&str> for AudiobookFormatIri {
	fn eq(&self, other: &&str) -> bool {
		*other == AUDIOBOOK_FORMAT_IRI_HTTP || *other == AUDIOBOOK_FORMAT_IRI_HTTPS
	}
}
impl PartialEq<AudiobookFormatIri> for &str {
	fn eq(&self, other: &AudiobookFormatIri) -> bool {
		*self == AUDIOBOOK_FORMAT_IRI_HTTP || *self == AUDIOBOOK_FORMAT_IRI_HTTPS
	}
}
pub struct AudiobookFormatIriOrLabel;
impl PartialEq<&str> for AudiobookFormatIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == AudiobookFormatIri || *other == AUDIOBOOK_FORMAT_LABEL
	}
}
impl PartialEq<AudiobookFormatIriOrLabel> for &str {
	fn eq(&self, other: &AudiobookFormatIriOrLabel) -> bool {
		*self == AudiobookFormatIri || *self == AUDIOBOOK_FORMAT_LABEL
	}
}
