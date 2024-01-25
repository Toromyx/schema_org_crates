/// <https://schema.org/guidelineSubject>
pub const GUIDELINE_SUBJECT_PROPERTY_IRI_HTTP: &str = "http://schema.org/guidelineSubject";
/// <https://schema.org/guidelineSubject>
pub const GUIDELINE_SUBJECT_PROPERTY_IRI_HTTPS: &str = "https://schema.org/guidelineSubject";
/// <https://schema.org/guidelineSubject>
pub const GUIDELINE_SUBJECT_PROPERTY_LABEL: &str = "guidelineSubject";
pub struct GuidelineSubjectPropertyIri;
impl PartialEq<&str> for GuidelineSubjectPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == GUIDELINE_SUBJECT_PROPERTY_IRI_HTTP
			|| *other == GUIDELINE_SUBJECT_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<GuidelineSubjectPropertyIri> for &str {
	fn eq(&self, other: &GuidelineSubjectPropertyIri) -> bool {
		*self == GUIDELINE_SUBJECT_PROPERTY_IRI_HTTP
			|| *self == GUIDELINE_SUBJECT_PROPERTY_IRI_HTTPS
	}
}
pub struct GuidelineSubjectPropertyIriOrLabel;
impl PartialEq<&str> for GuidelineSubjectPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == GuidelineSubjectPropertyIri || *other == GUIDELINE_SUBJECT_PROPERTY_LABEL
	}
}
impl PartialEq<GuidelineSubjectPropertyIriOrLabel> for &str {
	fn eq(&self, other: &GuidelineSubjectPropertyIriOrLabel) -> bool {
		*self == GuidelineSubjectPropertyIri || *self == GUIDELINE_SUBJECT_PROPERTY_LABEL
	}
}
