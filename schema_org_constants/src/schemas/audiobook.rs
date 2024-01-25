/// <https://schema.org/Audiobook>
pub const AUDIOBOOK_IRI_HTTP: &str = "http://schema.org/Audiobook";
/// <https://schema.org/Audiobook>
pub const AUDIOBOOK_IRI_HTTPS: &str = "https://schema.org/Audiobook";
/// <https://schema.org/Audiobook>
pub const AUDIOBOOK_LABEL: &str = "Audiobook";
pub struct AudiobookIri;
impl PartialEq<&str> for AudiobookIri {
	fn eq(&self, other: &&str) -> bool {
		*other == AUDIOBOOK_IRI_HTTP || *other == AUDIOBOOK_IRI_HTTPS
	}
}
impl PartialEq<AudiobookIri> for &str {
	fn eq(&self, other: &AudiobookIri) -> bool {
		*self == AUDIOBOOK_IRI_HTTP || *self == AUDIOBOOK_IRI_HTTPS
	}
}
pub struct AudiobookIriOrLabel;
impl PartialEq<&str> for AudiobookIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == AudiobookIri || *other == AUDIOBOOK_LABEL
	}
}
impl PartialEq<AudiobookIriOrLabel> for &str {
	fn eq(&self, other: &AudiobookIriOrLabel) -> bool {
		*self == AudiobookIri || *self == AUDIOBOOK_LABEL
	}
}
