/// <https://schema.org/availableLanguage>
pub const AVAILABLE_LANGUAGE_PROPERTY_IRI_HTTP: &str = "http://schema.org/availableLanguage";
/// <https://schema.org/availableLanguage>
pub const AVAILABLE_LANGUAGE_PROPERTY_IRI_HTTPS: &str = "https://schema.org/availableLanguage";
/// <https://schema.org/availableLanguage>
pub const AVAILABLE_LANGUAGE_PROPERTY_LABEL: &str = "availableLanguage";
pub struct AvailableLanguagePropertyIri;
impl PartialEq<&str> for AvailableLanguagePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == AVAILABLE_LANGUAGE_PROPERTY_IRI_HTTP
			|| *other == AVAILABLE_LANGUAGE_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<AvailableLanguagePropertyIri> for &str {
	fn eq(&self, other: &AvailableLanguagePropertyIri) -> bool {
		*self == AVAILABLE_LANGUAGE_PROPERTY_IRI_HTTP
			|| *self == AVAILABLE_LANGUAGE_PROPERTY_IRI_HTTPS
	}
}
pub struct AvailableLanguagePropertyIriOrLabel;
impl PartialEq<&str> for AvailableLanguagePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == AvailableLanguagePropertyIri || *other == AVAILABLE_LANGUAGE_PROPERTY_LABEL
	}
}
impl PartialEq<AvailableLanguagePropertyIriOrLabel> for &str {
	fn eq(&self, other: &AvailableLanguagePropertyIriOrLabel) -> bool {
		*self == AvailableLanguagePropertyIri || *self == AVAILABLE_LANGUAGE_PROPERTY_LABEL
	}
}
