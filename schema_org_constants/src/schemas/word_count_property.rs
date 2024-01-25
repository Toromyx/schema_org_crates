/// <https://schema.org/wordCount>
pub const WORD_COUNT_PROPERTY_IRI_HTTP: &str = "http://schema.org/wordCount";
/// <https://schema.org/wordCount>
pub const WORD_COUNT_PROPERTY_IRI_HTTPS: &str = "https://schema.org/wordCount";
/// <https://schema.org/wordCount>
pub const WORD_COUNT_PROPERTY_LABEL: &str = "wordCount";
pub struct WordCountPropertyIri;
impl PartialEq<&str> for WordCountPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == WORD_COUNT_PROPERTY_IRI_HTTP || *other == WORD_COUNT_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<WordCountPropertyIri> for &str {
	fn eq(&self, other: &WordCountPropertyIri) -> bool {
		*self == WORD_COUNT_PROPERTY_IRI_HTTP || *self == WORD_COUNT_PROPERTY_IRI_HTTPS
	}
}
pub struct WordCountPropertyIriOrLabel;
impl PartialEq<&str> for WordCountPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == WordCountPropertyIri || *other == WORD_COUNT_PROPERTY_LABEL
	}
}
impl PartialEq<WordCountPropertyIriOrLabel> for &str {
	fn eq(&self, other: &WordCountPropertyIriOrLabel) -> bool {
		*self == WordCountPropertyIri || *self == WORD_COUNT_PROPERTY_LABEL
	}
}
