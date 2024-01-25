/// <https://schema.org/inLanguage>
pub const IN_LANGUAGE_PROPERTY_IRI_HTTP: &str = "http://schema.org/inLanguage";
/// <https://schema.org/inLanguage>
pub const IN_LANGUAGE_PROPERTY_IRI_HTTPS: &str = "https://schema.org/inLanguage";
/// <https://schema.org/inLanguage>
pub const IN_LANGUAGE_PROPERTY_LABEL: &str = "inLanguage";
pub struct InLanguagePropertyIri;
impl PartialEq<&str> for InLanguagePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == IN_LANGUAGE_PROPERTY_IRI_HTTP || *other == IN_LANGUAGE_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<InLanguagePropertyIri> for &str {
	fn eq(&self, other: &InLanguagePropertyIri) -> bool {
		*self == IN_LANGUAGE_PROPERTY_IRI_HTTP || *self == IN_LANGUAGE_PROPERTY_IRI_HTTPS
	}
}
pub struct InLanguagePropertyIriOrLabel;
impl PartialEq<&str> for InLanguagePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == InLanguagePropertyIri || *other == IN_LANGUAGE_PROPERTY_LABEL
	}
}
impl PartialEq<InLanguagePropertyIriOrLabel> for &str {
	fn eq(&self, other: &InLanguagePropertyIriOrLabel) -> bool {
		*self == InLanguagePropertyIri || *self == IN_LANGUAGE_PROPERTY_LABEL
	}
}
