/// <https://schema.org/PronounceableText>
pub const PRONOUNCEABLE_TEXT_IRI_HTTP: &str = "http://schema.org/PronounceableText";
/// <https://schema.org/PronounceableText>
pub const PRONOUNCEABLE_TEXT_IRI_HTTPS: &str = "https://schema.org/PronounceableText";
/// <https://schema.org/PronounceableText>
pub const PRONOUNCEABLE_TEXT_LABEL: &str = "PronounceableText";
pub struct PronounceableTextIri;
impl PartialEq<&str> for PronounceableTextIri {
	fn eq(&self, other: &&str) -> bool {
		*other == PRONOUNCEABLE_TEXT_IRI_HTTP || *other == PRONOUNCEABLE_TEXT_IRI_HTTPS
	}
}
impl PartialEq<PronounceableTextIri> for &str {
	fn eq(&self, other: &PronounceableTextIri) -> bool {
		*self == PRONOUNCEABLE_TEXT_IRI_HTTP || *self == PRONOUNCEABLE_TEXT_IRI_HTTPS
	}
}
pub struct PronounceableTextIriOrLabel;
impl PartialEq<&str> for PronounceableTextIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == PronounceableTextIri || *other == PRONOUNCEABLE_TEXT_LABEL
	}
}
impl PartialEq<PronounceableTextIriOrLabel> for &str {
	fn eq(&self, other: &PronounceableTextIriOrLabel) -> bool {
		*self == PronounceableTextIri || *self == PRONOUNCEABLE_TEXT_LABEL
	}
}
