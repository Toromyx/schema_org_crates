/// <https://schema.org/workTranslation>
pub const WORK_TRANSLATION_PROPERTY_IRI_HTTP: &str = "http://schema.org/workTranslation";
/// <https://schema.org/workTranslation>
pub const WORK_TRANSLATION_PROPERTY_IRI_HTTPS: &str = "https://schema.org/workTranslation";
/// <https://schema.org/workTranslation>
pub const WORK_TRANSLATION_PROPERTY_LABEL: &str = "workTranslation";
pub struct WorkTranslationPropertyIri;
impl PartialEq<&str> for WorkTranslationPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == WORK_TRANSLATION_PROPERTY_IRI_HTTP
			|| *other == WORK_TRANSLATION_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<WorkTranslationPropertyIri> for &str {
	fn eq(&self, other: &WorkTranslationPropertyIri) -> bool {
		*self == WORK_TRANSLATION_PROPERTY_IRI_HTTP || *self == WORK_TRANSLATION_PROPERTY_IRI_HTTPS
	}
}
pub struct WorkTranslationPropertyIriOrLabel;
impl PartialEq<&str> for WorkTranslationPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == WorkTranslationPropertyIri || *other == WORK_TRANSLATION_PROPERTY_LABEL
	}
}
impl PartialEq<WorkTranslationPropertyIriOrLabel> for &str {
	fn eq(&self, other: &WorkTranslationPropertyIriOrLabel) -> bool {
		*self == WorkTranslationPropertyIri || *self == WORK_TRANSLATION_PROPERTY_LABEL
	}
}
