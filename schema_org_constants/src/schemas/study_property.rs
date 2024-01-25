/// <https://schema.org/study>
pub const STUDY_PROPERTY_IRI_HTTP: &str = "http://schema.org/study";
/// <https://schema.org/study>
pub const STUDY_PROPERTY_IRI_HTTPS: &str = "https://schema.org/study";
/// <https://schema.org/study>
pub const STUDY_PROPERTY_LABEL: &str = "study";
pub struct StudyPropertyIri;
impl PartialEq<&str> for StudyPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == STUDY_PROPERTY_IRI_HTTP || *other == STUDY_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<StudyPropertyIri> for &str {
	fn eq(&self, other: &StudyPropertyIri) -> bool {
		*self == STUDY_PROPERTY_IRI_HTTP || *self == STUDY_PROPERTY_IRI_HTTPS
	}
}
pub struct StudyPropertyIriOrLabel;
impl PartialEq<&str> for StudyPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == StudyPropertyIri || *other == STUDY_PROPERTY_LABEL
	}
}
impl PartialEq<StudyPropertyIriOrLabel> for &str {
	fn eq(&self, other: &StudyPropertyIriOrLabel) -> bool {
		*self == StudyPropertyIri || *self == STUDY_PROPERTY_LABEL
	}
}
