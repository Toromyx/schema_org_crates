/// <https://schema.org/speakable>
pub const SPEAKABLE_PROPERTY_IRI_HTTP: &str = "http://schema.org/speakable";
/// <https://schema.org/speakable>
pub const SPEAKABLE_PROPERTY_IRI_HTTPS: &str = "https://schema.org/speakable";
/// <https://schema.org/speakable>
pub const SPEAKABLE_PROPERTY_LABEL: &str = "speakable";
pub struct SpeakablePropertyIri;
impl PartialEq<&str> for SpeakablePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == SPEAKABLE_PROPERTY_IRI_HTTP || *other == SPEAKABLE_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<SpeakablePropertyIri> for &str {
	fn eq(&self, other: &SpeakablePropertyIri) -> bool {
		*self == SPEAKABLE_PROPERTY_IRI_HTTP || *self == SPEAKABLE_PROPERTY_IRI_HTTPS
	}
}
pub struct SpeakablePropertyIriOrLabel;
impl PartialEq<&str> for SpeakablePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == SpeakablePropertyIri || *other == SPEAKABLE_PROPERTY_LABEL
	}
}
impl PartialEq<SpeakablePropertyIriOrLabel> for &str {
	fn eq(&self, other: &SpeakablePropertyIriOrLabel) -> bool {
		*self == SpeakablePropertyIri || *self == SPEAKABLE_PROPERTY_LABEL
	}
}
