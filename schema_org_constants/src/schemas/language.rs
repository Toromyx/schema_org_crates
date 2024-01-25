/// <https://schema.org/Language>
pub const LANGUAGE_IRI_HTTP: &str = "http://schema.org/Language";
/// <https://schema.org/Language>
pub const LANGUAGE_IRI_HTTPS: &str = "https://schema.org/Language";
/// <https://schema.org/Language>
pub const LANGUAGE_LABEL: &str = "Language";
pub struct LanguageIri;
impl PartialEq<&str> for LanguageIri {
	fn eq(&self, other: &&str) -> bool {
		*other == LANGUAGE_IRI_HTTP || *other == LANGUAGE_IRI_HTTPS
	}
}
impl PartialEq<LanguageIri> for &str {
	fn eq(&self, other: &LanguageIri) -> bool {
		*self == LANGUAGE_IRI_HTTP || *self == LANGUAGE_IRI_HTTPS
	}
}
pub struct LanguageIriOrLabel;
impl PartialEq<&str> for LanguageIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == LanguageIri || *other == LANGUAGE_LABEL
	}
}
impl PartialEq<LanguageIriOrLabel> for &str {
	fn eq(&self, other: &LanguageIriOrLabel) -> bool {
		*self == LanguageIri || *self == LANGUAGE_LABEL
	}
}
