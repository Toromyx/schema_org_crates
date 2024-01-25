/// <https://schema.org/translationOfWork>
pub const TRANSLATION_OF_WORK_PROPERTY_IRI_HTTP: &str = "http://schema.org/translationOfWork";
/// <https://schema.org/translationOfWork>
pub const TRANSLATION_OF_WORK_PROPERTY_IRI_HTTPS: &str = "https://schema.org/translationOfWork";
/// <https://schema.org/translationOfWork>
pub const TRANSLATION_OF_WORK_PROPERTY_LABEL: &str = "translationOfWork";
pub struct TranslationOfWorkPropertyIri;
impl PartialEq<&str> for TranslationOfWorkPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == TRANSLATION_OF_WORK_PROPERTY_IRI_HTTP
			|| *other == TRANSLATION_OF_WORK_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<TranslationOfWorkPropertyIri> for &str {
	fn eq(&self, other: &TranslationOfWorkPropertyIri) -> bool {
		*self == TRANSLATION_OF_WORK_PROPERTY_IRI_HTTP
			|| *self == TRANSLATION_OF_WORK_PROPERTY_IRI_HTTPS
	}
}
pub struct TranslationOfWorkPropertyIriOrLabel;
impl PartialEq<&str> for TranslationOfWorkPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == TranslationOfWorkPropertyIri || *other == TRANSLATION_OF_WORK_PROPERTY_LABEL
	}
}
impl PartialEq<TranslationOfWorkPropertyIriOrLabel> for &str {
	fn eq(&self, other: &TranslationOfWorkPropertyIriOrLabel) -> bool {
		*self == TranslationOfWorkPropertyIri || *self == TRANSLATION_OF_WORK_PROPERTY_LABEL
	}
}
