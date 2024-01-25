/// <https://schema.org/knowsLanguage>
pub const KNOWS_LANGUAGE_PROPERTY_IRI_HTTP: &str = "http://schema.org/knowsLanguage";
/// <https://schema.org/knowsLanguage>
pub const KNOWS_LANGUAGE_PROPERTY_IRI_HTTPS: &str = "https://schema.org/knowsLanguage";
/// <https://schema.org/knowsLanguage>
pub const KNOWS_LANGUAGE_PROPERTY_LABEL: &str = "knowsLanguage";
pub struct KnowsLanguagePropertyIri;
impl PartialEq<&str> for KnowsLanguagePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == KNOWS_LANGUAGE_PROPERTY_IRI_HTTP || *other == KNOWS_LANGUAGE_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<KnowsLanguagePropertyIri> for &str {
	fn eq(&self, other: &KnowsLanguagePropertyIri) -> bool {
		*self == KNOWS_LANGUAGE_PROPERTY_IRI_HTTP || *self == KNOWS_LANGUAGE_PROPERTY_IRI_HTTPS
	}
}
pub struct KnowsLanguagePropertyIriOrLabel;
impl PartialEq<&str> for KnowsLanguagePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == KnowsLanguagePropertyIri || *other == KNOWS_LANGUAGE_PROPERTY_LABEL
	}
}
impl PartialEq<KnowsLanguagePropertyIriOrLabel> for &str {
	fn eq(&self, other: &KnowsLanguagePropertyIriOrLabel) -> bool {
		*self == KnowsLanguagePropertyIri || *self == KNOWS_LANGUAGE_PROPERTY_LABEL
	}
}
