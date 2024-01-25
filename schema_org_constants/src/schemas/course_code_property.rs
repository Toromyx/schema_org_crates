/// <https://schema.org/courseCode>
pub const COURSE_CODE_PROPERTY_IRI_HTTP: &str = "http://schema.org/courseCode";
/// <https://schema.org/courseCode>
pub const COURSE_CODE_PROPERTY_IRI_HTTPS: &str = "https://schema.org/courseCode";
/// <https://schema.org/courseCode>
pub const COURSE_CODE_PROPERTY_LABEL: &str = "courseCode";
pub struct CourseCodePropertyIri;
impl PartialEq<&str> for CourseCodePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == COURSE_CODE_PROPERTY_IRI_HTTP || *other == COURSE_CODE_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<CourseCodePropertyIri> for &str {
	fn eq(&self, other: &CourseCodePropertyIri) -> bool {
		*self == COURSE_CODE_PROPERTY_IRI_HTTP || *self == COURSE_CODE_PROPERTY_IRI_HTTPS
	}
}
pub struct CourseCodePropertyIriOrLabel;
impl PartialEq<&str> for CourseCodePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == CourseCodePropertyIri || *other == COURSE_CODE_PROPERTY_LABEL
	}
}
impl PartialEq<CourseCodePropertyIriOrLabel> for &str {
	fn eq(&self, other: &CourseCodePropertyIriOrLabel) -> bool {
		*self == CourseCodePropertyIri || *self == COURSE_CODE_PROPERTY_LABEL
	}
}
