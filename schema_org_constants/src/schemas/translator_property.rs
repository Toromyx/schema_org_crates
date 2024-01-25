/// <https://schema.org/translator>
pub const TRANSLATOR_PROPERTY_IRI_HTTP: &str = "http://schema.org/translator";
/// <https://schema.org/translator>
pub const TRANSLATOR_PROPERTY_IRI_HTTPS: &str = "https://schema.org/translator";
/// <https://schema.org/translator>
pub const TRANSLATOR_PROPERTY_LABEL: &str = "translator";
pub struct TranslatorPropertyIri;
impl PartialEq<&str> for TranslatorPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == TRANSLATOR_PROPERTY_IRI_HTTP || *other == TRANSLATOR_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<TranslatorPropertyIri> for &str {
	fn eq(&self, other: &TranslatorPropertyIri) -> bool {
		*self == TRANSLATOR_PROPERTY_IRI_HTTP || *self == TRANSLATOR_PROPERTY_IRI_HTTPS
	}
}
pub struct TranslatorPropertyIriOrLabel;
impl PartialEq<&str> for TranslatorPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == TranslatorPropertyIri || *other == TRANSLATOR_PROPERTY_LABEL
	}
}
impl PartialEq<TranslatorPropertyIriOrLabel> for &str {
	fn eq(&self, other: &TranslatorPropertyIriOrLabel) -> bool {
		*self == TranslatorPropertyIri || *self == TRANSLATOR_PROPERTY_LABEL
	}
}
