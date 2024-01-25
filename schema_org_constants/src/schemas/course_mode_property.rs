/// <https://schema.org/courseMode>
pub const COURSE_MODE_PROPERTY_IRI_HTTP: &str = "http://schema.org/courseMode";
/// <https://schema.org/courseMode>
pub const COURSE_MODE_PROPERTY_IRI_HTTPS: &str = "https://schema.org/courseMode";
/// <https://schema.org/courseMode>
pub const COURSE_MODE_PROPERTY_LABEL: &str = "courseMode";
pub struct CourseModePropertyIri;
impl PartialEq<&str> for CourseModePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == COURSE_MODE_PROPERTY_IRI_HTTP || *other == COURSE_MODE_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<CourseModePropertyIri> for &str {
	fn eq(&self, other: &CourseModePropertyIri) -> bool {
		*self == COURSE_MODE_PROPERTY_IRI_HTTP || *self == COURSE_MODE_PROPERTY_IRI_HTTPS
	}
}
pub struct CourseModePropertyIriOrLabel;
impl PartialEq<&str> for CourseModePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == CourseModePropertyIri || *other == COURSE_MODE_PROPERTY_LABEL
	}
}
impl PartialEq<CourseModePropertyIriOrLabel> for &str {
	fn eq(&self, other: &CourseModePropertyIriOrLabel) -> bool {
		*self == CourseModePropertyIri || *self == COURSE_MODE_PROPERTY_LABEL
	}
}
