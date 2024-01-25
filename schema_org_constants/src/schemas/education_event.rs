/// <https://schema.org/EducationEvent>
pub const EDUCATION_EVENT_IRI_HTTP: &str = "http://schema.org/EducationEvent";
/// <https://schema.org/EducationEvent>
pub const EDUCATION_EVENT_IRI_HTTPS: &str = "https://schema.org/EducationEvent";
/// <https://schema.org/EducationEvent>
pub const EDUCATION_EVENT_LABEL: &str = "EducationEvent";
pub struct EducationEventIri;
impl PartialEq<&str> for EducationEventIri {
	fn eq(&self, other: &&str) -> bool {
		*other == EDUCATION_EVENT_IRI_HTTP || *other == EDUCATION_EVENT_IRI_HTTPS
	}
}
impl PartialEq<EducationEventIri> for &str {
	fn eq(&self, other: &EducationEventIri) -> bool {
		*self == EDUCATION_EVENT_IRI_HTTP || *self == EDUCATION_EVENT_IRI_HTTPS
	}
}
pub struct EducationEventIriOrLabel;
impl PartialEq<&str> for EducationEventIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == EducationEventIri || *other == EDUCATION_EVENT_LABEL
	}
}
impl PartialEq<EducationEventIriOrLabel> for &str {
	fn eq(&self, other: &EducationEventIriOrLabel) -> bool {
		*self == EducationEventIri || *self == EDUCATION_EVENT_LABEL
	}
}
