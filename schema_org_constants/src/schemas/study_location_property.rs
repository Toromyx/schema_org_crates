/// <https://schema.org/studyLocation>
pub const STUDY_LOCATION_PROPERTY_IRI_HTTP: &str = "http://schema.org/studyLocation";
/// <https://schema.org/studyLocation>
pub const STUDY_LOCATION_PROPERTY_IRI_HTTPS: &str = "https://schema.org/studyLocation";
/// <https://schema.org/studyLocation>
pub const STUDY_LOCATION_PROPERTY_LABEL: &str = "studyLocation";
pub struct StudyLocationPropertyIri;
impl PartialEq<&str> for StudyLocationPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == STUDY_LOCATION_PROPERTY_IRI_HTTP || *other == STUDY_LOCATION_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<StudyLocationPropertyIri> for &str {
	fn eq(&self, other: &StudyLocationPropertyIri) -> bool {
		*self == STUDY_LOCATION_PROPERTY_IRI_HTTP || *self == STUDY_LOCATION_PROPERTY_IRI_HTTPS
	}
}
pub struct StudyLocationPropertyIriOrLabel;
impl PartialEq<&str> for StudyLocationPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == StudyLocationPropertyIri || *other == STUDY_LOCATION_PROPERTY_LABEL
	}
}
impl PartialEq<StudyLocationPropertyIriOrLabel> for &str {
	fn eq(&self, other: &StudyLocationPropertyIriOrLabel) -> bool {
		*self == StudyLocationPropertyIri || *self == STUDY_LOCATION_PROPERTY_LABEL
	}
}
