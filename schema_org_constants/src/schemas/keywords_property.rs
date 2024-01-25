/// <https://schema.org/keywords>
pub const KEYWORDS_PROPERTY_IRI_HTTP: &str = "http://schema.org/keywords";
/// <https://schema.org/keywords>
pub const KEYWORDS_PROPERTY_IRI_HTTPS: &str = "https://schema.org/keywords";
/// <https://schema.org/keywords>
pub const KEYWORDS_PROPERTY_LABEL: &str = "keywords";
pub struct KeywordsPropertyIri;
impl PartialEq<&str> for KeywordsPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == KEYWORDS_PROPERTY_IRI_HTTP || *other == KEYWORDS_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<KeywordsPropertyIri> for &str {
	fn eq(&self, other: &KeywordsPropertyIri) -> bool {
		*self == KEYWORDS_PROPERTY_IRI_HTTP || *self == KEYWORDS_PROPERTY_IRI_HTTPS
	}
}
pub struct KeywordsPropertyIriOrLabel;
impl PartialEq<&str> for KeywordsPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == KeywordsPropertyIri || *other == KEYWORDS_PROPERTY_LABEL
	}
}
impl PartialEq<KeywordsPropertyIriOrLabel> for &str {
	fn eq(&self, other: &KeywordsPropertyIriOrLabel) -> bool {
		*self == KeywordsPropertyIri || *self == KEYWORDS_PROPERTY_LABEL
	}
}
