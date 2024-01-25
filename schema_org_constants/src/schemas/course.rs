/// <https://schema.org/Course>
pub const COURSE_IRI_HTTP: &str = "http://schema.org/Course";
/// <https://schema.org/Course>
pub const COURSE_IRI_HTTPS: &str = "https://schema.org/Course";
/// <https://schema.org/Course>
pub const COURSE_LABEL: &str = "Course";
pub struct CourseIri;
impl PartialEq<&str> for CourseIri {
	fn eq(&self, other: &&str) -> bool {
		*other == COURSE_IRI_HTTP || *other == COURSE_IRI_HTTPS
	}
}
impl PartialEq<CourseIri> for &str {
	fn eq(&self, other: &CourseIri) -> bool {
		*self == COURSE_IRI_HTTP || *self == COURSE_IRI_HTTPS
	}
}
pub struct CourseIriOrLabel;
impl PartialEq<&str> for CourseIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == CourseIri || *other == COURSE_LABEL
	}
}
impl PartialEq<CourseIriOrLabel> for &str {
	fn eq(&self, other: &CourseIriOrLabel) -> bool {
		*self == CourseIri || *self == COURSE_LABEL
	}
}
