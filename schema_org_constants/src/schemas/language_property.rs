/// <https://schema.org/language>
#[deprecated = "This schema is superseded by <https://schema.org/inLanguage>."]
pub const LANGUAGE_PROPERTY_IRI_HTTP: &str = "http://schema.org/language";
/// <https://schema.org/language>
#[deprecated = "This schema is superseded by <https://schema.org/inLanguage>."]
pub const LANGUAGE_PROPERTY_IRI_HTTPS: &str = "https://schema.org/language";
/// <https://schema.org/language>
#[deprecated = "This schema is superseded by <https://schema.org/inLanguage>."]
pub const LANGUAGE_PROPERTY_LABEL: &str = "language";
pub struct LanguagePropertyIri;
impl PartialEq<&str> for LanguagePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == LANGUAGE_PROPERTY_IRI_HTTP || *other == LANGUAGE_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<LanguagePropertyIri> for &str {
	fn eq(&self, other: &LanguagePropertyIri) -> bool {
		*self == LANGUAGE_PROPERTY_IRI_HTTP || *self == LANGUAGE_PROPERTY_IRI_HTTPS
	}
}
pub struct LanguagePropertyIriOrLabel;
impl PartialEq<&str> for LanguagePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == LanguagePropertyIri || *other == LANGUAGE_PROPERTY_LABEL
	}
}
impl PartialEq<LanguagePropertyIriOrLabel> for &str {
	fn eq(&self, other: &LanguagePropertyIriOrLabel) -> bool {
		*self == LanguagePropertyIri || *self == LANGUAGE_PROPERTY_LABEL
	}
}
