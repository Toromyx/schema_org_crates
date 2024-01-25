/// <https://schema.org/studyDesign>
pub const STUDY_DESIGN_PROPERTY_IRI_HTTP: &str = "http://schema.org/studyDesign";
/// <https://schema.org/studyDesign>
pub const STUDY_DESIGN_PROPERTY_IRI_HTTPS: &str = "https://schema.org/studyDesign";
/// <https://schema.org/studyDesign>
pub const STUDY_DESIGN_PROPERTY_LABEL: &str = "studyDesign";
pub struct StudyDesignPropertyIri;
impl PartialEq<&str> for StudyDesignPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == STUDY_DESIGN_PROPERTY_IRI_HTTP || *other == STUDY_DESIGN_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<StudyDesignPropertyIri> for &str {
	fn eq(&self, other: &StudyDesignPropertyIri) -> bool {
		*self == STUDY_DESIGN_PROPERTY_IRI_HTTP || *self == STUDY_DESIGN_PROPERTY_IRI_HTTPS
	}
}
pub struct StudyDesignPropertyIriOrLabel;
impl PartialEq<&str> for StudyDesignPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == StudyDesignPropertyIri || *other == STUDY_DESIGN_PROPERTY_LABEL
	}
}
impl PartialEq<StudyDesignPropertyIriOrLabel> for &str {
	fn eq(&self, other: &StudyDesignPropertyIriOrLabel) -> bool {
		*self == StudyDesignPropertyIri || *self == STUDY_DESIGN_PROPERTY_LABEL
	}
}
