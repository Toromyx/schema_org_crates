/// <https://schema.org/studySubject>
pub const STUDY_SUBJECT_PROPERTY_IRI_HTTP: &str = "http://schema.org/studySubject";
/// <https://schema.org/studySubject>
pub const STUDY_SUBJECT_PROPERTY_IRI_HTTPS: &str = "https://schema.org/studySubject";
/// <https://schema.org/studySubject>
pub const STUDY_SUBJECT_PROPERTY_LABEL: &str = "studySubject";
pub struct StudySubjectPropertyIri;
impl PartialEq<&str> for StudySubjectPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == STUDY_SUBJECT_PROPERTY_IRI_HTTP || *other == STUDY_SUBJECT_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<StudySubjectPropertyIri> for &str {
	fn eq(&self, other: &StudySubjectPropertyIri) -> bool {
		*self == STUDY_SUBJECT_PROPERTY_IRI_HTTP || *self == STUDY_SUBJECT_PROPERTY_IRI_HTTPS
	}
}
pub struct StudySubjectPropertyIriOrLabel;
impl PartialEq<&str> for StudySubjectPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == StudySubjectPropertyIri || *other == STUDY_SUBJECT_PROPERTY_LABEL
	}
}
impl PartialEq<StudySubjectPropertyIriOrLabel> for &str {
	fn eq(&self, other: &StudySubjectPropertyIriOrLabel) -> bool {
		*self == StudySubjectPropertyIri || *self == STUDY_SUBJECT_PROPERTY_LABEL
	}
}
